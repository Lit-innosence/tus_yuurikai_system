use rand::{distributions::Alphanumeric, Rng};

// トークンの生成
pub fn generate_token() -> String{
    let mut rng = rand::thread_rng();
    let token: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();
    token
}