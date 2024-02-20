use core::Generator as GeneratorTrait;
use leetcode::{extension::Extension, Config, Generator};

#[tokio::main]
async fn main() {
    let config = Config::new("thibault-cne");
    let animation = Extension::Animation;
    let config = config.add_extension(animation);

    let mut generator = Generator::new(config);
    generator.verbose();

    let content = generator.generate().await.unwrap();

    println!("{}", content);
}
