use serde::Serialize;

use super::UserInfo;

const QUERY: &str = r#"
query UserInfo($id: String!) {
    problems: allQuestionsCount {
        difficulty
        count
    }
    matchedUser(username: $id) {
        username
        profile {
            realname: realName
            about: aboutMe
            avatar: userAvatar
            skills: skillTags
            country: countryName
            ranking
        }
        submitStats: submitStatsGlobal {
            acSubmissionNum {
                difficulty
                count
                submissions
            }
        }
        userCalendar {
            streak
        }
    }
}"#;

#[derive(Debug)]
pub struct Client<'a> {
    client: reqwest::Client,
    csrf: Option<String>,
    leetcode_session: String,
    user_id: Id<'a>,
}

impl<'a> Client<'a> {
    pub fn new(user_id: Id<'a>) -> Client<'a> {
        Client {
            client: reqwest::Client::new(),
            csrf: None,
            leetcode_session: String::new(),
            user_id,
        }
    }

    async fn init(&mut self) -> Result<(), ()> {
        crate::log! {"starting initialized"};

        let resp = self
            .client
            .get("https://leetcode.com")
            .header("User-Agent", "Mozilla/5.0 Rustme API")
            .send()
            .await
            .map_err(|_err| ())?;

        crate::log! {"handshake ok"};

        if resp.headers().contains_key("set-cookie") {
            let cookies = parse_cookie(
                resp.headers()
                    .get("set-cookie")
                    .unwrap()
                    .to_str()
                    .map_err(|_err| ())?,
            )?;

            self.csrf = Some(cookies.0.to_string());
        }

        crate::log! {"ending initialized"};
        Ok(())
    }

    pub async fn get(mut self) -> Result<UserInfo, ()> {
        if self.csrf.is_none() {
            self.init().await?;
        }

        let body = RequestBody {
            query: QUERY,
            variables: self.user_id,
        };

        let req = self
            .client
            .post("https://leetcode.com/graphql")
            .body(serde_json::to_vec(&body).unwrap())
            .header("Content-Type", "application/json")
            .header("User-Agent", "Mozilla/5.0 Rustme API")
            .header("x-csrftoken", self.csrf.as_ref().unwrap())
            .header("Referer", "https://leetcode.com")
            .header("Origin", "https://leetcode.com")
            .header(
                "Cookie",
                &format!(
                    "csrftoken={}; LEETCODE_SESSION={}",
                    self.csrf.as_ref().unwrap(),
                    self.leetcode_session
                ),
            )
            .build()
            .map_err(|_err| ())?;

        let res = self.client.execute(req).await.map_err(|_err| ())?;
        let bytes = res.bytes().await.map_err(|_err| ())?;

        match serde_json::from_slice::<GraphQLResponse>(&bytes) {
            Ok(resp) => resp.try_into(),
            Err(_) => Err(()),
        }
    }
}

fn parse_cookie(header: &str) -> Result<(&str, Option<&str>), ()> {
    let mut parts = header.split(';');

    let cookie = parts.next().ok_or(())?.split('=').last().ok_or(())?;

    Ok((cookie, None))
}

#[derive(serde::Serialize, Debug)]
pub struct Id<'a> {
    id: &'a str,
}

impl<'a> Id<'a> {
    pub fn new(id: &'a str) -> Id {
        Id { id }
    }
}

#[derive(serde::Serialize)]
pub struct RequestBody<'a, T: Serialize> {
    query: &'a str,
    variables: T,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GraphQLResponse {
    data: Data,
}

impl TryInto<UserInfo> for GraphQLResponse {
    type Error = ();

    fn try_into(self) -> Result<UserInfo, Self::Error> {
        self.data.try_into()
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Data {
    matched_user: MatchedUser,
    problems: Vec<ProblemData>,
}

impl TryInto<UserInfo> for Data {
    type Error = ();

    fn try_into(self) -> Result<UserInfo, Self::Error> {
        Ok(UserInfo {
            username: self.matched_user.username,
            profile: self.matched_user.profile.into(),
            streak: self.matched_user.user_calendar.streak,
            submissions: self
                .matched_user
                .submit_stats
                .ac_submission_num
                .into_iter()
                .map(|s| {
                    s.try_into_problem(
                        self.problems
                            .iter()
                            .find(|data| data.difficulty == s.difficulty)
                            .unwrap(),
                    )
                })
                .collect::<Result<Vec<super::Problem>, ()>>()?,
        })
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MatchedUser {
    username: String,
    profile: Profile,
    submit_stats: SubmitStats,
    user_calendar: UserCalendar,
}

#[derive(serde::Deserialize, Debug)]
struct Profile {
    realname: String,
    about: String,
    avatar: String,
    skills: Vec<String>,
    country: Option<String>,
    ranking: u32,
}

impl From<Profile> for super::Profile {
    fn from(value: Profile) -> super::Profile {
        super::Profile {
            realname: value.realname,
            about: value.about,
            avatar: value.avatar,
            skills: value.skills,
            country: value.country,
            ranking: value.ranking,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SubmitStats {
    ac_submission_num: Vec<Submission>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Submission {
    difficulty: String,
    count: u32,
    submissions: u32,
}

impl Submission {
    fn try_into_problem(&self, problem_data: &ProblemData) -> Result<super::Problem, ()> {
        Ok(super::Problem {
            difficulty: super::Difficulty::try_from(self.difficulty.as_str())?,
            count: self.count,
            total: problem_data.count,
            submissions: self.submissions,
        })
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UserCalendar {
    streak: u32,
}

#[derive(serde::Deserialize, Debug)]
struct ProblemData {
    difficulty: String,
    count: u32,
}
