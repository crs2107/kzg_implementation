use std::{ops::Mul, time::Instant};
use ark_ff::Field;
use ark_ec::pairing::Pairing;
use crate::utils::{division,point_eval} ;

pub struct KZG<E: Pairing> {
    pub g1: E::G1,
    pub g2: E::G2,
    pub degree: usize,
    pub crs_g1: Vec<E::G1>,
    pub crs_g2: Vec<E::G2>,
}

impl <E: Pairing> KZG<E> {
    pub fn new(g1: E::G1, g2: E::G2, degree: usize) -> Self {
        Self {
            g1,
            g2,
            degree,
            crs_g1: vec![],
            crs_g2: vec![],
        }
    }
    pub fn setup(&mut self, secret: E::ScalarField) {
        for i in 0..self.degree+1 {
            self.crs_g1.push(self.g1.mul(secret.pow(&[i as u64])));
            self.crs_g2.push(self.g2.mul(secret.pow(&[i as u64])));
        }
    }
    //where polynomials are in coefficient form
    pub fn commit(&self, poly: &[E::ScalarField])-> E::G1 {
        let mut com = self.g1.mul(E::ScalarField::ZERO) ;
        for i in 0..self.degree+1 {
            com += self.crs_g1[i]*poly[i] ;
        }
        com 
    }
    pub fn open(&self, poly: &[E::ScalarField], point: E::ScalarField) -> E::G1 {
        let value = point_eval(poly, point);
        let first = poly[0] - value;
        let rest = &poly[1..];
        let temp: Vec<E::ScalarField> = std::iter::once(first).chain(rest.iter().cloned()).collect();
        let numerator: &[E::ScalarField] = &temp;
       
        let quotient = division(numerator, point);

        // calculate pi as proof (quotient multiplied by CRS)
        
        let mut pi = self.g1.mul(E::ScalarField::ZERO);
        for i in 0..quotient.len() {
            pi += self.crs_g1[i] * quotient[i];
        }

        // return pi
        pi
        
    }
    pub fn verify(&self,
        point:E::ScalarField,
        value:E::ScalarField,
        commitment:E::G1,
        pi:E::G1)-> bool {
            let left = E::pairing(pi,self.crs_g2[1]-self.g2.mul(point));
            let right = E::pairing(commitment - self.g1.mul(value),self.g2);
        

            left == right
        }
}