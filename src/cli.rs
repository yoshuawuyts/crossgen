use clap_flags;
use structopt;

/// Command line parser.
#[derive(Debug, StructOpt)]
#[structopt(
  about = "",
  raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
pub struct Cli {
  #[structopt(flatten)]
  logger: clap_flags::Log,
  #[structopt(flatten)]
  verbosity: clap_flags::Verbosity,
}

impl Cli {
  /// Initialize a logger.
  #[inline]
  pub fn log(&self, name: &str) -> ::Result<()> {
    self
      .logger
      .log(self.verbosity.log_level(), name)
      .context(::ErrorKind::Log)?;
    Ok(())
  }
}
