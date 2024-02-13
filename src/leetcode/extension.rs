use super::{Generator, Item, UserInfo};

pub trait Extension: std::fmt::Debug {
    fn extend(
        &self,
        generator: &Generator,
        user_info: &UserInfo,
        body: &mut Vec<Item>,
        style: &mut Vec<String>,
    );
}

#[derive(Debug)]
pub struct Animation;

impl Animation {
    const KEYFRAME: &'static str = "@keyframes fade_in{from{opacity:0}to{opacity:1}}";

    fn order(&self) -> Vec<&str> {
        vec![
            "#icon",
            "#username",
            "#ranking",
            "#total-solved-bg",
            "#total-solved-ring",
            "#total-solved-text",
            "#easy-solved-type",
            "#easy-solved-count",
            "#easy-solved-bg",
            "#easy-solved-progress",
            "#medium-solved-type",
            "#medium-solved-count",
            "#medium-solved-bg",
            "#medium-solved-progress",
            "#hard-solved-type",
            "#hard-solved-count",
            "#hard-solved-bg",
            "#hard-solved-progress",
        ]
    }

    fn circle(&self, selector: &str, len: f64, delay: f32) -> String {
        let animation = format!("@keyframes circle{{0%{{opacity:0;stroke-dasharray:0 1000}}50%{{opacity:1}}100%{{opacity:1;stroke-dasharray:{len} 10000}}}}");
        let style = format!("{selector}{{animation:circle 1.2s ease {delay}s 1 forwards}}");
        format!("{}{}", animation, style)
    }
}

impl Extension for Animation {
    fn extend(
        &self,
        _: &Generator,
        user_info: &UserInfo,
        _: &mut Vec<Item>,
        style: &mut Vec<String>,
    ) {
        let mut css = Animation::KEYFRAME.to_string();
        let speed = 1_f32;

        self.order().iter().enumerate().for_each(|(i, select)| {
            css.push_str(&format!(
                "{}{{opacity:0;animation:fade_in {}s ease {}s 1 forwards}}",
                select,
                0.3 / speed,
                0.1 * i as f32
            ))
        });

        let (solved, total) = user_info.problems_stats();
        css.push_str(&self.circle(
            "#total-solved-ring",
            std::f64::consts::PI * 80.0 * solved as f64 / total as f64,
            0.7,
        ));

        style.push(css);
    }
}

#[derive(Debug)]
pub struct Themes(Vec<super::theme::Theme>);

impl Extension for Themes {
    fn extend(
        &self,
        generator: &Generator,
        user_info: &UserInfo,
        body: &mut Vec<Item>,
        style: &mut Vec<String>,
    ) {
        self.0.iter().for_each(|t| t.extend(style));
    }
}

impl From<Vec<super::theme::Theme>> for Themes {
    fn from(themes: Vec<super::theme::Theme>) -> Self {
        Themes(themes)
    }
}

#[derive(Debug)]
pub struct Font {
    font: FontType,
}

#[derive(Debug)]
pub enum FontType {
    Baloo2,
}

impl std::fmt::Display for FontType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontType::Baloo2 => write!(f, "Baloo 2"),
        }
    }
}

impl FontType {
    fn format_url(&self) -> String {
        match self {
            FontType::Baloo2 => "baloo_2".to_string(),
        }
    }
}

#[derive(serde::Deserialize)]
struct JsonFont {
    name: String,
    base64: String,
}

impl Font {
    const BASE_URL: &'static str = "https://cdn.jsdelivr.net/gh/JacobLinCool/nano-font@json/";

    async fn fetch(&self) -> JsonFont {
        let url = format!("{}{}.json", Self::BASE_URL, self.font.format_url());
        let resp = reqwest::get(&url).await.unwrap();
        let json: JsonFont = resp.json().await.unwrap();
        json
    }
}

#[derive(Debug)]
pub struct Fonts(Vec<Font>);

impl Extension for Fonts {
    fn extend(&self, _: &Generator, _: &UserInfo, _: &mut Vec<Item>, style: &mut Vec<String>) {
        let fonts = self.0.iter().map(|f| f.fetch());
        let fonts = futures::executor::block_on(futures::future::join_all(fonts));
        fonts.iter().for_each(|f| {
            let font = format!(
                r##"@font-face{{font-family:"{}";src:url("{}") format("woff2")}}"##,
                f.name, f.base64
            );
            style.push(font);
        });
        let body = fonts
            .iter()
            .map(|f| match f.name.as_str() {
                "sans" | "serif" | "monospace" => f.name.clone(),
                _ => format!(r#""{}""#, f.name),
            })
            .collect::<Vec<String>>()
            .join(",");
        style.push(format!("*{{font-family:{}}}", body));
    }
}

impl From<Vec<Font>> for Fonts {
    fn from(fonts: Vec<Font>) -> Self {
        Fonts(fonts)
    }
}

pub const BALOO_2: Font = Font {
    font: FontType::Baloo2,
};
