<<<<<<< HEAD
use std::ops::*;
use num_traits::{*, real::Real};

trait Complex<T : Num> {
    fn conj(self) -> Self;
    fn abs(self) -> T;
    fn arg(self) -> T;
    fn re(self) -> T;
    fn im(self) -> T;
    fn signum(self) -> Self;
}

impl<T> Complex<T> for T where T: Float, T: FloatConst, T: Signed {
    fn abs(self) -> T {
        self.abs()
    }

    fn arg(self) -> Self {
        match self.signum() {
            s if s == -Self::one() => -Self::PI(),
            s if s == Self::zero() => Self::nan(),
            s if s == Self::one() => Self::zero(),
            _ => panic!(),
        }
    }

    fn signum(self) -> Self {
        self.signum()
    }

    fn re(self) -> Self {
        self
    }

    fn im(self) -> Self {
        Self::zero()
    }
}

struct ComplexRect<T : Num> {
    real: T,
    imag: T,
}

impl<T> PartialEq for ComplexRect<T> where T: Real {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}

impl<T> Zero for ComplexRect<T> where T: Real {
    fn zero() -> Self {
        ComplexRect { real: T::zero(), imag: T::zero() }
    }

    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imag.is_zero()
    }
}

impl<T> One for ComplexRect<T> where T: Real {
    fn one() -> Self {
        ComplexRect { real: T::one(), imag: T::zero() }
    }

    fn is_one(&self) -> bool {
        self.real.is_one() && self.imag.is_zero()
    }
}

impl<T> Neg for ComplexRect<T> where T: Real {
    type Output = Self;

    fn neg(self) -> Self {
        ComplexRect { real: -self.real, imag: -self.imag }
    }
}

impl<T> Add for ComplexRect<T> where T: Real {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ComplexRect { real: self.real + other.real, imag: self.imag + other.imag }
    }
}

impl<T> Sub for ComplexRect<T> where T: Real {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        ComplexRect { real: self.real - other.real, imag: self.imag - other.imag }
    }
}

impl<T> Mul for ComplexRect<T> where T: Real {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        ComplexRect {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.imag * other.real + other.imag * self.real
        }
    }
}

impl<T> Div for ComplexRect<T> where T: Real {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        // http://mathworld.wolfram.com/ComplexDivision.html
        let denom = other.real * other.real + other.imag * other.imag;
        ComplexRect {
            real: (self.real * other.real + self.imag * other.imag) / denom,
            imag: (self.imag * other.real + self.real * other.imag) / denom
        }
    }
}

enum ComplexRectError<T : Num> {
    Empty,
    RepeatedTerm,
    MissingOperator,
    RadixTooLarge,
    InnerErr(T::FromStrRadixErr),
}

fn _parse_real<T : Num + Neg<Output = T>>(s: &str, radix: u32) -> Result<T, T::FromStrRadixErr> {
    if s.is_empty() {
        Ok(T::zero())
    } else {
        T::from_str_radix(s, radix)
    }
}

fn _parse_imag<T : Num + Neg<Output = T>>(s: &str, radix: u32) -> Result<T, T::FromStrRadixErr> {
    if s.is_empty() {
        Ok(T::one())
    } else if s == "-" {
        Ok(-T::one())
    } else {
        T::from_str_radix(s, radix)
    }
}

impl<T> ComplexRect<T> where T : Real {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ComplexRectError<T>> {
        if radix > 18 {
            Err(ComplexRectError::RadixTooLarge)
        } else if s.is_empty() {
            Err(ComplexRectError::Empty)
        } else {
            let (real_res, imag_res) = match s.find('i') {
                None =>
                    (T::from_str_radix(s, radix), Ok(T::zero())),
                Some(i) =>
                    if i == s.len() - 1 {
                        match s.rfind(&['+', '-'][..]) {
                            None | Some(0) => (Ok(T::zero()), _parse_imag(&s[..i], radix)),
                            Some(j) => (_parse_real(&s[..j], radix), _parse_imag(&s[j..i], radix)),
                        }
                    } else {
                        (_parse_real(&s[i+1..], radix), _parse_imag(&s[..i], radix))
                    }
            };
            match real_res {
                Ok(real) => match imag_res {
                    Ok(imag) => Ok(ComplexRect { real, imag }),
                    Err(err) => Err(ComplexRectError::InnerErr(err)),
                }
                Err(err) => Err(ComplexRectError::InnerErr(err)),
            }
        }
    }
}

impl<T> Complex<T> for ComplexRect<T> where T: Real {
    fn conj(self) -> Self {
        ComplexRect { real: self.real, imag: -self.imag }
    }

    fn abs(self) -> T {
        T::sqrt(self.real*self.real + self.imag*self.imag)
    }

    fn arg(self) -> T {
        T::atan2(self.imag, self.real)
    }

    fn re(self) -> T {
        self.real
    }

    fn im(self) -> T {
        self.imag
    }

    fn signum(self) -> Self {
        if self.is_zero() {
            self
        } else {
            self / self.abs()
        }
    }
}

struct ComplexPolar<T : Num> {
    abs: T,
    angle: T,
}