/*

    MIT License
    
    Copyright (c) 2022 Siandfrance
    
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:
    
    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.
    
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

*/


use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign, Div, DivAssign};


///  Implement binary operations with object references
///  This comes from the rust source code
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a> $imp<&'a $u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b> $imp<&'a $u> for &'b $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}


/// Implement operator assignment for some type a binary operator
macro_rules! forward_assign_binop {
    (impl $imp:ident, $method:ident from $operator:ident for $t:ty, $u:ty) => {
        impl $imp<$u> for $t {

            #[inline]
            fn $method(&mut self, rhs: $u) {
                *self = Self::$operator(*self, rhs);
            }
        }

        impl<'a> $imp<&'a $u> for $t {

            #[inline]
            fn $method(&mut self, rhs: &'a $u) {
                *self = Self::$operator(*self, rhs);
            }
        }
    };
}


macro_rules! impl_reinterpret_memory_as {
    (from $u:ty => $t:ty) => {
        impl AsRef<$t> for $u {

            #[inline]
            fn as_ref(&self) -> &$t {
                unsafe {
                    let ptr: *const $u = self;
                    &*(ptr as *const $t)
                }
            }
        }

        impl AsMut<$t> for $u {

            #[inline]
            fn as_mut(&mut self) -> &mut $t {
                unsafe {
                    let ptr: *mut $u = self;
                    &mut *(ptr as *mut $t)
                }
            }
        }

        // impl<'a> AsMut<$t> for &'a $u {

        //     #[inline]
        //     fn as_mut(&mut self) -> &'a mut $t {
        //         unsafe {
        //             let ptr: *mut $u = *self;
        //             &mut *(ptr as *mut $t)
        //         }
        //     }
        // }
    };
}


macro_rules! impl_one_way_vector_cast {
    (from $vec1:ty{$t1:ty, $($c1:ident),+} into $vec2:ty{$t2:ty, $($c2:ident),+}) => {
        impl From<$vec1> for $vec2 {

            #[inline]
            fn from(value: $vec1) -> Self {
                Self::new($(value.$c1 as $t2),+)
            }
        }
    };
}


macro_rules! impl_vector_cast {
    ($vec1:ty{$t1:ty, $($c1:ident),+} <=> $vec2:ty{$t2:ty, $($c2:ident),+}) => {
        impl_one_way_vector_cast!(from $vec1{$t1, $($c1),+} into $vec2{$t2, $($c2),+});
        impl_one_way_vector_cast!(from $vec2{$t2, $($c2),+} into $vec1{$t1, $($c1),+});
    };
}


macro_rules! impl_scalar_operation {
    (right impl $imp:ident, $method:ident from $operator:tt for $vec:ty{$t:ty, $($coord:ident),+}, $scalar:ty) => {
        
        impl $imp<$scalar> for $vec {
            type Output = $vec;

            #[inline]
            fn $method(self, rhs: $scalar) -> Self::Output {
                <$vec>::new($(self.$coord $operator rhs),+)
            }
        }
    };
    (left impl $imp:ident, $method:ident from $operator:tt for $scalar:ty, $vec:ty{$t:ty, $($coord:ident),+}) => {
        
        impl $imp<$vec> for $scalar {
            type Output = $vec;

            #[inline]
            fn $method(self, rhs: $vec) -> Self::Output {
                <$vec>::new($(self $operator rhs.$coord),+)
            }
        }
    };
}


macro_rules! impl_vector_operation {
    (impl $imp:ident, $method:ident from $operator:tt for $vec:ty{$t:ty, $($coord:ident),+}) => {
        
        impl $imp for $vec {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: Self) -> Self::Output {
                <$vec>::new($(self.$coord $operator rhs.$coord),+)
            }
        }
    };
}


macro_rules! impl_vec_new {
    ($vec:ty{$t:ty, $($coord:ident),+}) => {
        impl $vec {
            pub const fn new($($coord: $t),+) -> Self {
                Self {
                    $($coord: $coord),+
                }
            }
        }
    };
}


macro_rules! sum {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($ys:expr),+) => {
        $x + sum!($($ys),+)
    };
}


macro_rules! impl_vec_dot {
    ($vec:ty{$t:ty, $($coord:ident),+}) => {
        impl $vec {
            pub fn dot(&self, other: &Self) -> $t {
                sum!($(self.$coord * other.$coord),+)
            }
        }
    };
}

macro_rules! impl_vec_len_sq {
    ($vec:ty{$t:ty, $($coord:ident),+}) => {
        impl $vec {
            pub fn length_sq(&self) -> $t {
                sum!($(self.$coord * self.$coord),+)
            }
        }
    };
}

