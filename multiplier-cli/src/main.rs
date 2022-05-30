mod args_parse;
use args_parse::*;

use clap::Parser;
use multiplier_cli::{build_constructor, prepare_input, prove, setup, verify, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup {
            vkey: vkey_path,
            pkey: pkey_path,
        } => {
            setup(&vkey_path, &pkey_path)?;
        }
        Commands::Prove {
            pkey: pkey_path,
            witness: witness_path,
            proof: proof_path,
        } => {
            prove(&pkey_path, &witness_path, &proof_path)?;
        }
        Commands::Verify {
            public: public_path,
            vkey: vkey_path,
            proof: proof_path,
        } => {
            let res = verify(&vkey_path, &public_path, &proof_path)?;

            match res {
                true => println!("Verified Well"),
                false => println!("Bad proof"),
            }
        }
        Commands::PreparePublic { input, path } => prepare_input(&input, &path)?,
        Commands::BuildConstructor { vkey, input, path } => {
            build_constructor(&vkey, &input, &path)?;
        }
    }

    Ok(())
}
