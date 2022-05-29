use std::fs;
use std::str::FromStr;

use ark_bn254::{g1, g2, Bn254, Fq2Parameters, Fr, FrParameters, G1Affine};
use ark_ec::{bn::BnParameters, models::bn::G2Prepared};
use ark_ff::{
    fields::fp12_2over3over2::Fp12, BigInteger256, Fp256, Fp256Parameters, Fp2ParamsWrapper,
    QuadExtField,
};
use ark_groth16::{PreparedVerifyingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::io::Cursor;

#[test]
fn test() {
    let public = Fr::from_str("15").unwrap();
    println!("{:?}", public);
    let mut buf = vec![];
    public.serialize_uncompressed(&mut buf);

    println!("{}", buf.len());

    let c = Cursor::new(buf);
    let pubupd = <Fp256<FrParameters>>::deserialize_uncompressed(c).unwrap();
    println!("{:?}", pubupd);
}

#[test]
fn other_test() {
    let buf_vk = fs::read("../snark/vk").unwrap();
    let cursor = Cursor::new(buf_vk);

    let vk = <VerifyingKey<Bn254>>::deserialize_uncompressed(cursor).unwrap();

    // println!("{:?}", vk.alpha_g1);
    // println!("{:?}", vk.beta_g2);
    // println!("{:?}", vk.delta_g2);
    // println!("{:?}", vk.gamma_g2);
    // println!("{:?}", vk.gamma_abc_g1);

    // let mut buf = vec![];

    // vk.serialize_uncompressed(&mut buf);

    // println!("{}", buf.len());

    let pvk = PreparedVerifyingKey::from(vk);

    let alpha_g1_beta_g2 = pvk.alpha_g1_beta_g2;
    let delta_g2_neg_pc = pvk.delta_g2_neg_pc;
    let gamma_g2_neg_pc = pvk.gamma_g2_neg_pc;

    let mut buf = vec![];

    // alpha_g1_beta_g2.serialize_uncompressed(&mut buf).unwrap();

    // println!("{}", buf.len());

    // println!("{}", gamma_g2_neg_pc.ell_coeffs.len());

    // println!("{}", delta_g2_neg_pc.ell_coeffs[0].0.s);

    let elem = alpha_g1_beta_g2;

    println!("{:#?}", elem);

    elem.serialize_uncompressed(&mut buf).unwrap();

    let curs = Cursor::new(buf);

    // let elem_de =
    //     <ark_ff::Fp2<<ark_bn254::Parameters as BnParameters>::Fp2Params>>::deserialize_uncompressed(curs)
    //         .unwrap();

    let elem_de = 
        <ark_ff::Fp12<<ark_bn254::Parameters as BnParameters>::Fp12Params>>::deserialize_uncompressed(curs)
            .unwrap();

    println!("{:#?}", elem_de);

    // println!("{}", {
    //     delta_g2_neg_pc.ell_coeffs[49]
    //         .0
    //         .serialize_uncompressed(&mut buf)
    //         .unwrap();
    //     buf.len()
    // });

    // for elem in &delta_g2_neg_pc.ell_coeffs {
    //     let first = elem.0;
    //     // first.serialize_uncompressed(writer)
    //     elem.
    // }

    // println!("{:#?}", delta_g2_neg_pc);

    // println!("{:?}", pvk.delta_g2_neg_pc);
}
