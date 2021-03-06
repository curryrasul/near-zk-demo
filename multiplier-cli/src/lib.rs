use std::error;
use std::fs;
use std::str::FromStr;

use ark_bn254::{Bn254, Fr};
use ark_crypto_primitives::snark::*;
use ark_groth16::Groth16;

mod engine;
pub(crate) use engine::circuit::*;
pub(crate) use engine::data_structures::*;
pub(crate) use engine::deserialization::*;
pub(crate) use engine::serialization::*;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn setup(vkey_path: &str, pkey_path: &str) -> Result<()> {
    let rng = &mut ark_std::test_rng();
    let circuit = Circuit::default();

    let keys = Groth16::<Bn254>::circuit_specific_setup(circuit, rng)?;
    serialize_keys(keys, vkey_path, pkey_path)?;

    Ok(())
}

pub fn prove(pkey_path: &str, witness_path: &str, proof_path: &str) -> Result<()> {
    let rng = &mut ark_std::test_rng();

    let circuit = deserialize_witness(witness_path)?;
    let pkey = deserialize_pk(pkey_path)?;

    let proof = Groth16::prove(&pkey, circuit, rng).expect("Cannot prove");
    serialize_proof(proof, proof_path)?;

    Ok(())
}

pub fn verify(vkey_path: &str, public_path: &str, proof_path: &str) -> Result<bool> {
    let proof = deserialize_proof(proof_path)?;
    // let pvk = deserialize_pvk(vkey_path)?;
    let vk = deserialize_vk(vkey_path)?;
    let input = deserialize_input(public_path)?;

    // let res = ark_groth16::verifier::verify_proof(&pvk, &proof, &input)?;
    let res = Groth16::verify(&vk, &input, &proof)?;

    Ok(res)
}

pub fn prepare_input(input: &str, path: &str) -> Result<()> {
    let elem = Fr::from_str(input).expect("Cannot prepare public input");
    serialize_input(elem, path)?;

    Ok(())
}

pub fn build_constructor(vkey_path: &str, input_path: &str, file_path: &str) -> Result<()> {
    let jsoned_input = fs::read_to_string(input_path)?;
    let input: PublicInput = serde_json::from_str(&jsoned_input)?;

    let buf_vk = fs::read(vkey_path)?;

    let jsoned = Constructor::new(input.get_input(), buf_vk);
    let jsoned = serde_json::to_string(&jsoned)?;

    fs::write(file_path, jsoned)?;

    Ok(())
}
