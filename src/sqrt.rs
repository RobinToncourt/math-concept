pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

macro_rules! sqrt_impl {
    ($t:ty) => {
        impl Sqrt for $t {
            type Output = $t;

            #[inline]
            fn sqrt(self) -> Self::Output {
                (self as f64).sqrt() as $t
            }
        }
    };
}

sqrt_impl!(u8);
sqrt_impl!(u16);
sqrt_impl!(u32);
sqrt_impl!(u64);
sqrt_impl!(u128);

sqrt_impl!(i8);
sqrt_impl!(i16);
sqrt_impl!(i32);
sqrt_impl!(i64);
sqrt_impl!(i128);

sqrt_impl!(f32);
sqrt_impl!(f64);
