use std::ops::{Add, Div, Mul, Sub};

macro_rules! make {
    (op_self ~ $type: ident, $trait: ident, $fn: ident, { $($field: ident),* }) => {
        impl<T> $trait<$type<T>> for $type<T>
        where
            T: $trait<T, Output = T>,
        {
            type Output = Self;

            fn $fn(self, rhs: $type<T>) -> Self::Output {
                Self {
                    $($field: $trait::$fn(self.$field, rhs.$field)),*
                }
            }
        }
    };
    (op_t ~ $type: ident, $trait: ident, $fn: ident, { $($field: ident),* }) => {
        impl<T> $trait<T> for $type<T>
        where
            T: Copy + $trait<T, Output = T>,
        {
            type Output = Self;

            fn $fn(self, rhs: T) -> Self::Output {
                Self {
                    $($field: $trait::$fn(self.$field, rhs)),*
                }
            }
        }
    };
    (op_t_commutative ~ $type: ident, $trait: ident, $fn: ident) => {
        make!(op_t_commutative ~ $type, $trait, $fn, [i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64]);
    };
    (op_t_commutative ~ $type: ident, $trait: ident, $fn: ident, [ $($generic: ty),+ ]) => {
        $(
        impl $trait<$type<$generic>> for $generic {
            type Output = $type<$generic>;

            fn $fn(self, rhs: $type<$generic>) -> Self::Output {
                $trait::$fn(rhs, self)
            }
        }
        )+
    };
    (vec_consts ~ $type: ident, { $($field: ident),* }) => {
        make!(vec_consts ~ $type, { $($field),* }, [i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64]);
    };
    (vec_consts ~ $type: ident, { $($field: ident),* }, [ $fgeneric: ty $(,$generic: ty)* ]) => {
        impl ZeroExt for $type<$fgeneric> {
            const ZERO: Self = Self {
                $($field: 0 as $fgeneric),*
            };
        }
        make!(vec_consts ~ $type, { $($field),* }, [ $($generic),* ]);
    };
    (vec_consts ~ $type: ident, { $($field: ident),* }, []) => {};
    (from_to ~ $type: ident, $from: ty, $to: ty, { $($field: ident),* }) => {
        impl From<$type<$from>> for $type<$to> {
            fn from(value: $type<$from>) -> Self {
                Self {
                    $($field: value.$field as $to),*
                }
            }
        }
    };
}

pub trait ZeroExt {
    const ZERO: Self;
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

make!(op_self ~ Vec2, Add, add, {x, y});
make!(op_self ~ Vec3, Add, add, {x, y, z});
make!(op_self ~ Vec2, Sub, sub, {x, y});
make!(op_self ~ Vec3, Sub, sub, {x, y, z});
make!(op_t ~ Vec2, Mul, mul, {x, y});
make!(op_t ~ Vec3, Mul, mul, {x, y, z});
make!(op_t ~ Vec2, Div, div, {x, y});
make!(op_t ~ Vec3, Div, div, {x, y, z});
make!(op_t_commutative ~ Vec2, Mul, mul);
make!(op_t_commutative ~ Vec3, Mul, mul);
make!(vec_consts ~ Vec2, {x, y});
make!(vec_consts ~ Vec3, {x, y, z});
make!(from_to ~ Vec2, f32, f64, {x, y});
make!(from_to ~ Vec3, f32, f64, {x, y, z});
make!(from_to ~ Vec2, f64, f32, {x, y});
make!(from_to ~ Vec3, f64, f32, {x, y, z});
