use failure::ResultExt;
use mkdirp::mkdirp;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

static DEPLOY_PS1: &str = include_str!("../templates/before_deploy.ps1");
static DEPLOY_SH: &str = include_str!("../templates/before_deploy.sh");
static INSTALL: &str = include_str!("../templates/install.sh");
static SCRIPT: &str = include_str!("../templates/script.sh");
static TRAVIS: &str = include_str!("../templates/travis.yml");
static APPVEYOR: &str = include_str!("../templates/appveyor.yml");

/// GitHub template struct.
pub struct Templates {
  name: String,
  dir: PathBuf,
}

impl Templates {
  /// Create a new instance. Creates a `scripts/` directory if it doesn't exist
  /// already.
  pub fn new(mut dir: PathBuf, name: String) -> ::Result<Self> {
    let scripts_dir = dir.join("scripts");
    mkdirp(&dir).context(::ErrorKind::Other)?;
    Ok(Self { name, dir })
  }

  /// Write all templates.
  pub fn write_all(&self) -> ::Result<()> {
    let scripts_dir = &self.dir.join("scripts");
    self.write(&scripts_dir, "before_deploy.ps1", DEPLOY_PS1)?;
    self.write(&scripts_dir, "before_deploy.sh", DEPLOY_SH)?;
    self.write(&scripts_dir, "install.sh", INSTALL)?;
    self.write(&scripts_dir, "script.sh", SCRIPT)?;

    self.write(&self.dir, ".travis.yml", TRAVIS)?; // TODO: set token

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
    let template = str::replace(template, "{{PKG_NAME}}", &self.name);

    file
      .write_all(template.as_bytes())
      .context(::ErrorKind::Other)?;
    Ok(())
  }
}
