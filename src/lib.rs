#![feature(const_trait_impl)]
#![feature(fn_traits)]
#![feature(const_fn_floating_point_arithmetic)]

moddef::moddef!(
    flat(pub) mod {
        one_const,
        zero_const
    }
);

#[cfg(test)]
mod tests
{
    use num::{Complex, rational::Ratio};

    use super::*;

    #[test]
    #[allow(unused)]
    fn it_works()
    {
        const ONE: f32 = f32::ONE;
        const ZERO: f32 = f32::ZERO;

        const ONE_COMPLEX: Complex<f32> = Complex::ONE;
        const ZERO_COMPLEX: Complex<f32> = Complex::ZERO;

        const ONE_RATIO: Ratio<u8> = Ratio::ONE;
        const ZERO_RATIO: Ratio<u8> = Ratio::ZERO;
    }
}

mod hax
{
    use std::mem::ManuallyDrop;

    use num::{rational::Ratio, Complex};

    // HACKY WAY TO TURN RATIO INTO PARTS WITHOUT COPYING, nessecary for now until Ratio::into_parts is made const
    union RatioTransmutation<T>
    {
        numer_denum: ManuallyDrop<(T, T)>,
        ratio: ManuallyDrop<Ratio<T>>
    }

    #[inline]
    pub(crate) const fn ratio_into_parts<T>(ratio: Ratio<T>) -> (T, T)
    {
        let t = RatioTransmutation
        {
            ratio: ManuallyDrop::new(ratio)
        };
        ManuallyDrop::into_inner(unsafe {t.numer_denum})
    }
    
    // HACKY WAY TO TURN COMPLEX INTO PARTS WITHOUT COPYING.
    union ComplexTransmutation<T>
    {
        re_im: ManuallyDrop<(T, T)>,
        complex: ManuallyDrop<Complex<T>>
    }

    #[inline]
    pub(crate) const fn complex_into_parts<T>(complex: Complex<T>) -> (T, T)
    {
        let t = ComplexTransmutation
        {
            complex: ManuallyDrop::new(complex)
        };
        ManuallyDrop::into_inner(unsafe {t.re_im})
    }
}