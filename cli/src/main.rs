mod utils;
use utils::{
    deserialize_pk, deserialize_proof, deserialize_vk, get_public_input, get_witness,
    serialize_keys, serialize_proof,
};

use clap::{Arg, Command};
use cli::{verify, MultiplyCircuit};
use color_eyre::Result;

fn main() -> Result<()> {
    let matches = Command::new("Multiplier demo CLI")
        .arg(
            Arg::new("setup")
                .long("setup")
                .takes_value(false)
                .help("Do a trusted setup (Pk/Vk serialization)"),
        )
        .arg(
            Arg::new("prove")
                .long("prove")
                .takes_value(false)
                .help("Make a proof"),
        )
        .arg(
            Arg::new("verify")
                .long("verify")
                .takes_value(false)
                .help("Verify a proof"),
        )
        .get_matches();

    if matches.is_present("setup") {
        let circuit = MultiplyCircuit::default();
        let keys = circuit.setup();

        serialize_keys(keys);
    } else if matches.is_present("prove") {
        let (a, b) = get_witness();
        let pk = deserialize_pk();
        let circuit = MultiplyCircuit::new(&a, &b);

        let proof = circuit.prove(pk);

        serialize_proof(proof);
    } else if matches.is_present("verify") {
        let c = get_public_input();
        let vk = deserialize_vk();
        let proof = deserialize_proof();

        if verify(vk, &[c], proof) {
            println!("Verified well");
        } else {
            println!("Bad proof");
        }
    } else {
        panic!("No such arguments");
    }

    Ok(())
}
