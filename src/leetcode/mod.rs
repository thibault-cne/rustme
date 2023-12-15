use crate::leetcode;

mod extension;
mod graphql;
mod item;

use extension::Extension;
use item::{Item, ItemBuilder};

pub use extension::Animation;
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
        let mut ext_style = Vec::new();
        let mut ext_body = Vec::new();

        self.config.extensions.iter().for_each(|ext| {
            ext.extend(&self, &user_info, &mut ext_body, &mut ext_style);
        });

        let mut root = Item::root(&self.config, &user_info);

        root.push_child(Item::icon());
        root.push_child(Item::username(&user_info.username));
        root.push_child(Item::ranking(user_info.profile.ranking));

        let mut builder = ItemBuilder::default();

        let mut style = vec![
            "@namespace svg url(http://www.w3.org/2000/svg);".to_string(),
            builder.css(&mut root),
        ];
        style.extend_from_slice(&ext_style);
        style.push("svg{opacity:1}".to_string());

        root.push_child(Item::style(style.join("\n")));

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
            .fold((0, 0), |acc, s| (acc.0 + s.total, acc.1 + s.count))
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

#[derive(Debug)]
pub struct Problem {
    difficulty: Difficulty,
    count: u32,
    total: u32,
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
