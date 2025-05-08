use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::{rng, Rng};

const KEY_NAME: &str = "organizations/{org_id}/apiKeys/{key_id}";
const PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AfEGCCqGSM49AwE12BdddwawIBAQQgcto6lv7FAjpzOIzr
hcbe/9HC6tDlyK/i7zchMbROYTShsANsAARXqaQJvQOddLh2ObcICsIGi7Bn2IHUv0
GJbf7TjKcBDyAD4Y/OFNbbsjLeqMsQJ6A0gjykjcMddpTPpKdCtBYu8I1C
-----END PRIVATE KEY-----

";
const HOST: &str = "api.coinbase.com";


fn main() {
    println!("Hello, world!");
}

fn generate_nonce() -> String {
    let mut rng = rng();
    let nonce: [u8; 16] = rng.random();
    hex::encode(nonce)
}

fn create_jwt(method: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key_name = KEY_NAME;
    let private_key = PRIVATE_KEY;
    let method = method;
    let host = HOST;
    let path = path;

    let uri = format!("{} {}{}", method, host, path);
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let mut header = Header::new(Algorithm::ES256);
    header.kid = Some(key_name.to_string());
    header.typ = Some("JWT".to_string());

    let payload = serde_json::json!({
    "iss": "cdp",
    "nbf": now,
    "exp": now + 120,
    "sub": key_name,
    "uri": uri,
    "nonce": generate_nonce()
});

    Ok(encode(
        &header,
        &payload,
        &EncodingKey::from_ec_pem(private_key.as_bytes())?, // PKCS#8 format!
    )?)
}