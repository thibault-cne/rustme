use crate::leetcode;

mod extension;
mod graphql;
mod item;

use extension::Extension;
use item::{Item, ItemBuilder};

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
        let user_info = leetcode::Client::default().get(user_id).await;

        self.hydrate(user_info.unwrap())
    }

    fn hydrate(self, user_info: UserInfo) -> String {
        let mut root = Item::root(&self.config, &user_info);
        root.push_child(Item::icon());
        root.push_child(Item::username(&user_info.username));
        root.push_child(Item::ranking(user_info.profile.ranking));

        let mut builder = ItemBuilder::default();

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
    submissions: Vec<Submission>,
    streak: u32,
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

#[derive(Debug)]
pub struct Submission {
    difficulty: Difficulty,
    count: u32,
    submissions: u32,
}

#[derive(Debug)]
pub enum Difficulty {
    All,
    Easy,
    Medium,
    Hard,
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
