use base64;
use reqwest;

use failure::ResultExt;
use openssl::rsa::{Padding, Rsa};

/// Travis
#[derive(Debug, Clone, Deserialize)]
struct TravisKey {
  key: String,
}

pub fn travis_url() {}

/// Get the public key for a repo from Travis.
fn get_travis_key(username: &str, repo: &str) -> ::Result<String> {
  let url =
    format!("https://api.travis-ci.org/repos/{}/{}/key", username, repo);
  info!("GET {}", &url);
  let key: TravisKey = reqwest::get(&url)
    .context(::ErrorKind::Travis)?
    .json()
    .context(::ErrorKind::Travis)?;
  Ok(key.key)
}

/// Encrypt a travis key.
pub fn encrypt(username: &str, repo: &str, data: &str) -> ::Result<String> {
  let pub_key = get_travis_key(username, repo)?;

  let rsa = Rsa::public_key_from_pem(pub_key.as_bytes())
    .context(::ErrorKind::Travis)?;

  let mut res = vec![0; rsa.size() as usize];
  rsa
    .public_encrypt(data.as_bytes(), &mut res, Padding::PKCS1)
    .context(::ErrorKind::Travis)?;

  Ok(base64::encode(&res))
}
