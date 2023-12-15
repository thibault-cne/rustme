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

    fn circle(&self, selector: &str, len: f32, delay: f32) -> String {
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

        let (total, count) = user_info.problems_stats();
        css.push_str(&self.circle(
            "#total-solved-ring",
            std::f32::consts::PI * 80.0 * (count / total) as f32,
            0.7,
        ));

        style.push(css);
    }
}
