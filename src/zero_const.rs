use fn_zip::FnZip;
use num::{Zero, Complex, rational::Ratio};

use super::*;

#[const_trait]
pub trait ZeroConst: Zero
{
    const ZERO: Self;

    // Temporary function, until Zero is made a const trait
    fn is_zero2(self) -> bool;
}

// FINAL IMPLEMENTATION, when Zero is made const
/*impl<T> const ZeroConst for T
where
    T: ~const Zero
{
    const ZERO: Self = Self::zero();
}*/

macro_rules! impl_zero_const {
    ($zero:literal: $($num:ty)+) => {
        $(
            impl const ZeroConst for $num
            {
                const ZERO: Self = $zero;

                fn is_zero2(self) -> bool
                {
                    self == $zero
                }
            }
        )*
    };
}
impl_zero_const!(0:
    u8 u16 u32 u64 u128
    i8 i16 i32 i64 i128
);
impl_zero_const!(0.0:
    f32 f64
);

impl<F> const ZeroConst for Complex<F>
where
    Self: Zero,
    F: ~const ZeroConst
{
    const ZERO: Self = Complex::new(F::ZERO, F::ZERO);

    fn is_zero2(self) -> bool
    {
        let (re_is_zero, im_is_zero) = ZeroConst::is_zero2.fn_zip(ZeroConst::is_zero2)
            .call(hax::complex_into_parts(self));
        re_is_zero && im_is_zero
    }
}
impl<F> const ZeroConst for Ratio<F>
where
    Self: Zero,
    F: OneConst + ~const ZeroConst
{
    const ZERO: Self = Ratio::new_raw(F::ZERO, F::ONE);

    fn is_zero2(self) -> bool
    {
        let (numer_is_zero, denom_is_zero) = ZeroConst::is_zero2.fn_zip(ZeroConst::is_zero2)
            .call(hax::ratio_into_parts(self));
        numer_is_zero && !denom_is_zero
    }
}

// BAD IDEA
/*impl const ZeroConst for BigUint
{
    const ZERO: Self = private::biguint_zero();

    fn is_zero(self) -> bool
    {
        Zero::is_zero(&self)
    }
}
impl ZeroConst for BigInt
{
    const ZERO: Self = private::bigint_from_parts(num::bigint::Sign::NoSign, BigUint::ZERO);

    fn is_zero(self) -> bool
    {
        Zero::is_zero(&self)
    }
}*/