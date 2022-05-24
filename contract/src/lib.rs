use std::str::FromStr;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

use ark_bn254::{Bn254, Fr};
use ark_crypto_primitives::snark::SNARK;
use ark_groth16::{Groth16, Proof, VerifyingKey};
use ark_serialize::CanonicalDeserialize;
use ark_std::io::Cursor;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Verifier {
    public_input: String,
    vk: Vec<u8>,
}

#[near_bindgen]
impl Verifier {
    #[init]
    pub fn new(public_input: String, vk: Vec<u8>) -> Self {
        Self { public_input, vk }
    }

    pub fn verify(&self, proof: Vec<u8>) -> bool {
        let c = Fr::from_str(&self.public_input).unwrap();
        let vk = self.deserialize_vk();
        let proof = Self::deserialize_proof(proof);

        Groth16::<Bn254>::verify(&vk, &[c], &proof).unwrap()
    }

    fn deserialize_vk(&self) -> VerifyingKey<Bn254> {
        let cursor = Cursor::new(&self.vk);
        <VerifyingKey<Bn254>>::deserialize(cursor).unwrap()
    }

    fn deserialize_proof(proof: Vec<u8>) -> Proof<Bn254> {
        let cursor = Cursor::new(proof);
        <Proof<Bn254>>::deserialize(cursor).unwrap()
    }

    #[private]
    pub fn change_vk(&mut self, vk: Vec<u8>) {
        self.vk = vk;
    }

    #[private]
    pub fn change_public_input(&mut self, public_input: String) {
        self.public_input = public_input;
    }
}
