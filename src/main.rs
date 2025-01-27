
mod service;
mod gui;

#[tokio::main]
async fn main() {
    let result   =   gui::gui::run().await;
    result.unwrap();
}






