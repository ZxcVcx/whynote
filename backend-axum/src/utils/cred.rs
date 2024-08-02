use base64::Engine;
use jsonwebtoken::{decode, errors::Error, DecodingKey, TokenData, Validation};
use ring::{digest, pbkdf2};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

use crate::utils::constants::CFG;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;

async fn salt(username: &str) -> Vec<u8> {
    let db_salt_component: [u8; 16] = [
        // https://docs.rs/ring/latest/ring/pbkdf2/index.html
        // https://github.com/FlorisSteenkamp/squares-rng
        // This value was generated from a secure PRNG.
        0x19, 0x2f, 0xe4, 0xad, 0xce, 0x69, 0x4f, 0x71, 0x52, 0x83, 0xe6, 0x71, 0xfb, 0x59, 0x4e,
        0xcd,
    ];

    let mut salt = Vec::with_capacity(db_salt_component.len() + username.as_bytes().len());
    salt.extend(db_salt_component.as_ref());
    salt.extend(username.as_bytes());
    salt
}

pub async fn cred_encode(username: &str, password: &str) -> String {
    const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
    type Credential = [u8; CREDENTIAL_LEN];

    let salt = salt(username).await;

    let mut cred: Credential = [0u8; CREDENTIAL_LEN];

    pbkdf2::derive(
        PBKDF2_ALG,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        password.as_bytes(),
        &mut cred,
    );

    base64::engine::general_purpose::URL_SAFE.encode(cred)
}

pub async fn cred_verify(username: &str, pwd_try: &str, actual_credential: &str) -> bool {
    let salt = salt(username).await;

    let actual_cred_decode = base64::engine::general_purpose::URL_SAFE
        .decode(actual_credential)
        .unwrap();
    pbkdf2::verify(
        PBKDF2_ALG,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        pwd_try.as_bytes(),
        actual_cred_decode.as_slice(),
    )
    .is_ok()
}

#[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     pub email: String,
//     pub username: String,
//     pub exp: usize,
// }
pub struct Claims {
    pub email: String,
    pub username: String,
    // pub role: String,
    pub uat: u64,
    pub exp: u64,
}
impl Claims {
    pub fn new(email: String, username: String) -> Self {
        let uat = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            email,
            username,
            uat,
            exp: uat + CFG.get("CLAIM_EXP").unwrap().parse::<u64>().unwrap(),
        }
    }
}

pub async fn token_data(token: &str) -> Result<TokenData<Claims>, Error> {
    let decoding_key = DecodingKey::from_secret(CFG.get("JWT_SECRET").unwrap().as_bytes());
    decode::<Claims>(
        token,
        &decoding_key,
        &Validation::new(jsonwebtoken::Algorithm::HS512),
    )
}
