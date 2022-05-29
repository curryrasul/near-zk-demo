use ark_bn254::{Bn254, Fr};
use ark_groth16::{Proof, ProvingKey, VerifyingKey};
use ark_serialize::CanonicalDeserialize;
use ark_std::io::Cursor;
use std::fs;

pub fn deserialize_pk() -> ProvingKey<Bn254> {
    let buf_pk = fs::read("snark/pk").unwrap();
    let cursor = Cursor::new(buf_pk);

    <ProvingKey<Bn254>>::deserialize(cursor).unwrap()
}

pub fn deserialize_vk() -> VerifyingKey<Bn254> {
    let buf_vk = fs::read("snark/vk").unwrap();
    let cursor = Cursor::new(buf_vk);

    <VerifyingKey<Bn254>>::deserialize_uncompressed(cursor).unwrap()

    // <Option<PreparedVerifyingKey<Bn254>> as FromBytes>::read(cursor).unwrap();

    // todo!()
}

pub fn deserialize_proof() -> Proof<Bn254> {
    let buf_proof = fs::read("snark/proof").unwrap();
    let cursor = Cursor::new(buf_proof);

    <Proof<Bn254>>::deserialize_uncompressed(cursor).unwrap()
}
