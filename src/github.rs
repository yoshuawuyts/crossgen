use failure::ResultExt;
use github_auth::{Authenticator, Scope};

/// Authenticate with GitHub.
pub fn authenticate(app_name: &str) -> ::Result<String> {
  let auth = Authenticator::builder(app_name.to_string())
    .scope(Scope::PublicRepo)
    .build();
  let token = auth.auth().context(::ErrorKind::GitHub)?;
  Ok(token.into_string())
}
