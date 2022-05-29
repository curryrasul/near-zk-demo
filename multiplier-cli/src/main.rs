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
}

use multiplier_cli::{prove, setup, verify};

mod playground;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup {
            vkey: vkey_path,
            pkey: pkey_path,
        } => {
            // setup(vkey_path, pkey_path); => write to file vkey.json and pkey
        }
        Commands::Prove {
            pkey: pkey_path,
            witness: witness_path,
            proof: proof_path,
        } => {
            // prove(pkey_path, witness_path, proof_path); => write to file proof.json
        }
        Commands::Verify {
            public: public_path,
            vkey: vkey_path,
            proof: proof_path,
        } => {
            // verify(vkey_path, public_path, proof_path); => bool
        }
    }
}
