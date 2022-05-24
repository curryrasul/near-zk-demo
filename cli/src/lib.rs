use ark_bn254::{Bn254, Fr};
use ark_crypto_primitives::snark::*;
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey};
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use std::str::FromStr;

#[derive(Default, Copy, Clone)]
pub struct MultiplyCircuit {
    a: Option<Fr>,
    b: Option<Fr>,
}

impl MultiplyCircuit {
    pub fn new(a: &str, b: &str) -> Self {
        Self {
            a: Some(Fr::from_str(a).expect("The value have to be u64")),
            b: Some(Fr::from_str(b).expect("The value have to be u64")),
        }
    }

    pub fn prove(self, pk: ProvingKey<Bn254>) -> Proof<Bn254> {
        let rng = &mut ark_std::test_rng();

        Groth16::<Bn254>::prove(&pk, self, rng).unwrap()
    }

    pub fn setup(self) -> (ProvingKey<Bn254>, VerifyingKey<Bn254>) {
        let rng = &mut ark_std::test_rng();

        Groth16::<Bn254>::circuit_specific_setup(self, rng).unwrap()
    }
}

impl ConstraintSynthesizer<Fr> for MultiplyCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.new_witness_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let c = cs.new_input_variable(|| {
            let mut a = self.a.ok_or(SynthesisError::AssignmentMissing)?;
            let b = self.b.ok_or(SynthesisError::AssignmentMissing)?;

            a *= b;
            Ok(a)
        })?;

        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;

        Ok(())
    }
}

pub fn verify(vk: VerifyingKey<Bn254>, public_input: &[Fr], proof: Proof<Bn254>) -> bool {
    Groth16::<Bn254>::verify(&vk, public_input, &proof).unwrap()
}

#[test]
fn test_mult_groth16() {
    let rng = &mut ark_std::test_rng();

    // generate the setup parameters
    let (pk, vk) =
        Groth16::<Bn254>::circuit_specific_setup(MultiplyCircuit { a: None, b: None }, rng)
            .unwrap();

    let a = Fr::from_str("20").unwrap();
    let b = Fr::from_str("20").unwrap();
    let c = Fr::from_str("400").unwrap();

    // calculate the proof by passing witness variable value
    let proof = Groth16::<Bn254>::prove(
        &pk,
        MultiplyCircuit {
            a: Some(a),
            b: Some(b),
        },
        rng,
    )
    .unwrap();

    // validate the proof
    assert!(Groth16::<Bn254>::verify(&vk, &[c], &proof).unwrap());
}
