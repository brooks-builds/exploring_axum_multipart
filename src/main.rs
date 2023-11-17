use axum_json::Server;

#[tokio::main]
async fn main() {
    let app = Server::new();
    match app.run().await {
        Ok(_) => println!("Server exited"),
        Err(error) => panic!("Server exited with error: {error}"),
    }
}
