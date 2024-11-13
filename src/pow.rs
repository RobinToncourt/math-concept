pub trait Pow<T> {
    type Output;

    fn pow(self, exponent: T) -> Self::Output;
}

macro_rules! pow_impl {
    ($t:ty, $desired_rhs:ty, $rhs:ty, $method:expr) => {
        impl Pow<$desired_rhs> for $t {
            type Output = $t;

            #[inline]

            fn pow(self, rhs: $desired_rhs) -> $t {
                ($method)(self, rhs as $rhs)
            }
        }

        impl<'a> Pow<&'a $desired_rhs> for $t {
            type Output = $t;

            #[inline]

            fn pow(self, rhs: &'a $desired_rhs) -> $t {
                ($method)(self, *rhs as $rhs)
            }
        }

        impl<'a> Pow<$desired_rhs> for &'a $t {
            type Output = $t;

            #[inline]

            fn pow(self, rhs: $desired_rhs) -> $t {
                ($method)(*self, rhs as $rhs)
            }
        }

        impl<'a, 'b> Pow<&'a $desired_rhs> for &'b $t {
            type Output = $t;

            #[inline]

            fn pow(self, rhs: &'a $desired_rhs) -> $t {
                ($method)(*self, *rhs as $rhs)
            }
        }
    };

    ($t:ty, $new_type:ty, $desired_rhs:ty, $rhs:ty, $method:expr) => {
        impl Pow<$desired_rhs> for $t {
            type Output = $t;

            #[inline]

            fn pow(self, rhs: $desired_rhs) -> $t {
                ($method)(self as $new_type, rhs as $rhs) as $t
            }
        }

        impl<'a> Pow<&'a $desired_rhs> for $t {
            type Output = $t;

            #[inline]

            fn pow(self, rhs: &'a $desired_rhs) -> $t {
                ($method)(self as $new_type, *rhs as $rhs) as $t
            }
        }

        impl<'a> Pow<$desired_rhs> for &'a $t {
            type Output = $t;

            #[inline]

            fn pow(self, rhs: $desired_rhs) -> $t {
                ($method)(*self as $new_type, rhs as $rhs) as $t
            }
        }

        impl<'a, 'b> Pow<&'a $desired_rhs> for &'b $t {
            type Output = $t;

            #[inline]

            fn pow(self, rhs: &'a $desired_rhs) -> $t {
                ($method)(*self as $new_type, *rhs as $rhs) as $t
            }
        }
    };
}

pow_impl!(u8, u32, u32, u8::pow);
pow_impl!(u16, u32, u32, u16::pow);
pow_impl!(u32, u32, u32, u32::pow);
pow_impl!(u64, u32, u32, u64::pow);
pow_impl!(u128, u32, u32, u128::pow);

pow_impl!(u8, f32, f32, f32, f32::powf);

pow_impl!(i8, u32, u32, i8::pow);
pow_impl!(i16, u32, u32, i16::pow);
pow_impl!(i32, u32, u32, i32::pow);
pow_impl!(i64, u32, u32, i64::pow);
pow_impl!(i128, u32, u32, i128::pow);

pow_impl!(f32, u32, i32, f32::powi);
pow_impl!(f32, f32, f32, f32::powf);
pow_impl!(f64, u32, i32, f64::powi);
pow_impl!(f64, f64, f64, f64::powf);
