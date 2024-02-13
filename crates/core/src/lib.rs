pub mod item;
pub mod macros;

use std::future::Future;

use item::Item;

pub trait Generator: Default + Send {
    fn generate(self) -> impl Future<Output = String> + Send;
}

pub trait Extension<G: Generator>: Send {
    fn extend(&self, generator: &G, body: &mut Vec<Item>, style: &mut Vec<String>);
}
