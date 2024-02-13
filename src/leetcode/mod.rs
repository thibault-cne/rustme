use crate::leetcode;

mod extension;
mod graphql;
mod item;
mod macros;
pub mod theme;

use extension::Extension;
use item::{Item, ItemBuilder};

pub use extension::{Animation, Fonts, Themes, BALOO_2};
pub use graphql::{Client, Id};

#[derive(Debug)]
pub struct Generator {
    config: Config,
    verbose: bool,
}

impl Generator {
    pub fn new(config: Config) -> Generator {
        Generator {
            config,
            verbose: false,
        }
    }

    pub async fn generate(self) -> String {
        let user_id = leetcode::Id::new(&self.config.username);
        let user_info = leetcode::Client::default()
            .get(user_id)
            .await
            .unwrap_or_default();

        self.hydrate(user_info)
    }

    fn hydrate(self, user_info: UserInfo) -> String {
        let mut ext_style = Vec::new();
        let mut ext_body = Vec::new();

        self.config.extensions.iter().for_each(|ext| {
            ext.extend(&self, &user_info, &mut ext_body, &mut ext_style);
        });

        let mut root = Item::root(&self.config, &user_info);
        let (solved, total) = user_info.problems_stats();

        root.push_child(Item::icon());
        root.push_child(Item::username(&user_info.username));
        root.push_child(Item::ranking(user_info.profile.ranking));
        root.push_child(Item::total_solved(solved, total));
        root.push_child(Item::solved(user_info.submissions));

        let mut builder = ItemBuilder::default();

        let mut style = vec![
            "@namespace svg url(http://www.w3.org/2000/svg);".to_string(),
            builder.css(&mut root),
        ];
        style.extend_from_slice(&ext_style);
        style.push("svg{opacity:1}".to_string());

        root.push_child(Item::style(style.join("")));

        builder.stringify(&mut root)
    }
}

#[derive(Debug)]
pub struct Config {
    username: String,
    width: u32,
    height: u32,
    css: Vec<String>,
    extensions: Vec<Box<dyn Extension>>,
}

impl Config {
    pub fn new(username: &str) -> Config {
        Config {
            username: username.to_string(),
            ..Default::default()
        }
    }

    pub fn add_extension(&mut self, ext: Box<dyn Extension>) {
        self.extensions.push(ext);
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 500,
            height: 200,
            username: String::from("thibaultcne"),
            css: Vec::new(),
            extensions: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct UserInfo {
    username: String,
    profile: Profile,
    submissions: Vec<Problem>,
    streak: u32,
}

impl UserInfo {
    fn problems_stats(&self) -> (u32, u32) {
        self.submissions
            .iter()
            .find(|p| p.difficulty == Difficulty::All)
            .map(|p| (p.count, p.total))
            .unwrap()
    }
}

impl Default for UserInfo {
    fn default() -> Self {
        UserInfo {
            username: "thibaultcne".to_string(),
            profile: Profile::default(),
            submissions: vec![Problem {
                difficulty: Difficulty::Easy,
                count: 10,
                total: 1000,
                submissions: 100,
            }],
            streak: 50,
        }
    }
}

#[derive(Debug)]
pub struct Profile {
    realname: String,
    about: String,
    avatar: String,
    skills: Vec<String>,
    country: Option<String>,
    ranking: u32,
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            realname: "Thibault Cheneviere".to_string(),
            about: String::new(),
            avatar: String::new(),
            skills: Vec::new(),
            country: Some("France".to_string()),
            ranking: 810_207,
        }
    }
}

#[derive(Debug)]
pub struct Problem {
    difficulty: Difficulty,
    count: u32,
    total: u32,
    submissions: u32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Difficulty {
    All,
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn capitalize(&self) -> String {
        match self {
            Self::All => "All".to_string(),
            Self::Easy => "Easy".to_string(),
            Self::Medium => "Medium".to_string(),
            Self::Hard => "Hard".to_string(),
        }
    }
}

impl TryFrom<&str> for Difficulty {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "All" => Ok(Self::All),
            "Easy" => Ok(Self::Easy),
            "Medium" => Ok(Self::Medium),
            "Hard" => Ok(Self::Hard),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::All => "all",
            Self::Easy => "easy",
            Self::Medium => "medium",
            Self::Hard => "hard",
        };
        write!(f, "{}", value)
    }
}
