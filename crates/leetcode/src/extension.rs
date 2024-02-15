use core::{font::Font, item::Item, Extension as ExtensionTrait};

pub use crate::theme::Theme;
use crate::Generator;

#[derive(Clone)]
pub enum Extension {
    Animation,
    Theme(Theme),
    Themes(Vec<Theme>),
    Fonts(Vec<Font>),
}

impl ExtensionTrait<Generator> for Extension {
    fn extend(&self, generator: &mut Generator, items: &mut Vec<Item>, style: &mut Vec<String>) {
        match self {
            Extension::Animation => {
                animation::extend(generator, items, style);
            }
            Extension::Theme(theme) => {
                theme.extend(generator, items, style);
            }
            Extension::Themes(themes) => themes::extend(themes, generator, items, style),
            Extension::Fonts(fonts) => {
                fonts::extend(fonts, generator, items, style);
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

    pub fn extend(
        themes: &[Theme],
        generator: &mut Generator,
        body: &mut Vec<Item>,
        style: &mut Vec<String>,
    ) {
        themes.iter().for_each(|t| t.extend(generator, body, style));
    }
}

mod fonts {
    use super::Font;
    use crate::Generator;
    use core::item::Item;

    pub fn extend(fonts: &[Font], _: &mut Generator, _: &mut Vec<Item>, style: &mut Vec<String>) {
        let fonts = fonts.iter().map(|f| f.fetch());
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

impl From<Vec<Theme>> for Extension {
    fn from(themes: Vec<Theme>) -> Self {
        Extension::Themes(themes)
    }
}

impl From<Vec<Font>> for Extension {
    fn from(fonts: Vec<Font>) -> Self {
        Extension::Fonts(fonts)
    }
}

impl From<&[Font]> for Extension {
    fn from(fonts: &[Font]) -> Self {
        Extension::Fonts(fonts.to_vec())
    }
}
