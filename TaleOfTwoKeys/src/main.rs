use std::fs::File;

use ark_bls12_377::{Bls12_377, Fr};
use ark_crypto_primitives::snark::SNARK;
use ark_ec::pairing::Pairing;
use ark_ff::Field;
use ark_groth16::{ProvingKey, PreparedVerifyingKey, Groth16, Proof};
use ark_relations::{r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable}, lc};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use base64::encode;

/// This is our demo circuit for proving knowledge of the
/// x, such that x * x = square_target * y, where y = 1 all the time
/// The square target is a public input and is a part of setup
struct QRDemo<F: Field> {
    x: Option<F>,
    y: Option<F>,
    square_target: F,
}

impl<F: Field> ConstraintSynthesizer<F> for QRDemo<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let x =
            cs.new_witness_variable(|| self.x.ok_or(SynthesisError::AssignmentMissing))?;

        let y =
            cs.new_witness_variable(|| self.y.ok_or(SynthesisError::AssignmentMissing))?;

        cs.enforce_constraint(
            lc!() + x,
            lc!() + x,
            lc!() + (self.square_target, y),
        )?;

        cs.enforce_constraint(
            lc!() + Variable::One,
            lc!() + Variable::One,
            lc!() + y,
        )?;

        Ok(())
    }
}

struct SR <E: Pairing, F: Field + std::convert::From<i32>> {
    pk: ProvingKey<E>,
    pvk: PreparedVerifyingKey<E>,
    square_target: F
}

impl<E: Pairing, F: Field + std::convert::From<i32>> SR<E, F>
    where QRDemo<F>: ConstraintSynthesizer<<E as Pairing>::ScalarField> {
    
    fn new(pk_path: &str, pvk_path: &str, square_target: F) -> Self {
        let pk_bytes = File::open(pk_path).expect("Failed to open file");
        let pvk_bytes = File::open(pvk_path).expect("Failed to open file");

        let pk = ProvingKey::<E>::deserialize_with_mode(pk_bytes, ark_serialize::Compress::Yes, ark_serialize::Validate::Yes).unwrap();
        let pvk = PreparedVerifyingKey::<E>::deserialize_with_mode(pvk_bytes, ark_serialize::Compress::Yes, ark_serialize::Validate::Yes).unwrap();
        
        Self { pk, pvk, square_target }
    }

    pub fn from_sr15() -> Self {
        SR::new("pk_15.bin", "pvk_15.bin", F::from(15))
    }

    pub fn from_sr17() -> Self {
        SR::new("pk_17.bin", "pvk_17.bin", F::from(17))
    }

    pub fn create_proof(&self, x: F, y: i32) -> Result<Proof<E>, SynthesisError> {
        let circuit = QRDemo::<F> {
            x: Some(x),
            y: Some(F::from(y)),
            square_target: self.square_target
        };

        Groth16::<E>::create_proof_with_reduction_no_zk(circuit, &self.pk)
    }

    pub fn verify_proof(&self, proof: &Proof<E>) -> Result<bool, SynthesisError> {
        Groth16::<E>::verify_with_processed_vk(&self.pvk, &[], proof)
    }

}

fn proof_to_base64<E: Pairing>(proof: &Proof<E>) -> String {
    let mut buff = vec![];
    proof.serialize_compressed(&mut buff).unwrap();
    encode(&buff)
}

fn main() {
    let sr15 = SR::<Bls12_377, Fr>::from_sr15();
    let sr17 = SR::<Bls12_377, Fr>::from_sr17();

    let proof15 = sr15.create_proof(Fr::from(15).sqrt().unwrap(), 1).unwrap();
    let proof17 = sr17.create_proof(Fr::from(4), 1).unwrap();

    // Either of these proof, for any choice of x and y should not work
    assert!(
        sr15.verify_proof(&proof15).unwrap() || sr17.verify_proof(&proof17).unwrap()
    );

    println!("SR15: {}", proof_to_base64(&proof15));
    println!("SR17: {}", proof_to_base64(&proof15));
}
