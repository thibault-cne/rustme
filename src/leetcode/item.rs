use std::collections::HashMap;
use std::fmt::Display;

use super::{Config, Difficulty, Problem, UserInfo};
use crate::{attribute, style};

pub struct ItemBuilder {
    counter: u32,
}

impl ItemBuilder {
    pub fn stringify(&mut self, item: &mut Item) -> String {
        if !item.attr.contains_key("id") {
            item.attr.insert(
                "id".to_string(),
                Attribute::from(format!("_{}", self.counter)),
            );
            self.counter += 1;
        }

        let attr = item
            .attr
            .iter()
            .map(|(k, v)| format!(r#"{}="{}""#, k, v))
            .collect::<Vec<String>>()
            .join(" ");

        let children = item
            .children
            .as_mut()
            .map(|children| {
                children
                    .iter_mut()
                    .map(|child| self.stringify(child))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .unwrap_or_default();

        match item.single {
            Some(true) => format!("<{} {}/>", item.item_type, attr),
            _ => format!(
                "<{} {}>{}{}</{}>",
                item.item_type,
                attr,
                item.content.as_ref().unwrap_or(&"".to_string()),
                children,
                item.item_type
            ),
        }
    }

    pub fn css(&mut self, item: &mut Item) -> String {
        let id = if !item.attr.contains_key("id") {
            let id = format!("_{}", self.counter);
            item.attr.insert("id".to_string(), Attribute::from(&id));
            self.counter += 1;
            id
        } else {
            item.attr.get("id").unwrap().to_string()
        };

        if item.style.is_empty() {
            return item
                .children
                .as_mut()
                .map(|children| {
                    children
                        .iter_mut()
                        .map(|child| self.css(child))
                        .collect::<Vec<String>>()
                        .join("")
                })
                .unwrap_or_default();
        }

        let style = item
            .style
            .iter()
            .map(|(k, v)| format!(r#"{}:{}"#, k, v))
            .collect::<Vec<String>>()
            .join(";");

        let children = item
            .children
            .as_mut()
            .map(|children| {
                children
                    .iter_mut()
                    .map(|child| self.css(child))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .unwrap_or_default();

        format!("#{} {{{}}} {}", id, style, children)
    }
}

impl Default for ItemBuilder {
    fn default() -> Self {
        ItemBuilder { counter: 1 }
    }
}

pub struct Item {
    item_type: String,
    attr: HashMap<String, Attribute>,
    style: Vec<(String, String)>,
    single: Option<bool>,
    children: Option<Vec<Item>>,
    content: Option<String>,
}

impl Item {
    fn new(
        item_type: &str,
        attr: Option<HashMap<String, Attribute>>,
        style: Option<Vec<(String, String)>>,
        single: Option<bool>,
        children: Option<Vec<Item>>,
        content: Option<String>,
    ) -> Item {
        let item_type = item_type.to_string();
        let attr = attr.unwrap_or_default();
        let style = style.unwrap_or_default();

        Item {
            item_type,
            attr,
            style,
            single,
            children,
            content,
        }
    }

    pub fn push_child(&mut self, child: Item) {
        if let Some(children) = self.children.as_mut() {
            children.push(child)
        } else {
            self.children = Some(vec![child])
        }
    }

    pub fn style(content: String) -> Item {
        Item::new("style", None, None, None, None, Some(content))
    }
}

enum Attribute {
    String(String),
    Array(Vec<String>),
    Map(HashMap<String, Attribute>),
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Attribute::String(str) => str.to_string(),
            Attribute::Array(arr) => arr.join(" "),
            Attribute::Map(map) => map
                .iter()
                .map(|(k, v)| format!("{{{k}: {v}}}"))
                .fold(String::new(), |acc, kv| format!("{acc}\n{kv}")),
        };
        write!(f, "{}", value)
    }
}

impl From<Vec<(String, Attribute)>> for Attribute {
    fn from(value: Vec<(String, Attribute)>) -> Self {
        Attribute::Map(HashMap::from_iter(value))
    }
}

impl From<HashMap<String, Attribute>> for Attribute {
    fn from(value: HashMap<String, Attribute>) -> Self {
        Attribute::Map(value)
    }
}

impl From<&str> for Attribute {
    fn from(value: &str) -> Self {
        Attribute::String(value.to_string())
    }
}

impl From<&String> for Attribute {
    fn from(value: &String) -> Self {
        Attribute::String(value.to_string())
    }
}

impl From<String> for Attribute {
    fn from(value: String) -> Self {
        Attribute::String(value)
    }
}

const ICON_PATH: [&str; 3] = [
    "M67.506,83.066 C70.000,80.576 74.037,80.582 76.522,83.080 C79.008,85.578 79.002,89.622 76.508,92.112 L65.435,103.169 C55.219,113.370 38.560,113.518 28.172,103.513 C28.112,103.455 23.486,98.920 8.227,83.957 C-1.924,74.002 -2.936,58.074 6.616,47.846 L24.428,28.774 C33.910,18.621 51.387,17.512 62.227,26.278 L78.405,39.362 C81.144,41.577 81.572,45.598 79.361,48.342 C77.149,51.087 73.135,51.515 70.395,49.300 L54.218,36.217 C48.549,31.632 38.631,32.262 33.739,37.500 L15.927,56.572 C11.277,61.552 11.786,69.574 17.146,74.829 C28.351,85.816 36.987,94.284 36.997,94.294 C42.398,99.495 51.130,99.418 56.433,94.123 L67.506,83.066 Z",
    "M49.412,2.023 C51.817,-0.552 55.852,-0.686 58.423,1.722 C60.994,4.132 61.128,8.173 58.723,10.749 L15.928,56.572 C11.277,61.551 11.786,69.573 17.145,74.829 L36.909,94.209 C39.425,96.676 39.468,100.719 37.005,103.240 C34.542,105.760 30.506,105.804 27.990,103.336 L8.226,83.956 C-1.924,74.002 -2.936,58.074 6.617,47.846 L49.412,2.023 Z",
    "M40.606,72.001 C37.086,72.001 34.231,69.142 34.231,65.614 C34.231,62.087 37.086,59.228 40.606,59.228 L87.624,59.228 C91.145,59.228 94,62.087 94,65.614 C94,69.142 91.145,72.001 87.624,72.001 L40.606,72.001 Z"
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style;

    #[test]
    fn stringify() {
        let mut builder = ItemBuilder::default();

        let mut item = Item::new("g", None, None, Some(true), None, None);

        let stringify = builder.stringify(&mut item);
        assert_eq!(stringify, r#"<g id="_1"/>"#);

        let mut item = Item::new(
            "g",
            None,
            None,
            Some(false),
            None,
            Some("some content".to_string()),
        );

        let stringify = builder.stringify(&mut item);
        assert_eq!(stringify, r#"<g id="_2">some content</g>"#);
    }

    #[test]
    fn css() {
        let mut builder = ItemBuilder::default();

        let mut item = Item::new("g", None, None, None, None, None);

        let css = builder.css(&mut item);
        assert_eq!(css, "");

        let style = style! {
            "font-weight": "26px"
        };

        let mut item = Item::new("g", None, Some(style), None, None, None);

        let css = builder.css(&mut item);
        assert_eq!(css, "#_2 {font-weight:26px }");
    }
}

impl Item {
    pub fn root(config: &Config, user_info: &UserInfo) -> Item {
        let attr = attribute!({
            "id": "root",
            "width": format!("{}", config.width),
            "height": format!("{}", config.height),
            "viewBox": format!("0 0 {} {}", config.width, config.height),
            "version": "1.1",
            "xmlns": "http://www.w3.org/2000/svg",
            "xmlns:xlink": "http://www.w3.org/1999/xlink"
        });
        let style = style! {
            "fill": "none"
        };
        let backgroud_style = style! {
            "transform": "translate(0.5px, 0.5px)",
            "stroke": "var(--bg-2)",
            "fill": "var(--bg-0)",
            "stroke-width": 1,
            "width": format!("{}px", config.width - 1),
            "height": format!("{}px", config.height - 1),
            "rx": "4px",
        };

        let childs = vec![
            Item::new("title", None, None, None, None, Some(format!("{} | LeetCode Stat Card", user_info.username))),
            Item::new("style", Some(attribute!({"id": "default-colors"})), None, None, None, Some(String::from("svg{opacity:0}:root{--bg-0:#fff;--bg-1:#e5e5e5;--bg-2:#d3d3d3;--bg-3:#d3d3d3;--text-0:#000;--text-1:#808080;--text-2:#808080;--text-3:#808080;--color-0:#ffa116;--color-1:#5cb85c;--color-2:#f0ad4e;--color-3:#d9534f}"))),
            Item::new("rect", Some(attribute!({"id": "background"})), Some(backgroud_style), Some(true), None, None),
        ];

        Item::new("svg", Some(attr), Some(style), None, Some(childs), None)
    }

    pub fn icon() -> Item {
        let style = style! {
            "stroke": "none",
            "fill": "var(--text-0)",
            "fill-rule": "evenodd",
        };
        let child_1_attr = attribute!({
            "id": "C",
            "d": ICON_PATH[0],
        });
        let child_1_style = style! {
            "fill": "#FFA116",
            "fill-rule": "nonzero"
        };
        let child_2_attr = attribute!({
            "id": "L",
            "d": ICON_PATH[1]
        });
        let child_2_style = style! {
            "fill": "#000000",
        };
        let child_3_attr = attribute!({
            "id": "dash",
            "d": ICON_PATH[2]
        });
        let child_3_style = style! {
            "fill": "#B3B3B3",
        };

        let icon_path = Self::new(
            "g",
            None,
            Some(style),
            None,
            Some(vec![
                Item::new(
                    "path",
                    Some(child_1_attr),
                    Some(child_1_style),
                    None,
                    None,
                    None,
                ),
                Item::new(
                    "path",
                    Some(child_2_attr),
                    Some(child_2_style),
                    None,
                    None,
                    None,
                ),
                Item::new(
                    "path",
                    Some(child_3_attr),
                    Some(child_3_style),
                    None,
                    None,
                    None,
                ),
            ]),
            None,
        );

        let attr = attribute!({"id": "icon"});

        let style = style! {
            "transform": "translate(20px, 15px) scale(0.27)",
        };

        Item::new(
            "g",
            Some(attr),
            Some(style),
            None,
            Some(vec![icon_path]),
            None,
        )
    }

    pub fn username(username: &str) -> Item {
        let attr = attribute!({
            "id": "username",
            "href": format!("https://leetcode.com/{username}/"),
            "target": "_blank"
        });
        let style = style! {
            "transform": "translate(65px, 40px)",
        };

        let child_attr = attribute!({
            "id": "username-text",
        });
        let child_style = style! {
            "fill": "var(--text-0)",
            "font-size": "24px",
            "font-weight": "bold",
        };

        let child = Item::new(
            "text",
            Some(child_attr),
            Some(child_style),
            None,
            None,
            Some(username.to_string()),
        );

        Item::new("a", Some(attr), Some(style), None, Some(vec![child]), None)
    }

    pub fn ranking(ranking: u32) -> Item {
        let attr = attribute!({
            "id": "ranking",
        });
        let style = style! {
            "transform": "translate(480px, 40px)",
            "fill": "var(--text-1)",
            "font-size": "18px",
            "font-weight": "bold",
            "text-anchor": "end"
        };

        Item::new(
            "text",
            Some(attr),
            Some(style),
            None,
            None,
            Some(format!("#{ranking}")),
        )
    }

    pub fn total_solved(solved: u32, total: u32) -> Item {
        let attr = attribute!({
            "id": "total-solved",
        });
        let style = style! {
            "transform": "translate(30px, 85px)"
        };

        let circle_bg_attr = attribute!({
            "id": "total-solved-bg"
        });
        let circle_bg_style = style! {
            "cx": "40px",
            "cy": "40px",
            "r": "40px",
            "stroke": "var(--bg-1)",
            "stroke-width": "6px"
        };
        let circle_ring_attr = attribute!({
            "id": "total-solved-ring"
        });
        let circle_ring_style = style! {
            "cx": "40px",
            "cy": "40px",
            "r": "40px",
            "transform": "rotate(-90deg)",
            "transform-origin": "40px 40px",
            "stroke-dasharray": format!("{} 10000", 80.0 * std::f64::consts::PI * solved as f64 / total as f64),
            "stroke": "var(--color-0)",
            "stroke-width": "6px",
            "stroke-linecap": "round"
        };
        let text_attr = attribute!({
            "id": "total-solved-text"
        });
        let text_style = style! {
            "transform": "translate(40px, 40px)",
            "font-size": "28px",
            "alignment-baseline": "central",
            "dominant-baseline": "central",
            "text-anchor": "middle",
            "fill": "var(--text-0)",
            "font-weight": "bold"
        };

        let circle_bg = Item::new(
            "circle",
            Some(circle_bg_attr),
            Some(circle_bg_style),
            None,
            None,
            None,
        );
        let circle_ring = Item::new(
            "circle",
            Some(circle_ring_attr),
            Some(circle_ring_style),
            None,
            None,
            None,
        );
        let text = Item::new(
            "text",
            Some(text_attr),
            Some(text_style),
            None,
            None,
            Some(format!("{solved}")),
        );

        Item::new(
            "g",
            Some(attr),
            Some(style),
            None,
            Some(vec![circle_bg, circle_ring, text]),
            None,
        )
    }

    pub fn solved(problems: Vec<Problem>) -> Item {
        let attr = attribute!({
            "id": "solved",
        });
        let style = style! {
            "transform": "translate(160px, 80px)"
        };

        let childs = [
            (Difficulty::Easy, "var(--color-1)"),
            (Difficulty::Medium, "var(--color-2)"),
            (Difficulty::Hard, "var(--color-3)"),
        ]
        .iter()
        .enumerate()
        .map(|(i, (d, c))| {
            let problem = problems.iter().find(|p| p.difficulty == *d).unwrap();
            let pb_type = Item::new(
                "text",
                Some(attribute!({
                    "id": format!("{}-solved-type", d),
                })),
                Some(style! {
                    "fill": "var(--text-1)",
                    "font-size": "18px",
                    "font-weight": "bold"
                }),
                None,
                None,
                Some(d.capitalize()),
            );
            let count = Item::new(
                "text",
                Some(attribute!({
                    "id": format!("{}-solved-count", d),
                })),
                Some(style! {
                    "transform": "translate(300px, 0px)",
                    "fill": "var(--text-1)",
                    "font-size": "16px",
                    "font-weight": "bold",
                    "text-anchor": "end"
                }),
                None,
                None,
                Some(format!("{} / {}", problem.count, problem.total)),
            );
            let line_bg = Item::new(
                "line",
                Some(attribute!({
                    "id": format!("{}-solved-bg", d),
                    "x1": "0",
                    "y1": "10",
                    "x2": "300",
                    "y2": "10",
                })),
                Some(style! {
                    "stroke": "var(--bg-1)",
                    "stroke-width": "4px",
                    "stroke-linecap": "round"
                }),
                None,
                None,
                None,
            );
            let line_progress = Item::new(
                "line",
                Some(attribute!({
                    "id": format!("{}-solved-progress", d),
                    "x1": "0",
                    "y1": "10",
                    "x2": "300",
                    "y2": "10",
                })),
                Some(style! {
                    "stroke": c,
                    "stroke-width": "4px",
                    "stroke-dasharray": format!("{} 10000", 300.0 * problem.count as f64 / problem.total as f64),
                    "stroke-linecap": "round"
                }),
                None,
                None,
                None,
            );

            Item::new(
                "g",
                Some(attribute!({
                    "id": format!("{}-solved", d),
                })),
                Some(style! {
                    "transform": format!("translate(0px, {}px)", 40 * i),
                }),
                None,
                Some(vec![pb_type, count, line_bg, line_progress]),
                None,
            )
        })
        .collect::<Vec<Item>>();

        Item::new("g", Some(attr), Some(style), None, Some(childs), None)
    }
}
