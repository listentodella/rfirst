use anyhow::Result;
use clap::Parser;
use rgrep::*;

fn main() -> Result<()> {
    let config = RgrepConfig::parse();
    config.match_with_default_strategy()?;

    Ok(())
}
