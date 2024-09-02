pub trait Pow<T> {
    type Output;

    fn pow(self, exponent: T) -> Self::Output;
}

macro_rules! pow_impl {
    ($t:ty, $rhs:ty, $desired_rhs:ty, $method:expr) => {
        impl Pow<$rhs> for $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                ($method)(self, <$desired_rhs>::from(rhs))
            }
        }

        impl<'a> Pow<&'a $rhs> for $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: &'a $rhs) -> $t {
                ($method)(self, <$desired_rhs>::from(*rhs))
            }
        }

        impl<'a> Pow<$rhs> for &'a $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: $rhs) -> $t {
                ($method)(*self, <$desired_rhs>::from(rhs))
            }
        }

        impl<'a, 'b> Pow<&'a $rhs> for &'b $t {
            type Output = $t;
            #[inline]
            fn pow(self, rhs: &'a $rhs) -> $t {
                ($method)(*self, <$desired_rhs>::from(*rhs))
            }
        }
    };
}

pow_impl!(u8, u8, u32, u8::pow);
pow_impl!(u8, u16, u32, u8::pow);
pow_impl!(u8, u32, u32, u8::pow);
pow_impl!(i8, u8, u32, i8::pow);
pow_impl!(i8, u16, u32, i8::pow);
pow_impl!(i8, u32, u32, i8::pow);
pow_impl!(u16, u8, u32, u16::pow);
pow_impl!(u16, u16, u32, u16::pow);
pow_impl!(u16, u32, u32, u16::pow);
pow_impl!(i16, u8, u32, i16::pow);
pow_impl!(i16, u16, u32, i16::pow);
pow_impl!(i16, u32, u32, i16::pow);
pow_impl!(u32, u8, u32, u32::pow);
pow_impl!(u32, u16, u32, u32::pow);
pow_impl!(u32, u32, u32, u32::pow);
pow_impl!(i32, u8, u32, i32::pow);
pow_impl!(i32, u16, u32, i32::pow);
pow_impl!(i32, u32, u32, i32::pow);
pow_impl!(u64, u8, u32, u64::pow);
pow_impl!(u64, u16, u32, u64::pow);
pow_impl!(u64, u32, u32, u64::pow);
pow_impl!(i64, u8, u32, i64::pow);
pow_impl!(i64, u16, u32, i64::pow);
pow_impl!(i64, u32, u32, i64::pow);

pow_impl!(u128, u8, u32, u128::pow);
pow_impl!(u128, u16, u32, u128::pow);
pow_impl!(u128, u32, u32, u128::pow);

pow_impl!(i128, u8, u32, i128::pow);
pow_impl!(i128, u16, u32, i128::pow);
pow_impl!(i128, u32, u32, i128::pow);

pow_impl!(usize, u8, u32, usize::pow);
pow_impl!(usize, u16, u32, usize::pow);
pow_impl!(usize, u32, u32, usize::pow);
pow_impl!(isize, u8, u32, isize::pow);
pow_impl!(isize, u16, u32, isize::pow);
pow_impl!(isize, u32, u32, isize::pow);

pow_impl!(f32, i8, i32, f32::powi);
pow_impl!(f32, u8, i32, f32::powi);
pow_impl!(f32, i16, i32, f32::powi);
pow_impl!(f32, u16, i32, f32::powi);
pow_impl!(f32, i32, i32, f32::powi);
pow_impl!(f64, i8, i32, f64::powi);
pow_impl!(f64, u8, i32, f64::powi);
pow_impl!(f64, i16, i32, f64::powi);
pow_impl!(f64, u16, i32, f64::powi);
pow_impl!(f64, i32, i32, f64::powi);
pow_impl!(f32, f32, f32, f32::powf);
pow_impl!(f64, f32, f64, f64::powf);
pow_impl!(f64, f64, f64, f64::powf);
