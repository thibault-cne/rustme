use core::Generator as GeneratorTrait;
use leetcode::{
    extension::Extension,
    font::BALOO_2,
    theme::{DARK, LIGHT},
    Config, Generator,
};

#[tokio::main]
async fn main() {
    let mut config = Config::new("thibault-cne");
    let animation = Extension::Animation;
    let themes = Extension::from(vec![DARK, LIGHT]);
    let fonts = Extension::from(vec![BALOO_2]);
    config.add_extension(animation);
    config.add_extension(themes);
    config.add_extension(fonts);

    let generator = Generator::new(config);

    let content = generator.generate().await;

    println!("{}", content);
}
