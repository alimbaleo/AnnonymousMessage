use core::hash;
use argon2::{self, Config, };

pub fn hash_password(password: &String ) -> String{

    let salt = format!("{password}{password}");
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
    return hash;
}

pub fn verify_password(hash: &String, plain_text:&String ) -> bool{

    let config = Config::default();
    argon2::verify_encoded(hash, plain_text.as_bytes()).unwrap()
}
