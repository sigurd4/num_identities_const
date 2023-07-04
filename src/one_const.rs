use num::{One, Complex, rational::Ratio};

use super::*;

#[const_trait]
pub trait OneConst: One
{
    const ONE: Self;
}

// FINAL IMPLEMENTATION, when One is made const
/*impl<T> const OneConst for T
where
    T: ~const One
{
    const ONE: Self = Self::one();
}*/

macro_rules! impl_one_const {
    ($one:literal: $($num:ty)+) => {
        $(
            impl const OneConst for $num
            {
                const ONE: Self = $one;
            }
        )*
    };
}
impl_one_const!(1:
    u8 u16 u32 u64 u128
    i8 i16 i32 i64 i128
);
impl_one_const!(1.0:
    f32 f64
);

impl<F> const OneConst for Complex<F>
where
    Self: One,
    F: OneConst + ZeroConst
{
    const ONE: Self = Complex::new(F::ONE, F::ZERO);
}
impl<T> const OneConst for Ratio<T>
where
    Self: One,
    T: OneConst
{
    const ONE: Self = Ratio::new_raw(T::ONE, T::ONE);
}

// BAD IDEA
/*impl const OneConst for BigInt
where
    BigUint: OneConst
{
    const ONE: Self = private::bigint_from_parts(num::bigint::Sign::Plus, BigUint::ONE);
}*/