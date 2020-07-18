use std::fs;
use std::path;

use anyhow::anyhow;
use anyhow::Context as _;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// File to interpret.
    path: path::PathBuf,
}

fn main() -> anyhow::Result<()> {

    let opt = Opt::from_args();    
    let txt = fs::read_to_string(&opt.path)
        .with_context(|| anyhow!("Could not read file: '{}'", opt.path.display()))?;

    dbg!(txt);

    Ok(())

}
