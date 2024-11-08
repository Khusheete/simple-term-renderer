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


use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};


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
            pub fn dot(&self, other: Self) -> $t {
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

macro_rules! impl_vec_neg {
    ($vec:ty{$t:ty, $($coord:ident),+}) => {
        impl Neg for $vec {
            type Output = Self;

            fn neg(self) -> Self {
                Self::new($(-self.$coord),+)
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

        // Negation
        impl_vec_neg!($vec{$t, $($coord),+});
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


macro_rules! _mat_mult_elem {
    ($a:ident{$($c1a:ident),+}.$c1b:ident, $b:ident.$c2:ident) => {
        sum!(
            $($a.$c1a.$c1b * $b.$c2.$c1a),+
        )
    };
}


macro_rules! _mat_mult_line {
    ($a:ident $c1as:tt . {$($c1b:ident),+}, $b:ident.$c2:ident, $vec:ty) => {
        <$vec>::new($(
            _mat_mult_elem!($a$c1as.$c1b, $b.$c2)
        ),+)
    };
}

macro_rules! _mat_mult_expression {
    ($a:ident $coord1:tt, $b:ident{$($coord2:ident),+}, $vec:ty) => {
        Self::new($(_mat_mult_line!(
            $a $coord1 . $coord1, $b.$coord2, $vec
        )),+)
    };
}


macro_rules! impl_mat_mult {
    ($mat:ident{$vec:ty{$t:ty, $($coord:ident),+}}) => {

        impl Mul for $mat {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                _mat_mult_expression!(
                    self{$($coord),+},
                    other{$($coord),+},
                    $vec
                )
            }
        }
    };
}


macro_rules! _mat_vec_mult_line {
    ($mat:ident {$($c1:ident),+}.$c2:ident, $v:ident) => {
        sum!($(
            $mat.$c1.$c2 * $v.$c1
        ),+)
    };
}


macro_rules! _mat_vec_mult_expression {
    ($mat:ident $c1s:tt . {$($c2:ident),+}, $v:ident, $vec:ty) => {
        <$vec>::new($(
            _mat_vec_mult_line!($mat $c1s.$c2, $v)
        ),+)
    };
}


macro_rules! impl_mat_vec_mult {
    ($mat:ident{$vec:ty{$t:ty, $($coord:ident),+}}) => {
        
        impl Mul<$vec> for $mat {
            type Output = $vec;

            fn mul(self, other: $vec) -> Self::Output {
                    _mat_vec_mult_expression!(self {$($coord),+}.{$($coord),+}, other, $vec)
            }
        }
    };
}


macro_rules! define_matrix_type {
    ($mat:ident{$vec:ty{$t:ty, $($coord:ident),+}}) => {
        // Definition
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $mat {
            $(pub $coord: $vec),+
        }


        impl $mat {
            pub fn new($($coord: $vec),+) -> Self {
                Self {
                    $($coord: $coord),+
                }
            }
        }

        // Matrix addition
        impl_vector_operation!(impl Add, add from + for $mat{$vec, $($coord),+});
        forward_ref_binop!(impl Add, add for $mat, $mat);
        forward_assign_binop!(impl AddAssign, add_assign from add for $mat, $mat);

        // Matrix subtraction
        impl_vector_operation!(impl Sub, sub from + for $mat{$vec, $($coord),+});
        forward_ref_binop!(impl Sub, sub for $mat, $mat);
        forward_assign_binop!(impl SubAssign, sub_assign from sub for $mat, $mat);

        // Matrix multiplication
        impl_mat_mult!($mat{$vec{$t, $($coord),+}});
        forward_ref_binop!(impl Mul, mul for $mat, $mat);
        forward_assign_binop!(impl MulAssign, mul_assign from mul for $mat, $mat);

        // Matrix vector multiplication
        impl_mat_vec_mult!($mat{$vec{$t, $($coord),+}});
        forward_ref_binop!(impl Mul, mul for $mat, $vec);

        // Scalar multiplication
        impl_scalar_operation!(right impl Mul, mul from * for $mat{$vec, $($coord),+}, $t);
        forward_ref_binop!(impl Mul, mul for $mat, $t);
        forward_assign_binop!(impl MulAssign, mul_assign from mul for $mat, $t);

        impl_scalar_operation!(left impl Mul, mul from * for $t, $mat{$vec, $($coord),+});
        forward_ref_binop!(impl Mul, mul for $t, $mat);

        // Scalar division
        impl_scalar_operation!(right impl Div, div from / for $mat{$vec, $($coord),+}, $t);
        forward_ref_binop!(impl Div, div for $mat, $t);
        forward_assign_binop!(impl DivAssign, div_assign from div for $mat, $t);

        // Negation
        impl_vec_neg!($mat{$vec, $($coord),+});
    };
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}


impl Vec2 {
    pub const ZERO  : Vec2 = Vec2::new(0.0, 0.0);
    pub const UNIT_X: Vec2 = Vec2::new(1.0, 0.0);
    pub const UNIT_Y: Vec2 = Vec2::new(0.0, 1.0);
    pub const ONE   : Vec2 = Vec2::new(1.0, 1.0);


    pub fn det(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }
}


impl_vector_base!(for Vec2{f64, x, y}, f64);
impl_reinterpret_memory_as!(from (f64, f64) => Vec2);

impl_vec_len!(Vec2{f64, x, y});


#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        Vec2::new(($x) as f64, ($y) as f64)
    };
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2i {
    pub x: i64,
    pub y: i64
}


impl Vec2i {
    pub const ZERO  : Vec2i = Vec2i::new(0, 0);
    pub const UNIT_X: Vec2i = Vec2i::new(1, 0);
    pub const UNIT_Y: Vec2i = Vec2i::new(0, 1);
    pub const ONE   : Vec2i = Vec2i::new(1, 1);

    pub fn det(&self, other: &Self) -> i64 {
        self.x * other.y - self.y * other.x
    }
}


impl_vector_base!(for Vec2i{i64, x, y}, i64);
impl_reinterpret_memory_as!(from (i64, i64) => Vec2i);


#[macro_export]
macro_rules! vec2i {
    ($x:expr, $y:expr) => {
        Vec2i::new(($x) as i64, ($y) as i64)
    };
}


impl_vector_cast!(Vec2{f64, x, y} <=> Vec2i{i64, x, y});

define_matrix_type!(Mat2{Vec2{f64, x, y}});


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}


impl Vec3 {
    pub const ZERO  : Vec3 = Vec3::new(0.0, 0.0, 0.0);
    pub const UNIT_X: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    pub const UNIT_Y: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    pub const UNIT_Z: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    pub const ONE   : Vec3 = Vec3::new(1.0, 1.0, 1.0);

    pub fn cross(&self, other: Self) -> Vec3 {
        Vec3::new(
              self.y * other.z - self.z * other.y,
            -(self.x * other.z - self.z * other.x),
              self.x * other.y - self.y * other.x
        )
    }
}


impl_vector_base!(for Vec3{f64, x, y, z}, f64);
impl_reinterpret_memory_as!(from (f64, f64, f64) => Vec3);
impl_vec_len!(Vec3{f64, x, y, z});

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3::new(($x) as f64, ($y) as f64, ($z) as f64)
    };
}

define_matrix_type!(Mat3{Vec3{f64, x, y, z}});


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec3i {
    pub x: i64,
    pub y: i64,
    pub z: i64
}


impl Vec3i {
    pub const ZERO  : Vec3i = Vec3i::new(0, 0, 0);
    pub const UNIT_X: Vec3i = Vec3i::new(1, 0, 0);
    pub const UNIT_Y: Vec3i = Vec3i::new(0, 1, 0);
    pub const UNIT_Z: Vec3i = Vec3i::new(0, 0, 1);
    pub const ONE   : Vec3i = Vec3i::new(1, 1, 1);


    pub fn cross(&self, other: Self) -> Vec3i {
        Vec3i::new(
            self.y * other.z - self.z * other.y,
          -(self.x * other.z - self.z * other.x),
            self.x * other.y - self.z * other.x
      )
    }
}


impl_vector_base!(for Vec3i{i64, x, y, z}, i64);
impl_reinterpret_memory_as!(from (i64, i64, i64) => Vec3i);


impl_vector_cast!(Vec3{f64, x, y, z} <=> Vec3i{i64, x, y, z});


#[macro_export]
macro_rules! vec3i {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3i::new(($x) as i64, ($y) as i64, ($z) as i64)
    };
}