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
pub(crate) struct JsonFont {
    pub name: String,
    pub base64: String,
}

impl Font {
    const BASE_URL: &'static str = "https://cdn.jsdelivr.net/gh/JacobLinCool/nano-font@json/";

    pub(crate) async fn fetch(&self) -> JsonFont {
        let url = format!("{}{}.json", Self::BASE_URL, self.font.format_url());
        let resp = reqwest::get(&url).await.unwrap();
        let json: JsonFont = resp.json().await.unwrap();
        json
    }
}

pub const BALOO_2: Font = Font {
    font: FontType::Baloo2,
};
