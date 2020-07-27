#[tokio::main]
async fn main() {
    let status_list = mojang_api::retrieve_status().await.unwrap();
    for (server, status) in status_list {
        println!("{}: {}", server, status);
    }
}
