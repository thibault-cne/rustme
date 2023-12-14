use std::collections::HashMap;
use std::fmt::Display;

use super::{Config, UserInfo};

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

        format!("#{} {{{} {}}}", id, style, children)
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
    style: HashMap<String, String>,
    single: Option<bool>,
    children: Option<Vec<Item>>,
    content: Option<String>,
}

impl Item {
    fn new(
        item_type: &str,
        attr: Option<HashMap<String, Attribute>>,
        style: Option<HashMap<String, String>>,
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
        let style = style!({
            "fill": "none"
        });
        let backgroud_style = style!({
            "transform": "translate(0.5px, 0.5px)",
            "stroke": "var(--bg-2)",
            "fill": "var(--bg-0)",
            "stroke-width": 1,
            "width": format!("{} px", config.width - 1),
            "height": format!("{} px", config.height - 1),
            "rx": "4px",
        });

        let childs = vec![
            Item::new("title", None, None, None, None, Some(format!("{} | LeetCode Stat Card", user_info.username))),
            Item::new("style", Some(attribute!({"id": "default-colors"})), None, None, None, Some(String::from("svg{opacity:0}:root{--bg-0:#fff;--bg-1:#e5e5e5;--bg-2:#d3d3d3;--bg-3:#d3d3d3;--text-0:#000;--text-1:#808080;--text-2:#808080;--text-3:#808080;--color-0:#ffa116;--color-1:#5cb85c;--color-2:#f0ad4e;--color-3:#d9534f}"))),
            Item::new("rect", Some(attribute!({"id": "background"})), Some(backgroud_style), None, None, None),
        ];

        Item::new("svg", Some(attr), Some(style), None, Some(childs), None)
    }

    pub fn icon() -> Item {
        let style = HashMap::from_iter(vec![
            ("stroke".to_string(), Attribute::from("none")),
            ("fill".to_string(), Attribute::from("var(--text-0)")),
            ("fill-rule".to_string(), Attribute::from("evenodd")),
        ]);
        let child_1_attr = attribute!({
            "id": "C",
            "d": ICON_PATH[0],
        });
        let child_1_style = style!({
            "fill": "#FFA116",
            "fill-rule": "nonzero"
        });
        let child_2_attr = attribute!({
            "id": "L",
            "d": ICON_PATH[1]
        });
        let child_2_style = style!({
            "fill": "#000000",
        });
        let child_3_attr = attribute!({
            "id": "dash",
            "d": ICON_PATH[2]
        });
        let child_3_style = style!({
            "fill": "#B3B3B3",
        });

        Self::new(
            "g",
            Some(style),
            None,
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
        )
    }

    pub fn username(username: &str) -> Item {
        let attr = attribute!({
            "id": "username",
            "href": format!("https://leetcode.com/{username}/"),
            "target": "_blank"
        });
        let style = style!({
            "transform": "translate(65px, 40px)",
        });

        Item::new("a", Some(attr), Some(style), None, None, None)
    }

    pub fn ranking(ranking: u32) -> Item {
        let attr = attribute!({
            "id": "ranking",
            "content": format!("#{ranking}"),
        });
        let style = style!({
            "transform": "translate(480px, 40px)",
            "fill": "var(--text-1)",
            "font-size": "18px",
            "font-weight": "bold",
            "text-anchor": "end"
        });

        Item::new("text", Some(attr), Some(style), None, None, None)
    }
}

mod macros {
    #[macro_export]
    macro_rules! style {
        ({ $($key:literal: $value:expr),* }) => {
            std::collections::HashMap::<String, String>::from_iter(vec![
                $(
                    ($key.to_string(), $value.to_string())
                ),*
            ])
        };
        ({ $($key:literal: $value:expr),*, }) => {
            std::collections::HashMap::<String, String>::from_iter(vec![
                $(
                    ($key.to_string(), $value.to_string())
                ),*
            ])
        };
    }

