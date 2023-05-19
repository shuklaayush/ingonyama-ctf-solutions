use ark_ff::{to_bytes, FftField};
use ark_poly::UVPolynomial;
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::{LabeledPolynomial, PolynomialCommitment, LabeledCommitment, QuerySet};
use ark_std::{rand::RngCore};
use std::{thread, time};
use crate::{
    data_structures::{Proof, Statement},
    error::Error,
    rng::FiatShamirRng,
};
pub const PROTOCOL_NAME: &'static [u8] = b"OPERATION_ZK_RESCUE";

pub fn prove<
    F: FftField,
    PC: PolynomialCommitment<F, DensePolynomial<F>>,
    FS: FiatShamirRng,
    R: RngCore,
>(
    ck: &PC::CommitterKey,
    statement: &Statement<F, PC>,
    f: &LabeledPolynomial<F, DensePolynomial<F>>,
    f_rand: &PC::Randomness,
    rng: &mut R,
) -> Result<Proof<F, PC>, Error<PC::Error>> {
println!("Begin Proof generation.. \n");
thread::sleep(time::Duration::from_secs(2));

   /*
        ADD YOUR CODE HERE, use the document for reference on univariate sumcheck. 
        Some other good sources are in Justin Thaler's book
    */
    // Initialize Fiat-Shamir RNG for non-interactive proofs
    let mut fs_rng = FS::initialize(&to_bytes![&PROTOCOL_NAME, statement].unwrap());

    // Define the masking polynomial s as the negation (additive inverse) of the polynomial f over the field F
    let s = -f.polynomial().clone();
    let s = LabeledPolynomial::new("s".into(), s, None, Some(1));

    // The masked polynomial as defined in the verifier is the sum of the polynomial f and the masking polynomial s
    let masked = f.polynomial() + s.polynomial();
    // Since s is the additive inverse of f, the masked polynomial is identically 0
    assert_eq!(masked, DensePolynomial::from_coefficients_slice(&[F::zero()]));
    // This also implies that the masked polynomial satisfies the sumcheck property i.e. its sum over the domain is 0

    // Decompose the masked polynomial into polynomials h (degree < D - N) and g (degree < N) through dividing it by the vanishing polynomial of the domain
    // Since the masked polynomial satisfies the sumcheck property, the degree of g is < N - 1
    let (h, g) = masked.divide_by_vanishing_poly(statement.domain).unwrap();
    let h = LabeledPolynomial::new("h".into(), h, None, Some(1));
    let g = LabeledPolynomial::new("g".into(), g, Some(30), Some(1));

    // In this proof, since the masked polynomial is identically 0, both h and g would also be identically 0
    assert_eq!(h.polynomial().clone(), DensePolynomial::from_coefficients_slice(&[F::zero()]));
    assert_eq!(g.polynomial().clone(), DensePolynomial::from_coefficients_slice(&[F::zero()]));

    // Generate polynomial commitments for the polynomials s, h, g
    let (commitments, rands) = PC::commit(&ck, &[s.clone(), h.clone(), g.clone()], Some(rng)).unwrap();

    let f_commitment = LabeledCommitment::new("f".into(), statement.f.clone(), None);
    let s_commitment = commitments[0].clone();
    let h_commitment = commitments[1].clone();
    let g_commitment = commitments[2].clone();

    // Progress RNG state
    fs_rng.absorb(&to_bytes![s_commitment.commitment().clone(), h_commitment.commitment().clone(), g_commitment.commitment().clone()].unwrap());

    // Generate the polynomial commitment proof and the openings
    let xi = F::rand(&mut fs_rng);
    let opening_challenge = F::rand(&mut fs_rng);

    let point_label = String::from("xi");
    let query_set = QuerySet::from([
        ("f".into(), (point_label.clone(), xi)),
        ("s".into(), (point_label.clone(), xi)),
        ("h".into(), (point_label.clone(), xi)),
        ("g".into(), (point_label.clone(), xi)),
    ]);

    let polynomials = vec![f, &s, &h, &g];
    let commitments = vec![&f_commitment, &s_commitment, &h_commitment, &g_commitment];
    let rands = vec![f_rand, &rands[0], &rands[1], &rands[2]];

    let pc_proof = PC::batch_open(
        &ck,
        polynomials,
        commitments,
        &query_set,
        opening_challenge,
        rands,
        Some(rng),
    ).unwrap();

    let f_opening = f.evaluate(&xi);
    let s_opening = s.evaluate(&xi);
    let h_opening = h.evaluate(&xi);
    let g_opening = g.evaluate(&xi);

    // Sanity checks for the openings
    assert_eq!(f_opening, -s_opening);
    assert_eq!(h_opening, F::zero());
    assert_eq!(g_opening, F::zero());

println!("End Proof generation.. \n");
thread::sleep(time::Duration::from_secs(2));

    // Return the zk proof
    Ok(Proof {
        f_opening,
        s: s_commitment.commitment().clone(),
        s_opening,
        h: h_commitment.commitment().clone(),
        h_opening,
        g: g_commitment.commitment().clone(),
        g_opening,
        pc_proof,
    })
}
