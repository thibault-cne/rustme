#![allow(dead_code)]

use core::{
    font::Font,
    item::{Item, ItemBuilder},
    theme::Theme,
    Extension, Generator as GeneratorTrait,
};

pub mod extension;
pub mod font;
mod graphql;
mod item;
pub mod theme;

#[derive(Default)]
pub struct Generator {
    config: Config,
    verbose: bool,
    user_info: Option<UserInfo>,
}

impl GeneratorTrait for Generator {
    async fn generate(mut self) -> error::Result<String> {
        log! { self.verbose => "starting generation with config: {:?}", self.config };

        let user_id = graphql::Id::new(&self.config.username);
        log! { self.verbose => "awaiting user_info of: {:?}", self.config.username };
        let client = graphql::Client::new(user_id).set_verbose(self.verbose);
        let user_info = client.get().await.unwrap_or_default();
        self.user_info = Some(user_info);
        log! { self.verbose => "received user_info: {:?}", self.user_info };

        self.hydrate().await
    }
}

impl Generator {
    pub fn new(config: Config) -> Generator {
        Generator {
            config,
            verbose: false,
            user_info: None,
        }
    }

    async fn hydrate(mut self) -> error::Result<String> {
        log! {self.verbose => "starting hydration..."};
        let mut ext_style = Vec::new();
        let mut ext_body = Vec::new();

        log! {self.verbose => "starting extending extensions"};
        for ext in self.config.get_extensions() {
            ext.extend(&mut self, &mut ext_body, &mut ext_style).await?;
        }
        log! {self.verbose => "ending extending extensions"};

        log! {self.verbose => "starting building DOM"};

        let user_info = self.get_user_info();
        let mut root = item::root(self.config.width, self.config.height, user_info);
        let (solved, total) = user_info.problems_stats();

        root.push_child(item::icon());
        root.push_child(item::username(&user_info.username));
        root.push_child(item::ranking(user_info.profile.ranking));
        root.push_child(item::total_solved(solved, total));
        root.push_child(item::solved(&user_info.submissions));

        let mut builder = ItemBuilder::default();

        let mut style = vec![
            "@namespace svg url(http://www.w3.org/2000/svg);".to_string(),
            builder.css(&mut root),
        ];
        style.extend_from_slice(&ext_style);
        style.push("svg{opacity:1}".to_string());

        root.push_child(Item::style(style.join("")));

        log! {self.verbose => "ending building DOM"};
        log! {self.verbose => "ending hydration..."};

        Ok(builder.stringify(&mut root))
    }

    pub fn verbose(&mut self) {
        self.verbose = true;
    }

    pub fn non_verbose(&mut self) {
        self.verbose = false;
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    fn get_user_info(&self) -> &UserInfo {
        self.user_info.as_ref().unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    username: String,
    width: u32,
    height: u32,
    themes: [Option<Theme>; 2],
    font: Font,
    animation: bool,
    extensions: Vec<extension::Extension>,
}

impl Config {
    pub fn new(username: &str) -> Config {
        Config {
            username: username.to_string(),
            ..Default::default()
        }
    }

    pub fn set_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn set_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn set_username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }

    pub fn as_username(&self) -> bool {
        !self.username.is_empty()
    }

    pub fn set_font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    pub fn set_animation(mut self, animation: bool) -> Self {
        self.animation = animation;
        self
    }

    pub fn set_dark_theme(mut self, mut theme: Theme) -> Self {
        theme.set_dark();
        self.themes[1] = Some(theme);
        self
    }

    pub fn set_light_theme(mut self, mut theme: Theme) -> Self {
        theme.set_light();
        self.themes[0] = Some(theme);
        self
    }

    pub fn set_single_theme(mut self, theme: Theme) -> Self {
        self.themes = [Some(theme), None];
        self
    }

    pub fn add_extension(self, ext: extension::Extension) -> Self {
        let mut config = self;
        config.extensions.push(ext);
        config
    }

    fn get_extensions(&self) -> Vec<extension::Extension> {
        let mut extensions = self.extensions.clone();
        extensions.extend(
            self.themes
                .clone()
                .into_iter()
                .flatten()
                .map(|theme| theme.into()),
        );
        extensions.push(self.font.into());
        if self.animation {
            extensions.push(extension::Extension::Animation);
        }

        extensions
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 500,
            height: 200,
            animation: true,
            username: String::from("thibaultcne"),
            themes: [Some(core::theme::LIGHT), Some(core::theme::DARK)],
            font: font::BALOO_2,
            extensions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct UserInfo {
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
            username: "thibault-cne".to_string(),
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

#[derive(Debug, Clone)]
struct Profile {
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

#[derive(Debug, Clone)]
struct Problem {
    difficulty: Difficulty,
    count: u32,
    total: u32,
    submissions: u32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Difficulty {
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
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "All" => Ok(Self::All),
            "Easy" => Ok(Self::Easy),
            "Medium" => Ok(Self::Medium),
            "Hard" => Ok(Self::Hard),
            _ => Err(error::Error::new_invalid_difficulty_kind()),
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

mod macros {
    #[macro_export]
    macro_rules! log {
        { $verbose:expr => $($tt:tt)* } => {
            #[cfg(feature = "worker")]
            if $verbose {
                worker::console_log!($($tt)*);
            }
            #[cfg(not(feature = "worker"))]
            if $verbose {
                println!($($tt)*);
            }
        };
    }
}
