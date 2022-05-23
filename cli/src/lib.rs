use std::{ops::MulAssign, str::FromStr};

use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::snark::*;
use ark_ff::Field;
use ark_groth16::Groth16;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::UniformRand;

#[derive(Copy, Clone)]
pub struct MultiplyDemoCircuit {
    a: Option<Fr>,
    b: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for MultiplyDemoCircuit {
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

#[test]
fn test_mult_groth16() {
    let rng = &mut ark_std::test_rng();

    // generate the setup parameters
    let (pk, vk) =
        Groth16::<Bls12_381>::circuit_specific_setup(MultiplyDemoCircuit { a: None, b: None }, rng)
            .unwrap();

    let a = Fr::from_str("20").unwrap();
    let b = Fr::from_str("20").unwrap();
    let c = Fr::from_str("400").unwrap();

    // calculate the proof by passing witness variable value
    let proof = Groth16::<Bls12_381>::prove(
        &pk,
        MultiplyDemoCircuit {
            a: Some(a),
            b: Some(b),
        },
        rng,
    )
    .unwrap();

    // validate the proof
    assert!(Groth16::<Bls12_381>::verify(&vk, &[c], &proof).unwrap());
}
