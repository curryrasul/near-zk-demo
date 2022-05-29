use ark_bn254::Bn254;
use ark_crypto_primitives::snark::*;
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey};
use color_eyre::Result;

pub(crate) mod data_structures;

mod circuit;
use circuit::Circuit;

mod deserialization;
use deserialization::{deserialize_pk, deserialize_proof, deserialize_vk};

mod serialization;
use serialization::{serialize_keys, serialize_proof};

pub fn setup(vkey_path: &str, pkey_path: &str) -> Result<()> {
    let rng = &mut ark_std::test_rng();
    let circuit = Circuit::default();

    let keys = Groth16::<Bn254>::circuit_specific_setup(circuit, rng)?;
    serialize_keys(keys, vkey_path, pkey_path)?;

    Ok(())
}

pub fn prove(pkey_path: &str, witness_path: &str, proof_path: &str) -> Result<()> {
    let rng = &mut ark_std::test_rng();

    Ok(())
}

pub fn verify(vkey_path: &str, public_params: &str, proof_path: &str) -> Result<()> {
    todo!()
}
