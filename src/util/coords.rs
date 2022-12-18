use derive_more::{Add, AddAssign, Neg, Product, Sub, SubAssign, Sum};
use std::ops::{Div, DivAssign, Mul, MulAssign};
use num::PrimInt;

pub trait CoordNum: PrimInt + Default {}

impl<T> CoordNum for T where T: PrimInt + Default {}

#[derive(
    Add,
    AddAssign,
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Neg,
    Ord,
    PartialEq,
    PartialOrd,
    Product,
    Sub,
    SubAssign,
    Sum,
)]
pub struct Coord2<T: CoordNum>(pub T, pub T);

#[derive(
    Add,
    AddAssign,
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Neg,
    PartialEq,
    Product,
    Sub,
    SubAssign,
    Sum,
)]
pub struct Coord3<T: CoordNum>(pub T, pub T, pub T);

#[derive(
    Add,
    AddAssign,
    Copy,
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Neg,
    PartialEq,
    Product,
    Sub,
    SubAssign,
    Sum,
)]
pub struct Coord4<T: CoordNum>(pub T, pub T, pub T, pub T);

macro_rules! coord_impls {
    ($name:ident, $($field:tt),*) => {
        impl<T: CoordNum> $name<T> {
            neighbors_fn!($($field)*);

            orthogonal_neighbors_fn!($($field)*);

            pub fn manhattan_norm(self) -> T {
                T::default() $(+ abs(self.$field))*
            }

            pub fn manhattan_distance(self, other: Self) -> T {
                T::default() $(+ abs_diff(self.$field, other.$field))*
            }
        }

        impl<T: CoordNum> Mul<T> for $name<T> {
            type Output = Self;

            fn mul(self, rhs: T) -> Self::Output {
                Self($(self.$field * rhs,)*)
            }
        }

        impl<T: CoordNum> MulAssign<T> for $name<T> {
            fn mul_assign(&mut self, rhs: T) {
                $(self.$field = self.$field * rhs;)*
            }
        }

        impl<T: CoordNum> Div<T> for $name<T> {
            type Output = Self;

            fn div(self, rhs: T) -> Self::Output {
                Self($(self.$field / rhs,)*)
            }
        }

        impl<T: CoordNum> DivAssign<T> for $name<T> {
            fn div_assign(&mut self, rhs: T) {
                $(self.$field = self.$field / rhs;)*
            }
        }
    };
}

macro_rules! neighbors_fn {
    (@inner $self:ident $result:ident $neighbor:ident) => {
        if $neighbor != $self {
            $result.push($neighbor);
        }
    };
    (@inner $self:ident $result:ident $neighbor:ident $head_field:tt $($rest_field:tt)*) => {
        for x in [$self.$head_field - T::one(), $self.$head_field, $self.$head_field + T::one()] {
            $neighbor.$head_field = x;
            neighbors_fn!(@inner $self $result $neighbor $($rest_field)*);
        }
    };
    ($($field:tt)*) => {
        pub fn neighbors(self) -> Vec<Self> {
            let mut result = vec![];
            let mut neighbor = Self::default();
            neighbors_fn!(@inner self result neighbor $($field)*);
            result
        }
    };
}

macro_rules! orthogonal_neighbors_fn {
    ($($field:tt)*) => {
        pub fn orthogonal_neighbors(self) -> Vec<Self> {
            let mut result = vec![];
            $(
                for x in [self.$field - T::one(), self.$field + T::one()] {
                    let mut neighbor = self;
                    neighbor.$field = x;
                    result.push(neighbor);
                }
            )*
            result
        }
    };
}

coord_impls!(Coord2, 0, 1);
coord_impls!(Coord3, 0, 1, 2);
coord_impls!(Coord4, 0, 1, 2, 3);

fn abs<T: PrimInt>(x: T) -> T {
    if x < T::zero() { T::zero() - x } else { x }
}

fn abs_diff<T: PrimInt>(a: T, b: T) -> T {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_coord2() {
        assert_eq!(Coord2::default(), Coord2(0, 0));
        let c = Coord2(1, -2);
        assert_eq!(c + c, Coord2(2, -4));
        assert_eq!(c * 10, Coord2(10, -20));
        assert_eq!(c.manhattan_norm(), 3);
        let neighbors_set: HashSet<_> = c.neighbors().into_iter().collect();
        let neighbors_expected: HashSet<_> = [
            Coord2(0, -3),
            Coord2(0, -2),
            Coord2(0, -1),
            Coord2(1, -3),
            Coord2(1, -1),
            Coord2(2, -3),
            Coord2(2, -2),
            Coord2(2, -1),
        ]
        .into_iter()
        .collect();
        assert_eq!(neighbors_set, neighbors_expected);
        let neighbors_set: HashSet<_> = c.orthogonal_neighbors().into_iter().collect();
        let neighbors_expected: HashSet<_> =
            [Coord2(1, -3), Coord2(1, -1), Coord2(0, -2), Coord2(2, -2)]
                .into_iter()
                .collect();
        assert_eq!(neighbors_set, neighbors_expected);
        let c1 = Coord2(2_u32, 2_u32);
        let c2 = Coord2(1_u32, 3_u32);
        assert_eq!(c1.manhattan_distance(c2), 2);
    }

    #[test]
    fn test_coord3() {
        let c = Coord3(1, -2, 3);
        assert_eq!(c + c, Coord3(2, -4, 6));
        assert_eq!(c * 10, Coord3(10, -20, 30));
        assert_eq!(c.manhattan_norm(), 6)
    }
}
