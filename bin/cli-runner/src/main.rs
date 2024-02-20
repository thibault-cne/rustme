use core::Generator as GeneratorTrait;
use leetcode::{extension::Extension, font::FORMULA_1, theme::FERRARI, Config, Generator};

#[tokio::main]
async fn main() {
    let config = Config::new("thibault-cne");
    let animation = Extension::Animation;
    let themes = Extension::from(vec![FERRARI]);
    let font = Extension::from(FORMULA_1);
    let config = config
        .add_extension(animation)
        .add_extension(themes)
        .add_extension(font);

    let generator = Generator::new(config);

    let content = generator.generate().await;

    println!("{}", content);
}