    #[macro_export]
    macro_rules! attribute {
        // Done with trailing comma.
        (@array [$($elems:expr,)*]) => {
            attribute_internal_vec![$($elems,)*]
        };

        // Done without trailing comma.
        (@array [$($elems:expr),*]) => {
            attribute_internal_vec![$($elems),*]
        };

        // Next element is `null`.
        (@array [$($elems:expr,)*] null $($rest:tt)*) => {
            attribute!(@array [$($elems,)* attribute!(null)] $($rest)*)
        };

        // Next element is `true`.
        (@array [$($elems:expr,)*] true $($rest:tt)*) => {
            attribute!(@array [$($elems,)* attribute!(true)] $($rest)*)
        };

        // Next element is `false`.
        (@array [$($elems:expr,)*] false $($rest:tt)*) => {
            attribute!(@array [$($elems,)* attribute!(false)] $($rest)*)
        };

        // Next element is an array.
        (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
            attribute!(@array [$($elems,)* attribute!([$($array)*])] $($rest)*)
        };

        // Next element is a map.
        (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
            attribute!(@array [$($elems,)* attribute!({$($map)*})] $($rest)*)
        };

        // Next element is an expression followed by comma.
        (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
            attribute!(@array [$($elems,)* attribute!($next),] $($rest)*)
        };

        // Last element is an expression with no trailing comma.
        (@array [$($elems:expr,)*] $last:expr) => {
            attribute!(@array [$($elems,)* attribute!($last)])
        };

        // Comma after the most recent element.
        (@array [$($elems:expr),*] , $($rest:tt)*) => {
            attribute!(@array [$($elems,)*] $($rest)*)
        };

        // Done
        (@map $object:ident () () ()) => {};

        // Insert the current entry followed by trailing comma.
        (@map $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
            let _ = $object.insert(($($key)+).into(), $value);
            attribute!(@map $object () ($($rest)*) ($($rest)*));
        };

        // Insert the current entry followed by trailing comma.
        (@map $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
            let _ = $object.insert(($($key)+).into(), $value);
            attribute!(@map $object () ($($rest)*) ($($rest)*));
        };

        // Insert the last entry without trailing comma.
        (@map $object:ident [$($key:tt)+] ($value:expr)) => {
            let _ = $object.insert(($($key)+).into(), $value);
        };

        // Next value is an array.
        (@map $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
            attribute!(@map $object [$($key)+] (attribute!([$($array)*])) $($rest)*);
        };

        // Next value is a map.
        (@map $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
            attribute!(@map $object [$($key)+] (attribute!({$($map)*})) $($rest)*);
        };

        // Next value is an expression followed by comma.
        (@map $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
            attribute!(@map $object [$($key)+] (attribute!($value)) , $($rest)*);
        };

        // Last value is an expression with no trailing comma.
        (@map $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
            attribute!(@map $object [$($key)+] (attribute!($value)));
        };

        // Key is fully parenthesized. This avoids clippy double_parens false
        // positives because the parenthesization may be necessary here.
        (@map $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
            attribute!(@map $object ($key) (: $($rest)*) (: $($rest)*));
        };

        // Munch a token into the current key.
        (@map $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
            attribute!(@map $object ($($key)* $tt) ($($rest)*) ($($rest)*));
        };

        ([]) => {
            Attribute::from(vec![])
        };

        ([ $($tt:tt)+ ]) => {
            Attribute::from(attribute!(@array [] $($tt)+))
        };

        ({}) => {
            Attribute::from(HashMap::<String, Attribute>::new())
        };

        ({ $($tt:tt)+ }) => {
            {
                let mut map = std::collections::HashMap::<String, Attribute>::new();
                attribute!(@map map () ($($tt)+) ($($tt)+));
                map
            }
        };

        ($other:expr) => {
            Attribute::from($other)
        }
    }

    #[macro_export]
    macro_rules! attribute_internal_vec {
        ($($content:tt)*) => {
            vec![$($content)*]
        };
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

        let style = style!({
            "font-weight": "26px"
        });

        let mut item = Item::new("g", None, Some(style), None, None, None);

        let css = builder.css(&mut item);
        assert_eq!(css, "#_2 {font-weight:26px }");
    }
}
