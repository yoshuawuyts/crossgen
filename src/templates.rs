use failure::ResultExt;
use mkdirp::mkdirp;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

static BIN_DEPLOY_PS1: &str =
  include_str!("../templates/bin/before_deploy.ps1");
static BIN_DEPLOY_SH: &str = include_str!("../templates/bin/before_deploy.sh");
static BIN_INSTALL: &str = include_str!("../templates/bin/install.sh");
static BIN_SCRIPT: &str = include_str!("../templates/bin/script.sh");
static BIN_TRAVIS: &str = include_str!("../templates/bin/travis.yml");

static LIB_DEPLOY_PS1: &str =
  include_str!("../templates/lib/before_deploy.ps1");
static LIB_DEPLOY_SH: &str = include_str!("../templates/lib/before_deploy.sh");
static LIB_INSTALL: &str = include_str!("../templates/lib/install.sh");
static LIB_SCRIPT: &str = include_str!("../templates/lib/script.sh");
static LIB_TRAVIS: &str = include_str!("../templates/lib/travis.yml");
// static APPVEYOR: &str = include_str!("../templates/appveyor.yml");

/// GitHub template struct.
pub struct Templates {
  name: String,
  dir: PathBuf,
  token: String,
  lib: bool,
}

impl Templates {
  /// Create a new instance. Creates a `scripts/` directory if it doesn't exist
  /// already.
  #[deprecated(since = "0.5.0", note = "Use `gen_bin` or `gen_lib` instead")]
  pub fn new(dir: PathBuf, name: String, token: String) -> ::Result<Self> {
    Self::gen_bin(dir, name, token)
  }

  /// Create a new instance for binary target.
  pub fn gen_bin(dir: PathBuf, name: String, token: String) -> ::Result<Self> {
    Self::scaffold(false, dir, name, token)
  }

  /// Create a new instance for library target
  pub fn gen_lib(dir: PathBuf, name: String, token: String) -> ::Result<Self> {
    Self::scaffold(true, dir, name, token)
  }

  /// Private util function that sets up the scripts dir
  #[inline]
  fn scaffold(
    lib: bool,
    dir: PathBuf,
    name: String,
    token: String,
  ) -> ::Result<Self> {
    let scripts_dir = dir.join("scripts");
    mkdirp(&scripts_dir).context(::ErrorKind::Other)?;
    Ok(Self {
      lib,
      name,
      dir,
      token,
    })
  }

  /// Write all templates.
  pub fn write_all(&self) -> ::Result<()> {
    let scripts_dir = &self.dir.join("scripts");

    if self.lib {
      self.write(&scripts_dir, "before_deploy.ps1", LIB_DEPLOY_PS1)?;
      self.write(&scripts_dir, "before_deploy.sh", LIB_DEPLOY_SH)?;
      self.write(&scripts_dir, "install.sh", LIB_INSTALL)?;
      self.write(&scripts_dir, "script.sh", LIB_SCRIPT)?;
      self.write(&self.dir, ".travis.yml", LIB_TRAVIS)?; // TODO: set token
    } else {
      self.write(&scripts_dir, "before_deploy.ps1", BIN_DEPLOY_PS1)?;
      self.write(&scripts_dir, "before_deploy.sh", BIN_DEPLOY_SH)?;
      self.write(&scripts_dir, "install.sh", BIN_INSTALL)?;
      self.write(&scripts_dir, "script.sh", BIN_SCRIPT)?;
      self.write(&self.dir, ".travis.yml", BIN_TRAVIS)?; // TODO: set token
    }

    // self.write(&self.dir, "appveyor.yml", APPVEYOR)?;

    Ok(())
  }

  fn write(
    &self,
    issue_dir: &PathBuf,
    file_name: &str,
    template: &str,
  ) -> ::Result<()> {
    let dir = issue_dir.join(file_name);
    let mut file = File::create(dir).context(::ErrorKind::Other)?;
    let template = str::replace(&template, "{{PKG_NAME}}", &self.name);
    let template = str::replace(&template, "{{TOKEN}}", &self.token);

    file
      .write_all(template.as_bytes())
      .context(::ErrorKind::Other)?;
    Ok(())
  }
}
