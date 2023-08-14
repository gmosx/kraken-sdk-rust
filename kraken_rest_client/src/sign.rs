use crate::client::Result;
use crate::error::Error;
use hmac::{Hmac, Mac, NewMac};
use sha2::{Digest, Sha256, Sha512};
use std::time::{SystemTime, UNIX_EPOCH};

fn sha256(input: String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().to_vec()
}

fn sha512(input: Vec<u8>, secret: &[u8]) -> Result<Vec<u8>> {
    let mut mac = Hmac::<Sha512>::new_varkey(secret)?;
    mac.update(&input);
    Ok(mac.finalize().into_bytes().to_vec())
}

pub(crate) fn compute_nonce() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.as_millis() as u64
}

/// Computes the signature of the POST body
pub fn compute_signature(
    api_secret: &str,
    path: &str,
    nonce: &str,
    post_data: &str,
) -> Result<String> {
    let mut query = String::from("");

    query.push_str(nonce);
    query.push_str(post_data);

    let mut sha256_res = sha256(query);

    let mut to_hash = vec![];

    to_hash.append(&mut path.as_bytes().to_owned());
    to_hash.append(&mut sha256_res);

    let secret = base64::decode(api_secret).map_err(Error::internal)?;
    let sha512_res = sha512(to_hash, &secret)?;

    Ok(base64::encode(&sha512_res))
}

#[cfg(test)]
mod tests {
    use super::compute_nonce;

    #[test]
    fn test_compute_nonce() {
        let nonce1 = compute_nonce();

        std::thread::sleep(std::time::Duration::from_millis(2));

        let nonce2 = compute_nonce();

        assert!(nonce1 < nonce2);
    }
}
