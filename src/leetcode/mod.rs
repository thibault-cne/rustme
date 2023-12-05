use serde::Deserialize;

mod graphql;

pub use graphql::{Client, Id};

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
