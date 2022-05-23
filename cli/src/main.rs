// TODO
// 1. --setup -> Pk/Vk serialization
// 2. --proof --witness=w_path --pk=pk_path -> Proof serialization
// 3. --verify --pub=var --vk=vk_path --proof=proof_path -> Bool

use clap::{Arg, Command};
use color_eyre::Result;

fn main() -> Result<()> {
    let mut matches = Command::new("Multiplier demo CLI")
        .arg(
            Arg::new("setup")
                .long("setup")
                .takes_value(false)
                .help("Do a trusted setup (Pk/Vk serialization)")
        )
        .arg(
            Arg::new("prove")
                .long("prove")
                .takes_value(false)
                .help("Make a proof with specified witness and pk")
        )
        .arg(
            Arg::new("witness")
                .long("witness")
                .takes_value(true)
                .default_value("witness.json")
                .help("Witness path")
        )
        .arg(
            
        )

    Ok(())
}   
