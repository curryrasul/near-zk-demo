use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, PanicOnDefault};
use std::str::FromStr;

use ark_bn254::{Bn254, Fr};
use ark_crypto_primitives::snark::SNARK;
use ark_ff::{BigInteger256, Fp256};
use ark_groth16::{Groth16, Proof, VerifyingKey};
use ark_serialize::CanonicalDeserialize;
use ark_std::io::Cursor;

const TGAS: u64 = 1_000_000_000_000;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Verifier {
    public_input: Vec<u8>,
    vk: Vec<u8>,
}

#[near_bindgen]
impl Verifier {
    #[init]
    pub fn new(public_input: Vec<u8>, vk: Vec<u8>) -> Self {
        Self { public_input, vk }
    }

    pub fn verify(&self, proof: Vec<u8>) -> bool {
        let c = Fr::from_str(&String::from("50")).unwrap();
        log!("After public to Fr: {:?}", env::used_gas().0 / TGAS);

        let vk = self.deserialize_vk();
        log!("After Vk deserialization: {:?}", env::used_gas().0 / TGAS);

        let proof = Self::deserialize_proof(proof);
        log!(
            "After proof deserialization: {:?}",
            env::used_gas().0 / TGAS
        );

        let res = Groth16::<Bn254>::verify(&vk, &[c], &proof).unwrap();
        log!("After verification: {:?}", env::used_gas().0 / TGAS);

        true
    }

    fn deserialize_public(&self) -> Fp256<ark_bn254::FrParameters> {
        let c = Cursor::new(&self.public_input);
        let b = BigInteger256::deserialize_uncompressed(c).unwrap();

        Fr::new(b)
    }

    fn deserialize_vk(&self) -> VerifyingKey<Bn254> {
        let cursor = Cursor::new(&self.vk);
        <VerifyingKey<Bn254>>::deserialize_uncompressed(cursor).unwrap()
    }

    fn deserialize_proof(proof: Vec<u8>) -> Proof<Bn254> {
        let cursor = Cursor::new(proof);
        <Proof<Bn254>>::deserialize_uncompressed(cursor).unwrap()
    }

    #[private]
    pub fn change_vk(&mut self, vk: Vec<u8>) {
        self.vk = vk;
    }

    #[private]
    pub fn change_public_input(&mut self, public_input: Vec<u8>) {
        self.public_input = public_input;
    }
}
