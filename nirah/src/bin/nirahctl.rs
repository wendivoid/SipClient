#[macro_use]
extern crate log;
extern crate nirah;

#[tokio::main]
async fn main() {
    if let Err(err) = nirah::cli::run().await {
        error!("{:?}", err);
    }
}
