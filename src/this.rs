//better implementation
use std::cmp::Ordering;
use std::{ops::*, process::Output};
#[derive(Clone, Copy, Debug)]
pub struct Complex<T> {
    real: T,
    imaginary: T,
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
    // By writing where T: Add<Output=T>, we restrict T to types that can be added to
    // themselves, yielding another T value.
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Complex {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    } // add code here
}

impl<T> Mul for Complex<T>
where
    T: Mul<Output = T> + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + Clone,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let temp_self = self.clone();
        let temp_other = other.clone();

        let left =
            (temp_self.real * temp_other.real) - (temp_self.imaginary * temp_other.imaginary);
        let right = (self.real * other.imaginary) + (self.imaginary * other.real);
        Complex {
            real: left,
            imaginary: right,
        }
    } // add code here
}

impl<T, O> Neg for Complex<T>
where
    T: Neg<Output = O>,
{
    type Output = Complex<O>;
    fn neg(self) -> Complex<O> {
        Complex {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

// to-do next day 2022-07-27
impl<T> PartialOrd for Complex<T>
where
    T: PartialOrd,
{
    fn le(&self, other: &Self) -> bool {
        return self.real < other.real;
    }
    fn gt(&self, other: &Self) -> bool {
        return self.real > other.real;
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.real >= other.real {
            Some(Ordering::Less)
        } else if self.real <= other.real {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        return (self.real == other.real) && (self.imaginary == other.imaginary);
    }
    fn ne(&self, other: &Self) -> bool {
        return self.eq(other);
    }
}
