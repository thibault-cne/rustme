use worker::*;

use core::font::Font;
use core::theme::Theme;
use core::Generator;

use leetcode::extension::Extension;
use leetcode::Config;

pub enum QueryParams {
    Username(String),
    Width(u32),
    Height(u32),
    Font(Font),
    Themes(Vec<Theme>),
    Extension(String),
}

pub async fn leetcode_handler(req: Request, _: RouteContext<()>) -> Result<Response> {
    let config = match crate::leetcode::config_from_url(&req.url().unwrap()) {
        Some(config) => config,
        None => return Response::error("Invalid query parameters", 400),
    };

    let mut generator = leetcode::Generator::new(config);
    generator.verbose();
    let html = generator.generate().await;

    Response::from_html(html)
}

fn parse_query(query: &Url) -> Vec<QueryParams> {
    query
        .query_pairs()
        .map(|(key, value)| {
            let key = key.to_ascii_lowercase();

            match key.as_str() {
                "username" => QueryParams::Username(value.to_string()),
                "width" => QueryParams::Width(value.parse().unwrap()),
                "height" => QueryParams::Height(value.parse().unwrap()),
                "font" => QueryParams::Font(value.to_string().into()),
                "theme" => {
                    let themes = value.split(',').map(Theme::from).take(2).collect();
                    QueryParams::Themes(themes)
                }
                "ext" => QueryParams::Extension(value.to_string()),
                _ => panic!("Invalid query parameter"),
            }
        })
        .collect()
}

fn config_from_url(query: &Url) -> Option<Config> {
    let params = parse_query(query);

    let config = params
        .into_iter()
        .fold(Config::default(), |config, param| match param {
            QueryParams::Username(username) => config.set_username(&username),
            QueryParams::Width(width) => config.set_width(width),
            QueryParams::Height(height) => config.set_height(height),
            QueryParams::Font(font) => config.set_font(font),
            QueryParams::Themes(themes) if themes.len() == 1 => {
                config.add_extension(themes.into_iter().next().unwrap().into())
            }
            QueryParams::Themes(themes) => {
                let mut themes = themes.into_iter();
                config
                    .set_light_theme(themes.next().unwrap())
                    .set_dark_theme(themes.next().unwrap())
            }
            QueryParams::Extension(ext) => {
                if let Some(ext) = extension(&ext) {
                    config.add_extension(ext)
                } else {
                    config
                }
            }
        });

    if config.as_username() {
        Some(config)
    } else {
        None
    }
}

fn extension(ext: &str) -> Option<Extension> {
    match ext {
        "animation" => Some(Extension::Animation),
        _ => None,
    }
}