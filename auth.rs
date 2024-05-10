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
type TokenCache = Arc<Mutex<HashMap<String, String>>>;

fn init_users_db() -> UsersDb {
    Arc::new(Mutex::new(HashMap::new()))
}

fn init_token_cache() -> TokenCache {
    Arc::new(Mutex::new(HashMap::new()))
}

fn register_user(users_db: UsersDb, username: &str, password: &str) -> Result<(), &'static str> {
    let mut db = users_db.lock().unwrap();
    if db.contains_key(username) {
        Err("User already exists")
    } else {
        db.insert(username.to_string(), create_user(username, password));
        Ok(())
    }
}

fn create_user(username: &str, password: &str) -> User {
    User {
        username: username.to_string(),
        password: password.to_string(),
    }
}

fn login_user(users_db: UsersDb, token_cache: TokenCache, username: &str, password: &str) -> Result<String, &'static str> {
    let user = authenticate_user(&users_db, username, password)?;

    let token_cache_guard = token_cache.lock().unwrap();
    if let Some(token) = token_cache_guard.get(username) {
        return Ok(token.clone());
    }

    drop(token_cache_guard); // Explicitly drop the lock
    let token = create_token_for_user(&user)?;
    token_cache.lock().unwrap().insert(username.to_string(), token.clone());

    Ok(token)
}

fn authenticate_user(users_db: &UsersDb, username: &str, password: &str) -> Result<User, &'static str> {
    let db = users_db.lock().unwrap();
    db.get(username)
        .filter(|user| user.password == password)
        .cloned()
        .ok_or("Invalid username or password")
}

fn create_token_for_user(user: &User) -> Result<String, String> {
    let expiration = calculate_expiration(1); // 1 day
    let claims = Claims {
        sub: user.username.clone(),
        exp: expiration,
    };

    let header = Header::new(Algorithm::HS256);
    let secret = env::var(SECRET_KEY).expect("SECRET_KEY must be set");
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|e| e.to_string())
}

fn calculate_expiration(days: i64) -> usize {
    chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(days))
        .expect("valid timestamp")
        .timestamp() as usize
}

fn verify_token(_token: &str) -> bool {
    true // Placeholder for actual verification logic
}

fn main() {
    dotenv::dotenv().ok();

    let users_db = init_users_db();
    let token_cache = init_token_cache();

    match register_user(users_db.clone(), "newuser", "password123") {
        Ok(_) => println!("User registered successfully."),
        Err(e) => println!("Error registering user: {}", e),
    }

    match login_user(users_db, token_cache.clone(), "newuser", "password123") {
        Ok(token) => println!("Logged in successfully. Token: {}", token),
        Err(e) => println!("Error logging in: {}", e),
    }
}