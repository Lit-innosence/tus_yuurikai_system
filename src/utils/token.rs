use rand::{distributions::Alphanumeric, Rng};

/// ### generate_token
/// [A-Za-z0-9]のランダムな16文字のトークンを作成する
pub fn generate_token() -> String{
    let mut rng = rand::thread_rng();
    let token: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();
    token
}