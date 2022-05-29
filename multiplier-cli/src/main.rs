use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "Multiplier zkSNARK CLI")]
#[clap(version = "1.0")]
#[clap(about = "zkSNARK prover/verifier for knowledge of private factors of public C")]
#[clap(author = "Magamedrasul I. <curryrasul@gmail.com>")]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    #[clap(about = "Do a trustless setup (Pkey / Vkey)")]
    Setup {
        #[clap(long)]
        vkey: String,
        #[clap(long)]
        pkey: String,
    },
    #[clap(about = "Create a proof with specified Pkey and witness")]
    Prove {
        #[clap(long)]
        pkey: String,
        #[clap(long)]
        witness: String,
        #[clap(long)]
        proof: String,
    },
    #[clap(about = "Verify a proof with specified public parameters, Vkey and proof")]
    Verify {
        #[clap(long)]
        public: String,
        #[clap(long)]
        vkey: String,
        #[clap(long)]
        proof: String,
    },
    #[clap(about = "Prepare public inputs to specified path")]
    PreparePublic {
        #[clap(long)]
        input: String,
        #[clap(long)]
        path: String,
    },
}

use multiplier_cli::{prepare_input, prove, setup, verify, Result};

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
    }

    Ok(())
}
