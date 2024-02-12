mod leetcode;

#[tokio::main]
async fn main() {
    let mut config = leetcode::Config::new("thibaultcne");
    let animation = Box::new(leetcode::Animation);
    let themes = Box::new(leetcode::Themes::from(vec![
        leetcode::theme::DARK,
        leetcode::theme::LIGHT,
    ]));
    config.add_extension(animation);
    config.add_extension(themes);

    let generator = leetcode::Generator::new(config);

    let content = generator.generate().await;

    println!("{}", content);
}
