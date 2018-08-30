#[macro_use]
extern crate structopt;
extern crate clap_flags;
#[macro_use]
extern crate failure;
extern crate mkdirp;
extern crate reqwest;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate github_auth;
extern crate openssl;
extern crate toml;

mod cli;
mod error;
mod github;
mod templates;
mod travis;

pub use cli::Cli;
pub use error::{Error, ErrorKind, Result};
pub use github::*;
pub use templates::*;
pub use travis::*;
