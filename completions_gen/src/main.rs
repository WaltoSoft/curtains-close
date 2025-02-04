use clap::CommandFactory;
use clap_complete::{generate_to, shells};
use std::env;
use std::io::Error;

use curtains_close::options::Options;

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = Options::command();

    println!(
        "Bash completions generated: {:?}",
        generate_to(shells::Bash, &mut cmd, "curtains-close", &outdir)?
    );

    println!(
        "Zsh completions generated: {:?}",
        generate_to(shells::Zsh, &mut cmd, "curtains-close", &outdir)?
    );

    println!(
        "Fish completions generated: {:?}",
        generate_to(shells::Fish, &mut cmd, "curtains-close", &outdir)?
    );

    Ok(())
}
