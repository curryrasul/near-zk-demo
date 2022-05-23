use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{Proof, ProvingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::io::Cursor;
use std::fs;
use std::io;
use std::str::FromStr;

pub fn serialize_keys(keys: (ProvingKey<Bls12_381>, VerifyingKey<Bls12_381>)) {
    let mut buf_vk = vec![];
    let mut buf_pk = vec![];

    let pk = keys.0;
    let vk = keys.1;

    pk.serialize(&mut buf_pk).unwrap();
    vk.serialize(&mut buf_vk).unwrap();

    // println!("Proving key \n{:?}", buf_pk);
    // println!("Verifying key \n{:?}", buf_vk);

    fs::write("snark/pk", buf_pk).unwrap();
    println!("Pk was written to snark/pk file");

    fs::write("snark/vk", buf_vk).unwrap();
    println!("Vk was written to snark/vk file");
}

pub fn serialize_proof(proof: Proof<Bls12_381>) {
    let mut buf_proof = vec![];

    proof.serialize(&mut buf_proof).unwrap();
    println!("Proof \n{:?}", buf_proof);

    fs::write("snark/proof", buf_proof).unwrap();
    println!("Proof was written to snark/proof file");
}

pub fn deserialize_pk() -> ProvingKey<Bls12_381> {
    let buf_pk = fs::read("snark/pk").unwrap();
    let cursor = Cursor::new(buf_pk);

    <ProvingKey<Bls12_381>>::deserialize(cursor).unwrap()
}

pub fn deserialize_vk() -> VerifyingKey<Bls12_381> {
    let buf_vk = fs::read("snark/vk").unwrap();
    let cursor = Cursor::new(buf_vk);

    <VerifyingKey<Bls12_381>>::deserialize(cursor).unwrap()
}

pub fn deserialize_proof() -> Proof<Bls12_381> {
    let buf_proof = fs::read("snark/proof").unwrap();
    let cursor = Cursor::new(buf_proof);

    <Proof<Bls12_381>>::deserialize(cursor).unwrap()
}

pub fn get_witness() -> (String, String) {
    let mut witness = String::new();

    io::stdin().read_line(&mut witness).unwrap();
    let values: Vec<_> = witness
        .split_whitespace()
        .take(2)
        .map(String::from)
        .collect();

    (values[0].clone(), values[1].clone())
}

pub fn get_public_input() -> Fr {
    let mut pub_input = String::new();
    io::stdin().read_line(&mut pub_input).unwrap();

    Fr::from_str(&pub_input[..pub_input.len() - 1]).unwrap()
}
