use base64::Engine;

#[tokio::main]
async fn main() {
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
        let json_font = if font.remote_url.is_some() {
            font.fetch().await.unwrap()
        } else {
            font.convert().unwrap()
        };

        let json = serde_json::to_string(&json_font).unwrap();

        std::fs::write(
            format!("json/{}.json", font.name.replace(' ', "_").to_lowercase()),
            json,
        )
        .unwrap();
    }
}

struct Font {
    name: &'static str,
    filename: &'static str,
    remote_url: Option<&'static str>,
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

        Ok(JsonFont {
            name: self.name.to_string(),
            base64: base64::engine::general_purpose::STANDARD.encode(woff2),
        })
    }

    async fn fetch(&self) -> Result<JsonFont> {
        let url = format!("{}{}.json", self.remote_url.unwrap(), self.name);
        let resp = reqwest::get(&url).await?;
        resp.json().await.map_err(Error::Reqwest)
    }
}

const FONTS: &[Font] = &[Font {
    name: "Formula 1",
    filename: "Formula1-Regular-1.ttf",
    remote_url: None,
}];

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Error {
    Woff2,
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    Base64(base64::DecodeError),
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