macro_rules! impl_vector_base {
    (for $vec:ty{$t:ty, $($coord:ident),+}, $u:ty) => {
        // Basic implementation
        impl_vec_new!($vec{$t, $($coord),+});
        impl_vec_dot!($vec{$t, $($coord),+});
        impl_vec_len_sq!($vec{$t, $($coord),+});
        impl_reinterpret_memory_as!(from $vec => $vec);

        // Vector addition
        impl_vector_operation!(impl Add, add from + for $vec{$t, $($coord),+});
        forward_ref_binop!(impl Add, add for $vec, $vec);
        forward_assign_binop!(impl AddAssign, add_assign from add for $vec, $vec);

        // Vector subtraction
        impl_vector_operation!(impl Sub, sub from - for $vec{$t, $($coord),+});
        forward_ref_binop!(impl Sub, sub for $vec, $vec);
        forward_assign_binop!(impl SubAssign, sub_assign from sub for $vec, $vec);

        // Scalar multiplication
        impl_scalar_operation!(right impl Mul, mul from * for $vec{$t, $($coord),+}, $u);
        forward_ref_binop!(impl Mul, mul for $vec, $u);
        forward_assign_binop!(impl MulAssign, mul_assign from mul for $vec, $u);

        impl_scalar_operation!(left impl Mul, mul from * for $u, $vec{$t, $($coord),+});
        forward_ref_binop!(impl Mul, mul for $u, $vec);

        // Scalar division
        impl_scalar_operation!(right impl Div, div from / for $vec{$t, $($coord),+}, $u);
        forward_ref_binop!(impl Div, div for $vec, $u);
        forward_assign_binop!(impl DivAssign, div_assign from div for $vec, $u);
    };
}

macro_rules! impl_vec_len {
    ($vec:ty{$t:ty, $($coord:ident),+}) => {
        impl $vec {
            pub fn length(&self) -> $t {
                self.length_sq().sqrt()
            }

            pub fn normalized(&self) -> Self {
                self / self.length()
            }
        }
    };
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64
}


impl Vec2 {
    pub const ZERO  : Vec2 = Vec2::new(0, 0);
    pub const UNIT_X: Vec2 = Vec2::new(1, 0);
    pub const UNIT_Y: Vec2 = Vec2::new(0, 1);
    pub const ONE   : Vec2 = Vec2::new(1, 1);


    pub fn det(&self, other: &Self) -> i64 {
        self.x * other.y - self.y * other.x
    }
}


impl_vector_base!(for Vec2{i64, x, y}, i64);
impl_reinterpret_memory_as!(from (i64, i64) => Vec2);


#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        Vec2::new(($x) as i64, ($y) as i64)
    };
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64
}


impl Vec2f {
    pub const ZERO  : Vec2f = Vec2f::new(0.0, 0.0);
    pub const UNIT_X: Vec2f = Vec2f::new(1.0, 0.0);
    pub const UNIT_Y: Vec2f = Vec2f::new(0.0, 1.0);
    pub const ONE   : Vec2f = Vec2f::new(1.0, 1.0);

    pub fn det(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }
}


impl_vector_base!(for Vec2f{f64, x, y}, f64);
impl_reinterpret_memory_as!(from (f64, f64) => Vec2f);
impl_vec_len!(Vec2f{f64, x, y});


#[macro_export]
macro_rules! vec2f {
    ($x:expr, $y:expr) => {
        Vec2f::new(($x) as f64, ($y) as f64)
    };
}


impl_vector_cast!(Vec2{i64, x, y} <=> Vec2f{f64, x, y});


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64
}


impl Vec3 {
    pub const ZERO  : Vec3 = Vec3::new(0, 0, 0);
    pub const UNIT_X: Vec3 = Vec3::new(1, 0, 0);
    pub const UNIT_Y: Vec3 = Vec3::new(0, 1, 0);
    pub const UNIT_Z: Vec3 = Vec3::new(0, 0, 1);
    pub const ONE   : Vec3 = Vec3::new(1, 1, 1);

    pub fn cross(&self, other: &Self) -> i64 {
        self.x * other.y - self.y * other.x
    }
}


impl_vector_base!(for Vec3{i64, x, y, z}, i64);
impl_reinterpret_memory_as!(from (i64, i64, i64) => Vec3);


#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3::new(($x) as i64, ($y) as i64, ($z) as i64)
    };
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64
}


impl Vec3f {
    pub const ZERO  : Vec3f = Vec3f::new(0.0, 0.0, 0.0);
    pub const UNIT_X: Vec3f = Vec3f::new(1.0, 0.0, 0.0);
    pub const UNIT_Y: Vec3f = Vec3f::new(0.0, 1.0, 0.0);
    pub const UNIT_Z: Vec3f = Vec3f::new(0.0, 0.0, 1.0);
    pub const ONE   : Vec3f = Vec3f::new(1.0, 1.0, 1.0);


    pub fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }
}


impl_vector_base!(for Vec3f{f64, x, y, z}, f64);
impl_reinterpret_memory_as!(from (f64, f64, f64) => Vec3f);
impl_vec_len!(Vec3f{f64, x, y, z});


impl_vector_cast!(Vec3{i64, x, y, z} <=> Vec3f{f64, x, y, z});


#[macro_export]
macro_rules! vec3f {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3f::new(($x) as f64, ($y) as f64, ($z) as f64)
    };
}