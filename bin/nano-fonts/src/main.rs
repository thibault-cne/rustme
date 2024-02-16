use std::collections::HashMap;

use base64::Engine;

#[tokio::main]
async fn main() {
    // Load env variables from `.env` file
    dotenvy::dotenv().unwrap();

    // Check if `json/` directory exists
    if !std::path::Path::new("json").exists() {
        std::fs::create_dir("json").unwrap();
    } else {
        // Remove all files in `json/` directory
        std::fs::remove_dir_all("json").unwrap();
        std::fs::create_dir("json").unwrap();
    }

    // Convert all fonts to WOFF2 and save them to `json/` directory
    for font in FONTS {
        let json_font = font.convert().unwrap();

        let json = serde_json::to_string(&json_font).unwrap();

        std::fs::write(
            format!("json/{}.json", font.name.replace(' ', "_").to_lowercase()),
            json,
        )
        .unwrap();
    }

    // Download all Google Fonts and save them to `json/` directory
    for font in GoogleApiResponse::fetch()
        .await
        .unwrap()
        .items
        .iter()
        .take(2)
    {
        if let Ok(json_font) = font.try_into_json().await {
            let json = serde_json::to_string(&json_font).unwrap();

            std::fs::write(
                format!("json/{}.json", font.family.replace(' ', "_").to_lowercase()),
                json,
            )
            .unwrap();
        }
    }
}

#[derive(serde::Deserialize)]
struct GoogleApiResponse {
    items: Vec<GoogleFont>,
}

impl GoogleApiResponse {
    const BASE_URL: &'static str = "https://www.googleapis.com/webfonts/v1/webfonts?sort=alpha";

    async fn fetch() -> Result<Self> {
        let api_key = std::env::var("GOOGLE_API_KEY").unwrap();
        let url = format!("{}&key={}", Self::BASE_URL, api_key);
        let response = reqwest::get(url).await?.json().await?;
        Ok(response)
    }
}

#[derive(serde::Deserialize)]
struct GoogleFont {
    family: String,
    files: HashMap<String, String>,
}

impl GoogleFont {
    async fn try_into_json(&self) -> Result<JsonFont> {
        if let Some(regular) = self.files.get("regular") {
            let ttf = reqwest::get(regular).await?.bytes().await?;
            let woff2 =
                woff2::convert_ttf_to_woff2(&ttf, &[], 11, true).map_err(|_| Error::Woff2)?;
            let base64 = base64::engine::general_purpose::STANDARD.encode(woff2);

            Ok(JsonFont {
                name: self.family.clone(),
                base64: format!("data:font/woff2;charset=utf-8;base64,{}", base64),
            })
        } else {
            Err(Error::RegularFontNotFound)
        }
    }
}

struct Font {
    name: &'static str,
    filename: &'static str,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct JsonFont {
    pub name: String,
    pub base64: String,
}

impl Font {
    fn convert(&self) -> Result<JsonFont> {
        let ttf = std::fs::read(format!("font/{}", self.filename))?;
        let woff2 = woff2::convert_ttf_to_woff2(&ttf, &[], 11, true).map_err(|_| Error::Woff2)?;
        let base64 = base64::engine::general_purpose::STANDARD.encode(woff2);

        Ok(JsonFont {
            name: self.name.to_string(),
            base64: format!("data:font/woff2;charset=utf-8;base64,{}", base64),
        })
    }
}

const FONTS: &[Font] = &[Font {
    name: "Formula 1",
    filename: "Formula1-Regular-1.ttf",
}];

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Error {
    Woff2,
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    Base64(base64::DecodeError),
    RegularFontNotFound,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error::Base64(e)
    }
}
