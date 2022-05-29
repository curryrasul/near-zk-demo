use std::str::FromStr;

use ark_bn254::{Bn254, Fr};
use ark_ec::bn::BnParameters;
use ark_groth16::{PreparedVerifyingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::io::Cursor;
use serde::{Deserialize, Serialize};

use super::circuit::Circuit;

/////////////////////////////////////////////////////

// PreparedVerificationKey Ser-De, Don't change it.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PvkJson {
    vkey: Vec<u8>,
    alpha_g1_beta_g2: Vec<u8>,
    delta_g2_neg_pc: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
    gamma_g2_neg_pc: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
}

impl From<&PreparedVerifyingKey<Bn254>> for PvkJson {
    fn from(pvk: &PreparedVerifyingKey<Bn254>) -> Self {
        const MSG: &str = "Failed pvk serialization";

        let mut vkey = vec![];
        pvk.vk.serialize_unchecked(&mut vkey).expect(MSG);

        let mut alpha_g1_beta_g2 = vec![];
        pvk.alpha_g1_beta_g2
            .serialize_unchecked(&mut alpha_g1_beta_g2)
            .expect(MSG);

        assert_eq!(
            pvk.delta_g2_neg_pc.ell_coeffs.len(),
            pvk.gamma_g2_neg_pc.ell_coeffs.len()
        );

        let arr_len = pvk.delta_g2_neg_pc.ell_coeffs.len();

        let mut delta_g2_neg_pc = vec![];
        let mut gamma_g2_neg_pc = vec![];

        for i in 0..arr_len {
            let mut delta_buf_first = vec![];
            let mut delta_buf_second = vec![];
            let mut delta_buf_third = vec![];

            let mut gamma_buf_first = vec![];
            let mut gamma_buf_second = vec![];
            let mut gamma_buf_third = vec![];

            pvk.delta_g2_neg_pc.ell_coeffs[i]
                .0
                .serialize_unchecked(&mut delta_buf_first)
                .expect(MSG);
            pvk.delta_g2_neg_pc.ell_coeffs[i]
                .1
                .serialize_unchecked(&mut delta_buf_second)
                .expect(MSG);
            pvk.delta_g2_neg_pc.ell_coeffs[i]
                .2
                .serialize_unchecked(&mut delta_buf_third)
                .expect(MSG);

            pvk.gamma_g2_neg_pc.ell_coeffs[i]
                .0
                .serialize_unchecked(&mut gamma_buf_first)
                .expect(MSG);
            pvk.gamma_g2_neg_pc.ell_coeffs[i]
                .1
                .serialize_unchecked(&mut gamma_buf_second)
                .expect(MSG);
            pvk.gamma_g2_neg_pc.ell_coeffs[i]
                .2
                .serialize_unchecked(&mut gamma_buf_third)
                .expect(MSG);

            delta_g2_neg_pc.push((delta_buf_first, delta_buf_second, delta_buf_third));
            gamma_g2_neg_pc.push((gamma_buf_first, gamma_buf_second, gamma_buf_third));
        }

        PvkJson {
            vkey,
            alpha_g1_beta_g2,
            delta_g2_neg_pc,
            gamma_g2_neg_pc,
        }
    }
}

impl From<&PvkJson> for PreparedVerifyingKey<Bn254> {
    fn from(ser_pvk: &PvkJson) -> Self {
        const MSG: &str = "Failed pvk deserialization";

        let mut pvk = PreparedVerifyingKey::<Bn254>::default();

        let cursor_first = Cursor::new(&ser_pvk.vkey);
        pvk.vk = <VerifyingKey<Bn254>>::deserialize_unchecked(cursor_first).expect(MSG);

        let cursor_second = Cursor::new(&ser_pvk.alpha_g1_beta_g2);
        pvk.alpha_g1_beta_g2 = <ark_ff::Fp12<<ark_bn254::Parameters as BnParameters>::Fp12Params>>::deserialize_unchecked(cursor_second)
            .expect(MSG);

        assert_eq!(
            pvk.delta_g2_neg_pc.ell_coeffs.len(),
            pvk.gamma_g2_neg_pc.ell_coeffs.len()
        );

        let arr_len = pvk.delta_g2_neg_pc.ell_coeffs.len();

        for i in 0..arr_len {
            let curs_1_1 = Cursor::new(&ser_pvk.delta_g2_neg_pc[i].0);
            let curs_1_2 = Cursor::new(&ser_pvk.delta_g2_neg_pc[i].1);
            let curs_1_3 = Cursor::new(&ser_pvk.delta_g2_neg_pc[i].2);

            let curs_2_1 = Cursor::new(&ser_pvk.gamma_g2_neg_pc[i].0);
            let curs_2_2 = Cursor::new(&ser_pvk.gamma_g2_neg_pc[i].1);
            let curs_2_3 = Cursor::new(&ser_pvk.gamma_g2_neg_pc[i].2);

            let delta_1 = <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_unchecked(curs_1_1)
                .expect(MSG);
            let delta_2 = <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_unchecked(curs_1_2)
                .expect(MSG);
            let delta_3 = <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_unchecked(curs_1_3)
                .expect(MSG);

            let gamma_1 = <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_unchecked(curs_2_1)
                .expect(MSG);
            let gamma_2 = <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_unchecked(curs_2_2)
                .expect(MSG);
            let gamma_3 = <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_unchecked(curs_2_3)
                .expect(MSG);

            pvk.delta_g2_neg_pc.ell_coeffs[i] = (delta_1, delta_2, delta_3);
            pvk.gamma_g2_neg_pc.ell_coeffs[i] = (gamma_1, gamma_2, gamma_3);
        }

        pvk
    }
}

/////////////////////////////////////////////////////

// Specific for a project
#[derive(Deserialize, Debug)]
pub struct Witness {
    a: String,
    b: String,
}

impl From<Witness> for Circuit {
    fn from(ser_witness: Witness) -> Self {
        Self {
            a: Some(Fr::from_str(&ser_witness.a).expect("Cannot deserialize witness")),
            b: Some(Fr::from_str(&ser_witness.b).expect("Cannot deserialize witness")),
        }
    }
}

/////////////////////////////////////////////////////

// Specific for a project
#[derive(Serialize, Deserialize, Debug)]
pub struct PublicInput {
    pub(crate) input: Vec<Vec<u8>>,
}

impl PublicInput {
    pub fn new(input: Vec<Vec<u8>>) -> Self {
        Self { input }
    }
}

#[cfg(test)]
mod tests {
    use ark_bn254::Bn254;
    use ark_crypto_primitives::SNARK;
    use ark_groth16::{Groth16, PreparedVerifyingKey};

    use super::PvkJson;
    use crate::engine::circuit::Circuit;

    #[test]
    fn test_pvk() {
        let circuit = Circuit::default();
        let rng = &mut ark_std::test_rng();
        let (_, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, rng).unwrap();

        let pvk = PreparedVerifyingKey::from(vk);

        let pvk_json = PvkJson::from(&pvk);
        let pvk_restored = PreparedVerifyingKey::<Bn254>::from(&pvk_json);

        assert_eq!(pvk, pvk_restored);
    }
}
