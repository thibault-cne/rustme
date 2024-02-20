pub mod font;
pub mod item;
pub mod macros;
pub mod theme;

use item::Item;

pub trait Generator: Default + Send + Sync {
    fn generate(self) -> impl std::future::Future<Output = error::Result<String>>;
}

pub trait Extension<G: Generator>: Send + Sync {
    fn extend(
        &self,
        generator: &mut G,
        body: &mut Vec<Item>,
        style: &mut Vec<String>,
    ) -> impl std::future::Future<Output = error::Result<()>>;
}

pub fn minimize_css(css: &str) -> String {
    let css = css.replace(' ', "");
    css.replace('\n', "")
}
