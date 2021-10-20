use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use lazy_static::lazy_static;
use sha2::Sha256;
use std::{collections::BTreeMap, env::var};
use types::users::User;

lazy_static! {
    static ref KEY: String = var("SECRET_KEY").unwrap();
}

pub fn create_key(user: &User) -> String {
    let key = Hmac::<Sha256>::new_from_slice(KEY.as_bytes()).unwrap();

    let User { id, username } = user;

    let mut map = BTreeMap::default();

    map.insert("sub", id.to_string());
    map.insert("name", username.to_owned());

    map.sign_with_key(&key).unwrap()
}
