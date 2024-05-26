use ark_ff::Field;
//function for point evaluation 
pub fn point_eval<E:Field>(poly: &[E], point: E)-> E {
    let mut value = E::ZERO;

    for i in 0..poly.len() {
        value += poly[i]*point.pow(&[i as u64]) ;
    }
    value
}
//function to divide f(x) by x-r for some constant r 
pub fn division<E:Field>(poly: &[E],value: E) -> Vec<E> {
    let mut quotient = vec![E::ZERO;poly.len()-1] ;
    quotient[poly.len()-2] = poly[poly.len()-1] ;
   
    (0..=poly.len()-3).rev().for_each(|i| {
        quotient[i]=(quotient[i+1]*value)+poly[i+1] ;
    });
    quotient
}
/*pub fn division<E:Field>(p1: &[E], p2: &[E]) -> Result<Vec<E>, &'static str> {
    if p2.is_empty() || p2.iter().all(|&x| x == E::ZERO) {
        return Err("Cannot divide by zero polynomial");
    }

    if p1.len() < p2.len() {
        return Ok(vec![E::ZERO]);
    }

    let mut quotient = vec![E::ZERO; p1.len() - p2.len() + 1];
    let mut remainder: Vec<E> = p1.to_vec();

    while remainder.len() >= p2.len() {
        let coeff = *remainder.last().unwrap() / *p2.last().unwrap();
        let pos = remainder.len() - p2.len();

        quotient[pos] = coeff;

        for (i, &factor) in p2.iter().enumerate() {
            remainder[pos + i] -= factor * coeff;
        }

        while let Some(true) = remainder.last().map(|x| *x == E::ZERO) {
            remainder.pop();
        }
    }

    Ok(quotient)
}*/
