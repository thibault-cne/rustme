use core::{font::Font, item::Item, theme::Theme, Extension as ExtensionTrait};

use crate::Generator;

#[derive(Clone, Debug)]
pub enum Extension {
    Animation,
    Theme(Theme),
    Themes(Vec<Theme>),
    Font(Font),
}

impl ExtensionTrait<Generator> for Extension {
    async fn extend(
        &self,
        generator: &mut Generator,
        items: &mut Vec<Item>,
        style: &mut Vec<String>,
    ) {
        match self {
            Extension::Animation => {
                animation::extend(generator, items, style);
            }
            Extension::Theme(theme) => {
                theme.extend(generator, items, style).await;
            }
            Extension::Themes(themes) => themes::extend(themes, generator, items, style).await,
            Extension::Font(font) => {
                font::extend(font, generator, items, style).await;
            }
        }
    }
}

mod animation {
    use crate::Generator;
    use core::item::Item;

    const KEYFRAME: &str = "@keyframes fade_in{from{opacity:0}to{opacity:1}}";

    fn order() -> Vec<&'static str> {
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

    fn circle(selector: &str, len: f64, delay: f32) -> String {
        let animation = format!("@keyframes circle{{0%{{opacity:0;stroke-dasharray:0 1000}}50%{{opacity:1}}100%{{opacity:1;stroke-dasharray:{len} 10000}}}}");
        let style = format!("{selector}{{animation:circle 1.2s ease {delay}s 1 forwards}}");
        format!("{}{}", animation, style)
    }

    pub fn extend(generator: &mut Generator, _: &mut Vec<Item>, style: &mut Vec<String>) {
        let mut css = KEYFRAME.to_string();
        let speed = 1_f32;

        order().iter().enumerate().for_each(|(i, select)| {
            css.push_str(&format!(
                "{}{{opacity:0;animation:fade_in {}s ease {}s 1 forwards}}",
                select,
                0.3 / speed,
                0.1 * i as f32
            ))
        });

        let (solved, total) = generator.get_user_info().problems_stats();
        css.push_str(&circle(
            "#total-solved-ring",
            std::f64::consts::PI * 80.0 * solved as f64 / total as f64,
            0.7,
        ));

        style.push(css);
    }
}

mod themes {
    use super::Theme;
    use crate::Generator;
    use core::{item::Item, Extension};

    pub async fn extend(
        themes: &[Theme],
        generator: &mut Generator,
        body: &mut Vec<Item>,
        style: &mut Vec<String>,
    ) {
        for theme in themes {
            theme.extend(generator, body, style).await;
        }
    }
}

mod font {
    use super::Font;
    use crate::Generator;
    use core::item::Item;

    pub async fn extend(
        font: &Font,
        _: &mut Generator,
        _: &mut Vec<Item>,
        style: &mut Vec<String>,
    ) {
        let font = font.fetch().await;
        style.push(format!(
            r##"@font-face{{font-family:"{}";src:url("{}") format("woff2")}}"##,
            font.name, font.base64
        ));
        let font_family = match font.name.as_str() {
            "sans" | "serif" | "monospace" => font.name.clone(),
            _ => format!(r#""{}""#, font.name),
        };
        style.push(format!("*{{font-family:{}}}", font_family));
    }
}

impl From<Theme> for Extension {
    fn from(theme: Theme) -> Self {
        Extension::Theme(theme)
    }
}

impl From<Vec<Theme>> for Extension {
    fn from(themes: Vec<Theme>) -> Self {
        Extension::Themes(themes)
    }
}

impl From<Font> for Extension {
    fn from(font: Font) -> Self {
        Extension::Font(font)
    }
}

impl From<&Font> for Extension {
    fn from(font: &Font) -> Self {
        Extension::Font(*font)
    }
}
