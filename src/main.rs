#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

#[macro_use]
extern crate human_panic;
extern crate cargo_toml;
extern crate crossgen;
extern crate exitfailure;
extern crate log;
extern crate serde;
extern crate structopt;

use cargo_toml::TomlManifest;
use crossgen::Cli;
use exitfailure::ExitFailure;
use std::fs::read;
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
  setup_panic!();
  let args = Cli::from_args();
  args.log(env!("CARGO_PKG_NAME"))?;

  let dir = args.dir()?;
  let name = args.name()?;
  let lib = args.lib().unwrap_or(false);

  let manifest = TomlManifest::from_slice(&read("Cargo.toml")?)?;
  let repo = manifest
    .package
    .repository
    .expect("No repository found in Cargo.toml");

  let parts: Vec<&str> = repo.split("/").collect();
  let username = parts.get(3).expect("Could not access the username portion of the repository field from Cargo.toml");
  let project = parts.get(4).expect("Could not access the project name portion of the repository field from Cargo.toml");

  let token = crossgen::authenticate(env!("CARGO_PKG_NAME"))?;
  let token = crossgen::encrypt(username, project, &token)?;
  let templ = if lib {
    crossgen::Templates::gen_lib(dir, name, token)
  } else {
    crossgen::Templates::gen_bin(dir, name, token)
  }?;

  templ.write_all()?;

  Ok(())
}
