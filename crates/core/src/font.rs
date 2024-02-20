#[derive(Debug, Clone, Copy)]
pub enum Font {
    Baloo2,
    Formula1,
}

impl Font {
    const BASE_URL: &'static str = "https://cdn.jsdelivr.net/gh/thibault-cne/rustme@json/";

    pub async fn fetch(&self) -> JsonFont {
        let url = format!("{}{}.json", Self::BASE_URL, self.filename());

        let resp = reqwest::get(url).await.unwrap();
        let bytes = resp.bytes().await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct JsonFont {
    pub name: String,
    pub base64: String,
}

macros::impl_fonts!(
    BALOO_2 => Baloo2, "baloo_2", "Baloo 2";
    FORMULA_1 => Formula1, "formula_1", "Formula 1";
);

mod macros {
    macro_rules! impl_fonts {
        ($($const:ident => $font:ident, $filename:literal, $font_family:literal);*;) => {
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

            impl From<&str> for Font {
                fn from(s: &str) -> Self {
                    match s {
                        $($filename | $font_family => Font::$font,)*
                        _ => Font::Baloo2
                    }
                }
            }

            impl From<String> for Font {
                fn from(s: String) -> Self {
                    s.as_str().into()
                }
            }

            $(
                pub const $const: Font = Font::$font;
            )*

            pub const ALL_FONTS: &[Font] = &[$($const),*];
        };
    }

    pub(super) use impl_fonts;
}
