use std::cmp::Ordering;
use std::ops::*;
use num_traits::{*, real::Real};

trait Complex<T : Num> : Signed {
    fn conj(self) -> Self;
    fn get_angle(self) -> T;
    fn get_real(self) -> T;
    fn get_imag(self) -> T;
}

impl<T> Complex<T> for T where T: Float, T: FloatConst, T: Signed, T: Ord {
    fn conj(self) -> Self {
        self
    }

    fn get_angle(self) -> Self {
        match Ord::cmp(&self, &Self::zero()) {
            Ordering::Less => -Self::PI(),
            Ordering::Equal => Self::nan(),
            Ordering::Greater => Self::zero(),
        }
    }

    fn get_real(self) -> Self {
        self
    }

    fn get_imag(self) -> Self {
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

impl<T> Rem for ComplexRect<T> where T: Real {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        // Not sure what to do with this
        self::zero()
    }
}

impl<T> Num for ComplexRect<T> where T: Real {}

impl<T> Signed for ComplexRect<T> where T: Real {
    fn abs(self) -> Self {
        T::sqrt(self.real*self.real + self.imag*self.imag)
    }

    fn abs_sub(self, other: Self) -> Self {
        abs(self - other)
    }

    fn is_positive(self) -> bool {
        self.real.is_positive()
    }

    fn is_negative(self) -> bool {
        self.real.is_negative()
    }

    fn signum(self) -> Self {
        if self.is_zero() {
            self
        } else {
            self / abs(self)
        }
    }
}

impl<T> Complex<T> for ComplexRect<T> where T: Real {
    fn conj(self) -> Self {
        ComplexRect { real: self.real, imag: -self.imag }
    }

    fn get_angle(self) -> Self {
        Self::atan2(self.imag, self.real)
    }

    fn get_real(self) -> Self {
        self.real
    }

    fn get_imag(self) -> Self {
        self.imag
    }
}

struct ComplexPolar<T : Num> {
    abs: T,
    angle: T,
}