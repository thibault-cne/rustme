mod leetcode;

#[tokio::main]
async fn main() {
    let config = leetcode::Config::new("thibaultcne");
    let generator = leetcode::Generator::new(config);

    let content = generator.generate().await;

    println!("{}", content);
}
