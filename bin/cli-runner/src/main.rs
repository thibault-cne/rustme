use core::Generator as GeneratorTrait;
use leetcode::{
    extension::{Animation, Fonts, Themes},
    font::BALOO_2,
    theme::{DARK, LIGHT},
    Config, Generator,
};

#[tokio::main]
async fn main() {
    let mut config = Config::new("thibault-cne");
    let animation = Box::new(Animation);
    let themes = Box::new(Themes::from(vec![DARK, LIGHT]));
    let fonts = Box::new(Fonts::from(vec![BALOO_2]));
    config.add_extension(animation);
    config.add_extension(themes);
    config.add_extension(fonts);

    let generator = Generator::new(config);

    let content = generator.generate().await;

    println!("{}", content);
}
