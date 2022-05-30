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
    #[clap(about = "Build a constructor.json for the smart-contract")]
    BuildConstructor {
        #[clap(long)]
        vkey: String,
        #[clap(long)]
        input: String,
        #[clap(long)]
        path: String,
    },
}
