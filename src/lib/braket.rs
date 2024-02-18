// use std::{borrow::Borrow, fmt, io::ErrorKind, ops};
// use crate::complex::Complex;
// //pub trait Ket {
// //    fn add(&self, ket: &Box<dyn Ket>) -> Box<dyn Ket>;
// //}
// //
//
// #[derive(Debug, PartialEq)]
// pub struct BinaryKet {
//     pub a: Complex,
//     pub b: Complex
// }
// pub enum Ket {
//     BinaryKet(BinaryKet)
// }
//
//
//
// impl BinaryKet {
//     pub fn new(a: Complex, b: Complex) -> Result<Self, ErrorKind> {
//         println!("lol");
//         if a.magnitude()+b.magnitude() == 1.0 {
//             return Ok(BinaryKet {a, b});
//         } else {
//             return Err(ErrorKind::InvalidData);
//         }
//     }
//     pub fn add(&mut self, ket: &BinaryKet) {
//         self.a += ket.a;
//         self.b += ket.b;
//     }
// }
//
// impl ops::Add<BinaryKet> for BinaryKet {
//     type Output = BinaryKet;
//
//     fn add(self, _rhs: BinaryKet) -> BinaryKet {
//         return BinaryKet {a: self.a+_rhs.a, b: self.b+_rhs.b};
//     }
// }
//
// impl fmt::Display for BinaryKet {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         return write!(f, "Binary Ket: [{}\n             {}]", self.a, self.b);
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use crate::BinaryKet;
//
//     #[test]
//     fn add() {
//         let ket1 = BinaryKet {a: -4.32, b: 5.32};
//         let ket2 = BinaryKet {a: 3.21, b: -10.22};
//         assert_eq!(ket1+ket2, BinaryKet {a: -4.32+3.21, b: 5.32+-10.22});
//     }
// }
