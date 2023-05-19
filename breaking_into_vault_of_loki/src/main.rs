use lambdaworks_crypto::commitments::{
    kzg::{KateZaveruchaGoldberg, StructuredReferenceString},
    traits::IsCommitmentScheme,
};
use lambdaworks_math::{
    elliptic_curve::{
        short_weierstrass::{
            curves::bls12_381::{
                curve::BLS12381Curve,
                default_types::{FrConfig, FrElement},
                field_extension::BLS12381PrimeField,
                pairing::BLS12381AtePairing,
                twist::BLS12381TwistCurve,
            },
            point::ShortWeierstrassProjectivePoint,
        },
        traits::{IsEllipticCurve, FromAffine},
    },
    field::{
        element::FieldElement, fields::montgomery_backed_prime_fields::MontgomeryBackendPrimeField,
    },
    polynomial::Polynomial,
    unsigned_integer::element::UnsignedInteger, cyclic_group::IsGroup,
};

type G1Point = ShortWeierstrassProjectivePoint<BLS12381Curve>;
type G2Point = ShortWeierstrassProjectivePoint<BLS12381TwistCurve>;

type KZG = KateZaveruchaGoldberg<MontgomeryBackendPrimeField<FrConfig, 4>, BLS12381AtePairing>;
pub type Fq = FieldElement<BLS12381PrimeField>;

fn challenge_polynomial() -> Polynomial<FrElement> {
    Polynomial::<FrElement>::new(&[
        FieldElement::from(69),
        FieldElement::from(78),
        FieldElement::from(32),
        FieldElement::from(65),
        FieldElement::from(82),
        FieldElement::from(71),
        FieldElement::from(69),
        FieldElement::from(78),
        FieldElement::from(84),
        FieldElement::from(73),
        FieldElement::from(78),
        FieldElement::from(65),
        FieldElement::from(32),
        FieldElement::from(78),
        FieldElement::from(65),
        FieldElement::from(67),
        FieldElement::from(73),
        FieldElement::from(32),
        FieldElement::from(84),
        FieldElement::from(73),
        FieldElement::from(69),
        FieldElement::from(82),
        FieldElement::from(65),
    ])
}

fn decode(num: &FrElement) -> String {
    let binding = num.to_string();
    let eval_sum_str = binding.strip_prefix("0x").unwrap().to_string();
    (0..eval_sum_str.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&eval_sum_str[i..i + 2], 16).unwrap_or_default() as char)
            .collect()
}

fn print_g1(point: &G1Point) {
    println!("x: {}", point.to_affine().x().to_string());
    println!("y: {}", point.to_affine().y().to_string());
}

fn main() {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    let srs_path = base_dir.to_owned() + "/srs.bin";
    let srs = StructuredReferenceString::<G1Point, G2Point>::from_file(&srs_path).unwrap();

    let kzg = KZG::new(srs.clone());

    let p = challenge_polynomial();

    let coeff_str = p.coefficients().into_iter().map(|x| decode(x)).collect::<String>();
    println!("{}", coeff_str);

    let g = BLS12381Curve::generator();
    for i in 0..1024 {
        if g == srs.powers_main_group[i] {
            println!("i: {}", i);
        }
    }

    let p_commitment: G1Point = kzg.commit(&p);
    print_g1(&p_commitment);

    let x = FrElement::from(1);
    let y = FrElement::from(3);

    // TO DO: Make your own fake proof
    // let fake_proof =
    //     ShortWeierstrassProjectivePoint::<BLS12381Curve>::from_affine(Fq::from(0), y).unwrap();
    // let fake_proof =
    //     ShortWeierstrassProjectivePoint::<BLS12381Curve>::neutral_element();

    let fake_proof = ShortWeierstrassProjectivePoint::<BLS12381Curve>::from_affine(
        Fq::from_hex("10ae7a6f7e98fe0c57cdbc62c5172f8d647ff90e2cd8032ac5f5370b79d7a1950f07d206346b3f57af07a22c8929e5d2"),
        Fq::from_hex("1014256cd4a805263d027c682bf1e5dafcbe2e347e11e33735287fb4aafcebd3e255904f428f2faa847388d3e40badc6")
    ).unwrap();

    println!("Fake proof for submission:");
    println!("{:?}", &fake_proof.to_affine().x().to_string());
    println!("{:?}", &fake_proof.to_affine().y().to_string());

    assert!(kzg.verify(
        &x,
        &y,
        &p_commitment,
        &fake_proof
    ));
}
