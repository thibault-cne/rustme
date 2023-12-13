use super::{Generator, Item, UserInfo};

pub trait Extension: std::fmt::Debug {
    fn extend(
        self,
        generator: &Generator,
        user_info: &mut UserInfo,
        body: &mut Vec<Item>,
        style: &mut Vec<String>,
    );
}
