use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, PanicOnDefault};

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
    public_input: Vec<Vec<u8>>,
    vk: Vec<u8>,
}

#[near_bindgen]
impl Verifier {
    #[init]
    pub fn new(public_input: Vec<Vec<u8>>, vk: Vec<u8>) -> Self {
        assert!(!env::state_exists(), "Contract already initialized");

        log!("Contract is initialized");

        Self { public_input, vk }
    }

    pub fn verify(&self, proof: Vec<u8>) -> bool {
        let input = self.deserialize_public();
        log!("After public to Fr: {:?}", env::used_gas().0 / TGAS);

        let vk = self.deserialize_vk();
        log!("After Vk deserialization: {:?}", env::used_gas().0 / TGAS);

        let proof = Self::deserialize_proof(proof);
        log!(
            "After proof deserialization: {:?}",
            env::used_gas().0 / TGAS
        );

        let res = Groth16::<Bn254>::verify(&vk, &input, &proof).expect("Failed verifying prood");
        log!("After verification: {:?}", env::used_gas().0 / TGAS);

        res
    }

    fn deserialize_public(&self) -> Vec<Fp256<ark_bn254::FrParameters>> {
        let mut res = vec![];

        for v in &self.public_input {
            let c = Cursor::new(v);
            let b = BigInteger256::deserialize_unchecked(c).expect("Failed input deserialize");

            res.push(Fr::new(b));
        }

        res
    }

    fn deserialize_vk(&self) -> VerifyingKey<Bn254> {
        let cursor = Cursor::new(&self.vk);
        <VerifyingKey<Bn254>>::deserialize_unchecked(cursor).expect("Failed vk deserialize")
    }

    fn deserialize_proof(proof: Vec<u8>) -> Proof<Bn254> {
        let cursor = Cursor::new(proof);
        <Proof<Bn254>>::deserialize_unchecked(cursor).expect("Failed proof deserialize")
    }

    #[private]
    pub fn change_vk(&mut self, vk: Vec<u8>) {
        self.vk = vk;
    }

    #[private]
    pub fn change_public_input(&mut self, public_input: Vec<Vec<u8>>) {
        self.public_input = public_input;
    }
}
