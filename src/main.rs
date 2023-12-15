mod leetcode;

#[tokio::main]
async fn main() {
    let mut config = leetcode::Config::new("thibaultcne");
    let ext = Box::new(leetcode::Animation);
    config.add_extension(ext);

    let generator = leetcode::Generator::new(config);

    let content = generator.generate().await;

    println!("{}", content);
}
