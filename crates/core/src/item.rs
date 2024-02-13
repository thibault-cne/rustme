use std::collections::HashMap;

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
    pub fn new(
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

pub enum Attribute {
    String(String),
    Array(Vec<String>),
    Map(HashMap<String, Attribute>),
}

impl std::fmt::Display for Attribute {
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
