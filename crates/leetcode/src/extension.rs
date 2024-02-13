use core::{item::Item, Extension};

use crate::font::Font;
pub use crate::theme::Theme;
use crate::Generator;

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

impl Extension<Generator> for Animation {
    fn extend(&self, generator: &Generator, _: &mut Vec<Item>, style: &mut Vec<String>) {
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

        let (solved, total) = generator.get_user_info().problems_stats();
        css.push_str(&self.circle(
            "#total-solved-ring",
            std::f64::consts::PI * 80.0 * solved as f64 / total as f64,
            0.7,
        ));

        style.push(css);
    }
}

#[derive(Debug)]
pub struct Themes(Vec<Theme>);

impl Extension<Generator> for Themes {
    fn extend(&self, generator: &Generator, body: &mut Vec<Item>, style: &mut Vec<String>) {
        self.0.iter().for_each(|t| t.extend(generator, body, style));
    }
}

impl From<Vec<Theme>> for Themes {
    fn from(themes: Vec<Theme>) -> Self {
        Themes(themes)
    }
}

#[derive(Debug)]
pub struct Fonts(Vec<Font>);

impl Extension<Generator> for Fonts {
    fn extend(&self, _: &Generator, _: &mut Vec<Item>, style: &mut Vec<String>) {
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
