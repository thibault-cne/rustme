pub mod font;
pub mod item;
pub mod macros;
pub mod theme;

use std::future::Future;

use item::Item;

pub trait Generator: Default + Send {
    fn generate(self) -> impl Future<Output = String> + Send;
}

pub trait Extension<G: Generator>: Send {
    fn extend(&self, generator: &mut G, body: &mut Vec<Item>, style: &mut Vec<String>);
}

pub fn minimize_css(css: &str) -> String {
    let css = css.replace(' ', "");
    css.replace('\n', "")
}
