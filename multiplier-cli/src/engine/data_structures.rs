use ark_bn254::Bn254;
use ark_ec::bn::{BnParameters, G2Prepared};
use ark_groth16::PreparedVerifyingKey;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::io::Cursor;
use serde::{Deserialize, Serialize};

/////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PvkJson {
    alpha_g1_beta_g2: Vec<u8>,
    delta_g2_neg_pc: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
    gamma_g2_neg_pc: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
}

impl From<PreparedVerifyingKey<Bn254>> for PvkJson {
    fn from(pvk: PreparedVerifyingKey<Bn254>) -> Self {
        let mut alpha_g1_beta_g2 = vec![];
        pvk.alpha_g1_beta_g2
            .serialize_uncompressed(&mut alpha_g1_beta_g2)
            .expect("Cannot serialize pvkey");

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
                .serialize_uncompressed(&mut delta_buf_first)
                .unwrap();
            pvk.delta_g2_neg_pc.ell_coeffs[i]
                .1
                .serialize_uncompressed(&mut delta_buf_second)
                .unwrap();
            pvk.delta_g2_neg_pc.ell_coeffs[i]
                .2
                .serialize_uncompressed(&mut delta_buf_third)
                .unwrap();

            pvk.gamma_g2_neg_pc.ell_coeffs[i]
                .0
                .serialize_uncompressed(&mut gamma_buf_first)
                .unwrap();
            pvk.gamma_g2_neg_pc.ell_coeffs[i]
                .1
                .serialize_uncompressed(&mut gamma_buf_second)
                .unwrap();
            pvk.gamma_g2_neg_pc.ell_coeffs[i]
                .2
                .serialize_uncompressed(&mut gamma_buf_third)
                .unwrap();

            delta_g2_neg_pc.push((delta_buf_first, delta_buf_second, delta_buf_third));
            gamma_g2_neg_pc.push((gamma_buf_first, gamma_buf_second, gamma_buf_third));
        }

        PvkJson {
            alpha_g1_beta_g2,
            delta_g2_neg_pc,
            gamma_g2_neg_pc,
        }
    }
}

impl From<PvkJson> for PreparedVerifyingKey<Bn254> {
    fn from(ser_pvk: PvkJson) -> Self {
        let mut pvk = PreparedVerifyingKey::<Bn254>::default();

        let cursor_first = Cursor::new(ser_pvk.alpha_g1_beta_g2);
        pvk.alpha_g1_beta_g2 = <ark_ff::Fp12<<ark_bn254::Parameters as BnParameters>::Fp12Params>>::deserialize_uncompressed(cursor_first)
            .expect("Failed deserialize alpha_g1_beta_g2");

        let mut delta_g2_neg_pc = G2Prepared::<ark_bn254::Parameters>::default();
        let mut gamma_g2_neg_pc = G2Prepared::<ark_bn254::Parameters>::default();

        for i in 0..ser_pvk.delta_g2_neg_pc.len() {
            let curs_1_1 = Cursor::new(&ser_pvk.delta_g2_neg_pc[i].0);
            let curs_1_2 = Cursor::new(&ser_pvk.delta_g2_neg_pc[i].1);
            let curs_1_3 = Cursor::new(&ser_pvk.delta_g2_neg_pc[i].2);

            let curs_2_1 = Cursor::new(&ser_pvk.gamma_g2_neg_pc[i].0);
            let curs_2_2 = Cursor::new(&ser_pvk.gamma_g2_neg_pc[i].1);
            let curs_2_3 = Cursor::new(&ser_pvk.gamma_g2_neg_pc[i].2);

            let delta_1 = <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_uncompressed(curs_1_1)
                .unwrap();
            let delta_2 = 
            <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_uncompressed(curs_1_2)
                .unwrap();
            let delta_3 = <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_uncompressed(curs_1_3)
                .unwrap();

            let gamma_1 = <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_uncompressed(curs_2_1)
                .unwrap();
            let gamma_2 = <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_uncompressed(curs_2_2)
                .unwrap();
            let gamma_3 = <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_uncompressed(curs_2_3)
                .unwrap();

            delta_g2_neg_pc.ell_coeffs.push((delta_1, delta_2, delta_3));
            gamma_g2_neg_pc.ell_coeffs.push((gamma_1, gamma_2, gamma_3));
        }

        pvk
    }
}

/////////////////////////////////////////////////////

// struct Witness {
//     a: u64,
//     b: u64,
// }
