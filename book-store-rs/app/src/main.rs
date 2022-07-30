// SPDX-License-Identifier: Apache-2.0
use book_store_lib::Server;

#[tokio::main]
async fn main() {
    let server = Server::default();
    if let Err(e) = server.run().await {
        println!("[ERROR] {}", e);
    }
}
