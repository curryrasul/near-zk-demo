use ark_bn254::Bn254;
use ark_groth16::PreparedVerifyingKey;
use ark_groth16::{Proof, ProvingKey, VerifyingKey};
use ark_serialize::CanonicalSerialize;
use color_eyre::Result;
use std::fs;

use super::data_structures::PvkJson;

pub fn serialize_keys(
    keys: (ProvingKey<Bn254>, VerifyingKey<Bn254>),
    vkey_path: &str,
    pkey_path: &str,
) -> Result<()> {
    // Pkey serialization
    let pk = keys.0;

    let mut buf_pk = vec![];
    pk.serialize(&mut buf_pk).unwrap();

    fs::write(pkey_path, buf_pk)?;
    println!("Pkey was written to {}", pkey_path);

    // Vkey serialization
    let pvk = ark_groth16::prepare_verifying_key(&keys.1);
    serialize_pvk(pvk, vkey_path)?;
    println!("Vkey was written to {}", vkey_path);

    Ok(())
}

fn serialize_pvk(pvk: PreparedVerifyingKey<Bn254>, vkey_path: &str) -> Result<()> {
    let ser_pvk = PvkJson::from(pvk);

    Ok(())
}

pub fn serialize_proof(proof: Proof<Bn254>) {
    let mut buf_proof = vec![];

    proof.serialize_uncompressed(&mut buf_proof).unwrap();
    println!("Proof \n{:?}", buf_proof);

    fs::write("snark/proof", buf_proof).unwrap();
    println!("Proof was written to snark/proof file");
}
