mod leetcode;

#[tokio::main]
async fn main() {
    let user_id = leetcode::Id::new("neal_wu");
    let user_info = leetcode::Client::default().get(user_id).await;

    println!("{:?}", user_info);
}
