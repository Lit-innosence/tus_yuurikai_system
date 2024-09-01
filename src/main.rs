use tus_yuurikai_system::infrastracture;

#[rocket::main]
async fn main() {
    infrastracture::router::run();
}