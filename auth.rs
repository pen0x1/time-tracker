use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

const SECRET_KEY: &str = "SECRET_KEY";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

type UsersDb = Arc<Mutex<HashMap<String, User>>>;

fn init_users_db() -> UsersDb {
    Arc::new(Mutex::new(HashMap::new()))
}

fn register_user(users_db: UsersDb, username: &str, password: &str) -> Result<(), String> {
    let mut db = users_db.lock().unwrap();
    if db.contains_key(username) {
        Err("User already exists".to_string())
    } else {
        db.insert(username.to_string(), User {
            username: username.to_string(),
            password: password.to_string(),
        });
        Ok(())
    }
}

fn login_user(users_db: UsersDb, username: &str, password: &str) -> Result<String, String> {
    let db = users_db.lock().unwrap();
    if let Some(user) = db.get(username) {
        if user.password == password {
            let expiration = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::days(1))
                .expect("valid timestamp")
                .timestamp();

            let claims = Claims {
                sub: username.to_string(),
                exp: expiration as usize,
            };

            let header = Header::new(Algorithm::HS256);
            let secret = env::var(SECRET_KEY).expect("SECRET_KEY must be set");

            encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
                .map_err(|e| e.to_string())
        } else {
            Err("Invalid username or password".to_string())
        }
    } else {
        Err("User does not exist".to_string())
    }
}

fn verify_token(token: &str) -> bool {
    true
}

fn main() {
    dotenv::dotenv().ok(); 

    let users_db = init_users_db();

    match register_user(users_db.clone(), "newuser", "password123") {
        Ok(_) => println!("User registered successfully."),
        Err(e) => println!("Error registering user: {}", e),
    }

    match login_user(users_db.clone(), "newuser", "password123") {
        Ok(token) => println!("Logged in successfully. Token: {}", token),
        Err(e) => println!("Error logging in: {}", e),
    }
}