use ark_bn254::{Bn254, FrParameters};
use ark_ff::Fp256;
use ark_groth16::PreparedVerifyingKey;
use ark_groth16::{Proof, ProvingKey, VerifyingKey};
use ark_serialize::CanonicalSerialize;
use std::fs;

use crate::engine::data_structures::Proof as JsonedProof;
use crate::{PublicInput, PvkJson};

use crate::Result;

pub(crate) fn serialize_keys(
    keys: (ProvingKey<Bn254>, VerifyingKey<Bn254>),
    vkey_path: &str,
    pkey_path: &str,
) -> Result<()> {
    // Pkey serialization
    let pk = keys.0;

    let mut buf_pk = vec![];
    pk.serialize_unchecked(&mut buf_pk)?;

    fs::write(pkey_path, buf_pk)?;
    println!("Pkey was written to {}", pkey_path);

    // // Vkey serialization
    // let pvk = ark_groth16::prepare_verifying_key(&keys.1);
    // serialize_pvk(pvk, vkey_path)?;
    // println!("PVkey was written to {}", vkey_path);

    serialize_vk(keys.1, vkey_path)?;
    println!("Vkey was written to {}", vkey_path);

    Ok(())
}

#[allow(dead_code)]
fn serialize_vk(vk: VerifyingKey<Bn254>, path: &str) -> Result<()> {
    let mut buf_vk = vec![];

    vk.serialize_unchecked(&mut buf_vk)?;

    fs::write(path, buf_vk)?;

    Ok(())
}

#[allow(dead_code)]
fn serialize_pvk(pvk: PreparedVerifyingKey<Bn254>, vkey_path: &str) -> Result<()> {
    let ser_pvk = PvkJson::from(&pvk);
    let jsoned = serde_json::to_string(&ser_pvk)?;

    fs::write(vkey_path, jsoned)?;

    Ok(())
}

pub(crate) fn serialize_proof(proof: Proof<Bn254>, path: &str) -> Result<()> {
    let mut buf_proof = vec![];

    proof.serialize_unchecked(&mut buf_proof)?;

    let jsoned = JsonedProof::new(buf_proof);
    let jsoned = serde_json::to_string(&jsoned)?;

    fs::write(path, jsoned)?;
    println!("Proof was written to {} file", path);

    Ok(())
}

pub(crate) fn serialize_input(elem: Fp256<FrParameters>, path: &str) -> Result<()> {
    let mut buf_input = vec![];
    elem.0.serialize_unchecked(&mut buf_input)?;

    let jsoned: PublicInput = PublicInput::new(vec![buf_input]);
    let jsoned = serde_json::to_string(&jsoned)?;

    fs::write(path, jsoned)?;

    Ok(())
}
