mod leetcode;

#[tokio::main]
async fn main() {
    let mut config = leetcode::Config::new("thibault-cne");
    let animation = Box::new(leetcode::Animation);
    let themes = Box::new(leetcode::Themes::from(vec![
        leetcode::theme::DARK,
        leetcode::theme::LIGHT,
    ]));
    let fonts: Box<leetcode::Fonts> = Box::new(vec![leetcode::BALOO_2].into());
    config.add_extension(animation);
    config.add_extension(themes);
    config.add_extension(fonts);

    let generator = leetcode::Generator::new(config);

    let content = generator.generate().await;

    println!("{}", content);
}
