// use std::fmt;
// use std::ops;
//
// #[derive(Debug, PartialEq)]
// pub struct Complex {
//     pub real: f64,
//     pub imaginary: f64
// }
//
// impl Complex {
//     pub fn magnitude(&self) -> f64 {
//         return (self.real.powf(2.0) + self.imaginary.powf(2.0)).sqrt();
//     }
//     pub fn add(&mut self, ket: &Complex) {
//         self.real += ket.real;
//         self.imaginary += ket.imaginary;
//     }
// }
//
// impl fmt::Display for Complex {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         return write!(f, "Complex: {} + {}i", self.real, self.imaginary);
//     }
// }
//
// impl ops::Add<Complex> for Complex {
//     type Output = Complex;
//
//     fn add(self, _rhs: Complex) -> Complex {
//         return Complex {real: self.real+_rhs.real, imaginary: self.imaginary+_rhs.imaginary};
//     }
// }
//
// impl ops::AddAssign<Complex> for Complex {
//     fn add_assign(&mut self, _rhs: Complex) {
//         self.real += _rhs.real;
//         self.imaginary = _rhs.imaginary;
//     }
// }
//
// impl Copy for Complex { }
//
// impl Clone for Complex {
//     fn clone(&self) -> Complex {
//         *self
//     }
// }
