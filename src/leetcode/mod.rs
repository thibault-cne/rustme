mod extension;
mod graphql;
mod item;

use extension::Extension;
use item::Item;

pub use graphql::{Client, Id};

#[derive(Debug)]
pub struct Generator {
    config: Config,
    verbose: bool,
}

#[derive(Debug)]
pub struct Config {
    username: String,
    width: u32,
    height: u32,
    css: Vec<String>,
    extensions: Vec<Box<dyn Extension>>,
}

#[derive(Debug)]
pub struct UserInfo {
    username: String,
    submissions: Vec<Submission>,
    streak: u32,
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
