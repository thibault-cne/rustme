#[derive(Debug, Clone, Copy)]
pub enum Font {
    Baloo2,
    Formula1,
}

impl Font {
    const BASE_URL: &'static str = "https://cdn.jsdelivr.net/gh/thibault-cne/rustme@json/";

    pub async fn fetch(&self) -> JsonFont {
        let url = format!("{}{}.json", Self::BASE_URL, self.filename());
        let resp = reqwest::get(&url).await.unwrap();
        let json: JsonFont = resp.json().await.unwrap();
        json
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct JsonFont {
    pub name: String,
    pub base64: String,
}

macro_rules! fonts {
    ($($font:ident => $filename:literal, $font_family:literal),*) => {
        impl Font {
            pub fn filename(&self) -> &'static str {
                match self {
                    $(Font::$font => $filename),*
                }
            }

            pub fn font_family(&self) -> &'static str {
                match self {
                    $(Font::$font => $font_family),*
                }
            }
        }
    };
}

fonts!(
    Baloo2 => "baloo_2", "Baloo 2",
    Formula1 => "formula_1", "Formula 1"
);
