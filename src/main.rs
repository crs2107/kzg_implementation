pub mod kzg ;
use std::time::Instant;
pub mod utils ;
use kzg::KZG ;
use utils::point_eval ;
use ark_std::UniformRand;
use ark_bls12_381::{Bls12_381, Fr, G1Projective as G1, G2Projective as G2};
//use rand::seq::IteratorRandom;


fn main() {
    let mut rng = ark_std::test_rng();
    let degree = 16 ;
    let mut kzg_instance = KZG::<Bls12_381>::new(
        G1::rand(&mut rng),
        G2::rand(&mut rng),
        degree 
    );
    //setup ceremony
    let secret = Fr::rand(&mut rng);
    kzg_instance.setup(secret);
    //generate a random polynomial and commit to it 
    let poly = vec![Fr::rand(&mut rng); degree+1];
    let commitment = kzg_instance.commit(&poly);
    //testing 
    //generate a random point and open the polynomial at the point 
    let point = Fr::rand(&mut rng) ;
    let start = Instant::now() ;
    let pi = kzg_instance.open(&poly,point) ;
    let duration = start.elapsed() ;
    println!("prover time is {:?}",duration) ;
    //verify the proof
    let value = point_eval(&poly, point);
    let start = Instant::now() ;
    assert!(kzg_instance.verify(point,value,commitment,pi)) ;
    let duration = start.elapsed() ;
    println!("verifier time is {:?}",duration) ;
    println!("VERIFIED");

}
