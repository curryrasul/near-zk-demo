use ark_bn254::{Bn254, Fr, FrParameters};
use ark_ff::{BigInteger256, Fp256};
use ark_groth16::{PreparedVerifyingKey, Proof, ProvingKey, VerifyingKey};
use ark_serialize::CanonicalDeserialize;
use ark_std::io::Cursor;
use std::fs;

use crate::engine::data_structures::Proof as JsonedProof;

use crate::Witness;
use crate::{Circuit, PublicInput, PvkJson};

use crate::Result;

pub(crate) fn deserialize_pk(path: &str) -> Result<ProvingKey<Bn254>> {
    let buf_pk = fs::read(path)?;
    let cursor = Cursor::new(buf_pk);

    let pkey =
        <ProvingKey<Bn254>>::deserialize_unchecked(cursor).expect("Failed pkey deserialization");

    Ok(pkey)
}

#[allow(dead_code)]
pub(crate) fn deserialize_pvk(path: &str) -> Result<PreparedVerifyingKey<Bn254>> {
    let jsoned = fs::read_to_string(path)?;

    let pvk_json: PvkJson = serde_json::from_str(&jsoned)?;

    Ok(PreparedVerifyingKey::<Bn254>::from(&pvk_json))
}

#[allow(dead_code)]
pub(crate) fn deserialize_vk(path: &str) -> Result<VerifyingKey<Bn254>> {
    let buf_vk = fs::read(path)?;
    let cursor = Cursor::new(buf_vk);

    let vk = VerifyingKey::<Bn254>::deserialize_unchecked(cursor)?;

    Ok(vk)
}

pub(crate) fn deserialize_proof(path: &str) -> Result<Proof<Bn254>> {
    let jsoned = fs::read_to_string(path)?;
    let jsoned: JsonedProof = serde_json::from_str(&jsoned)?;

    let cursor = Cursor::new(jsoned.get_proof());

    let proof =
        <Proof<Bn254>>::deserialize_unchecked(cursor).expect("Failed proof deserialization");

    Ok(proof)
}

pub(crate) fn deserialize_witness(path: &str) -> Result<Circuit> {
    let jsoned = fs::read_to_string(path)?;

    let witness: Witness = serde_json::from_str(&jsoned)?;

    Ok(Circuit::from(witness))
}

pub(crate) fn deserialize_input(path: &str) -> Result<Vec<Fp256<FrParameters>>> {
    let jsoned = fs::read_to_string(path)?;
    let input: PublicInput = serde_json::from_str(&jsoned)?;

    let mut res = vec![];

    for v in input.input {
        let cursor = Cursor::new(v);

        let deserialized = <BigInteger256>::deserialize_unchecked(cursor)?;
        let deserialized = Fr::new(deserialized);
        res.push(deserialized);
    }

    Ok(res)
}
