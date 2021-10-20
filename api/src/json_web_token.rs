use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey};
use lazy_static::lazy_static;
use sha2::Sha256;
use std::{collections::BTreeMap, env::var};
use trillium::{Conn, Status};
use types::users::User;

lazy_static! {
    static ref KEY: String = var("SECRET_KEY").unwrap();
}

#[derive(Debug, Clone)]
pub enum UserAuth {
    User(User),
    Forbidden,
}

pub fn create_key(user: &User) -> String {
    let key = Hmac::<Sha256>::new_from_slice(KEY.as_bytes()).unwrap();

    let User { id, username } = user;

    let mut map = BTreeMap::default();

    map.insert("sub", id.to_string());
    map.insert("name", username.to_owned());

    map.sign_with_key(&key).unwrap()
}

pub fn validate_signature(jwt: String) -> UserAuth {
    let key = Hmac::<Sha256>::new_from_slice(KEY.as_bytes()).unwrap();

    let claims: BTreeMap<String, String> = match jwt.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(_) => return UserAuth::Forbidden,
    };

    if let (Some(sub), Some(name)) = (claims.get("sub"), claims.get("name")) {
        UserAuth::User(User {
            id: sub.parse::<i32>().unwrap(),
            username: name.to_owned(),
        })
    } else {
        UserAuth::Forbidden
    }
}

pub async fn user_handler(conn: Conn) -> Conn {
    if let Some(auth) = conn.headers().get("Authorization").cloned() {
        if let Some(auth_bearer) = auth.as_str() {
            if let Some((_, jwt)) = auth_bearer.split_once(' ') {
                match validate_signature(jwt.to_string()) {
                    UserAuth::User(u) => conn.with_state(u),
                    UserAuth::Forbidden => conn.with_status(Status::Forbidden),
                }
            } else {
                conn.with_status(Status::Forbidden)
            }
        } else {
            conn.with_state(UserAuth::Forbidden)
        }
    } else {
        conn.with_state(UserAuth::Forbidden)
    }
}
