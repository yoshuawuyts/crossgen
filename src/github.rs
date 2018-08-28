use failure::ResultExt;
use github_auth::Authenticator;

/// Authenticate with GitHub.
pub fn authenticate(app_name: &str) -> ::Result<String> {
  let auth = Authenticator::new(app_name.to_string());
  let token = auth.auth().context(::ErrorKind::GitHub)?;
  Ok(token.into_string())
}
