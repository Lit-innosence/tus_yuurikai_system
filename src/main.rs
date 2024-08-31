mod infrastracture;


async fn main() {
    infrastracture::router::run().await;
}