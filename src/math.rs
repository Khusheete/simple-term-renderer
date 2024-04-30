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


pub trait Number: Sized + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
        + Copy + Clone + PartialEq {}

impl Number for i32 {}
impl Number for f32 {}


#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Vector2<N: Number> {
    pub x: N,
    pub y: N
}


impl<N: Number> Vector2<N> {

    pub const fn new(x: N, y: N) -> Self {
        Self {
            x: x,
            y: y
        }
    }


    pub fn dot(a: Self, b: Self) -> N {
        a.x * b.x + a.y * b.y
    }


    pub fn det(a: Self, b: Self) -> N {
        a.x * b.y - a.y * b.x
    }


    pub fn length_sq(&self) -> N {
        self.x * self.x + self.y * self.y
    }
}



impl<N: Number> Add for Vector2<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}


impl<N: Number> AddAssign for Vector2<N> {

    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}


impl<N: Number> Sub for Vector2<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}


impl<N: Number> SubAssign for Vector2<N> {

    fn sub_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}


impl<N: Number> Mul<N> for Vector2<N> {
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}


impl<N: Number> MulAssign<N> for Vector2<N> {

    fn mul_assign(&mut self, rhs: N) {
        *self = *self * rhs;
    }
}


impl<N: Number> Div<N> for Vector2<N> {
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}


impl<N: Number> DivAssign<N> for Vector2<N> {

    fn div_assign(&mut self, rhs: N) {
        *self = *self / rhs;
    }
}


#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {Vec2::new($x as i32, $y as i32)};
}


pub type Vec2 = Vector2<i32>;


impl Vec2 {
    pub const ZERO: Vec2 = Vec2::new(0, 0);
    pub const UNIT_X: Vec2 = Vec2::new(1, 0);
    pub const UNIT_Y: Vec2 = Vec2::new(0, 1);
    pub const ONE: Vec2 = Vec2::new(1, 1);
}


impl Eq for Vec2 {}



impl AsRef<Vec2> for (u32, u32) {


    fn as_ref(&self) -> &Vec2 {
        if self.0 > i32::MAX as u32 || self.1 > i32::MAX as u32 {
            panic!("Cannot convert {:?} to Vec2, integeroverflow", self);
        }
        unsafe {
            let ptr: *const (u32, u32) = self;
            &*(ptr as *const Vec2)
        }
    }
}


impl AsRef<Vec2> for (i32, i32) {


    fn as_ref(&self) -> &Vec2 {
        unsafe {
            let ptr: *const (i32, i32) = self;
            &*(ptr as *const Vec2)
        }
    }
}


impl AsMut<Vec2> for (u32, u32) {


    fn as_mut(&mut self) -> &mut Vec2 {
        if self.0 > i32::MAX as u32 || self.1 > i32::MAX as u32 {
            panic!("Cannot convert {:?} to Vec2, integeroverflow", self);
        }
        unsafe {
            let ptr: *mut (u32, u32) = self;
            &mut *(ptr as *mut Vec2)
        }
    }
}


impl AsMut<Vec2> for (i32, i32) {


    fn as_mut(&mut self) -> &mut Vec2 {
        unsafe {
            let ptr: *mut (i32, i32) = self;
            &mut *(ptr as *mut Vec2)
        }
    }
}


impl Into<Vec2> for (i32, i32) {


    fn into(self) -> Vec2 {
        Vec2::new(self.0, self.1)
    }
}


impl Into<Vec2> for (u32, u32) {


    fn into(self) -> Vec2 {
        Vec2::new(self.0 as i32, self.1 as i32)
    }
}


impl AsRef<Vec2> for (usize, usize) {


    fn as_ref(&self) -> &Vec2 {
        if self.0 > i32::MAX as usize || self.1 > i32::MAX as usize {
            panic!("Cannot convert {:?} to Vec2, integeroverflow", self);
        }
        unsafe {
            let ptr: *const (usize, usize) = self;
            &*(ptr as *const Vec2)
        }
    }
}


impl AsRef<Vec2> for (isize, isize) {


    fn as_ref(&self) -> &Vec2 {
        unsafe {
            let ptr: *const (isize, isize) = self;
            &*(ptr as *const Vec2)
        }
    }
}


impl AsMut<Vec2> for (usize, usize) {


    fn as_mut(&mut self) -> &mut Vec2 {
        if self.0 > i32::MAX as usize || self.1 > i32::MAX as usize {
            panic!("Cannot convert {:?} to Vec2, integeroverflow", self);
        }
        unsafe {
            let ptr: *mut (usize, usize) = self;
            &mut *(ptr as *mut Vec2)
        }
    }
}


impl AsMut<Vec2> for (isize, isize) {


    fn as_mut(&mut self) -> &mut Vec2 {
        unsafe {
            let ptr: *mut (isize, isize) = self;
            &mut *(ptr as *mut Vec2)
        }
    }
}


impl Into<Vec2> for (isize, isize) {


    fn into(self) -> Vec2 {
        Vec2::new(self.0 as i32, self.1 as i32)
    }
}


impl Into<Vec2> for (usize, usize) {


    fn into(self) -> Vec2 {
        Vec2::new(self.0 as i32, self.1 as i32)
    }
}


impl AsRef<Vec2> for Vec2 {

    fn as_ref(&self) -> &Vec2 {
        self
    }
}


impl AsMut<Vec2> for Vec2 {

    fn as_mut(&mut self) -> &mut Vec2 {
        self
    }
}

impl From<Vec2f> for Vec2 {

    fn from(value: Vec2f) -> Self {
        Vec2::new(value.x as i32, value.y as i32)
    }
}



#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Vector3<N: Number> {
    pub x: N,
    pub y: N,
    pub z: N
}


impl<N: Number> Vector3<N> {

    pub const fn new(x: N, y: N, z: N) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }


    pub fn dot(a: Self, b: Self) -> N {
        a.x * b.x + a.y * b.y + a.z * b.z
    }


    pub fn length_sq(&self) -> N {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}



impl<N: Number> Add for Vector3<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}


impl<N: Number> AddAssign for Vector3<N> {

    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}


impl<N: Number> Sub for Vector3<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}


impl<N: Number> SubAssign for Vector3<N> {

    fn sub_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}


impl<N: Number> Mul<N> for Vector3<N> {
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}


impl<N: Number> MulAssign<N> for Vector3<N> {

    fn mul_assign(&mut self, rhs: N) {
        *self = *self * rhs;
    }
}


impl<N: Number> Div<N> for Vector3<N> {
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}


impl<N: Number> DivAssign<N> for Vector3<N> {

    fn div_assign(&mut self, rhs: N) {
        *self = *self / rhs;
    }
}


#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {Vec3::new($x as i32, $y as i32, $z as i32)};
}


pub type Vec3 = Vector3<i32>;


impl Vec3 {
    pub const ZERO  : Vec3 = Vec3::new(0, 0, 0);
    pub const UNIT_X: Vec3 = Vec3::new(1, 0, 0);
    pub const UNIT_Y: Vec3 = Vec3::new(0, 1, 0);
    pub const UNIT_Z: Vec3 = Vec3::new(0, 0, 1);
    pub const ONE   : Vec3 = Vec3::new(1, 1, 1);
}


impl AsRef<Vec3> for (u32, u32, u32) {


    fn as_ref(&self) -> &Vec3 {
        if self.0 > i32::MAX as u32 || self.1 > i32::MAX as u32 || self.2 > i32::MAX as u32 {
            panic!("Cannot convert {:?} to Vec3, integeroverflow", self);
        }
        unsafe {
            let ptr: *const (u32, u32, u32) = self;
            &*(ptr as *const Vec3)
        }
    }
}


impl AsRef<Vec3> for (i32, i32, i32) {


    fn as_ref(&self) -> &Vec3 {
        unsafe {
            let ptr: *const (i32, i32, i32) = self;
            &*(ptr as *const Vec3)
        }
    }
}


impl AsMut<Vec3> for (u32, u32, u32) {


    fn as_mut(&mut self) -> &mut Vec3 {
        if self.0 > i32::MAX as u32 || self.1 > i32::MAX as u32 || self.2 > i32::MAX as u32 {
            panic!("Cannot convert {:?} to Vec3, integeroverflow", self);
        }
        unsafe {
            let ptr: *mut (u32, u32, u32) = self;
            &mut *(ptr as *mut Vec3)
        }
    }
}


impl AsMut<Vec3> for (i32, i32, i32) {


    fn as_mut(&mut self) -> &mut Vec3 {
        unsafe {
            let ptr: *mut (i32, i32, i32) = self;
            &mut *(ptr as *mut Vec3)
        }
    }
}


impl Into<Vec3> for (i32, i32, i32) {


    fn into(self) -> Vec3 {
        Vec3::new(self.0, self.1, self.2)
    }
}


impl Into<Vec3> for (u32, u32, u32) {


    fn into(self) -> Vec3 {
        Vec3::new(self.0 as i32, self.1 as i32, self.2 as i32)
    }
}


impl AsRef<Vec3> for (usize, usize, usize) {


    fn as_ref(&self) -> &Vec3 {
        if self.0 > i32::MAX as usize || self.1 > i32::MAX as usize || self.2 > i32::MAX as usize {
            panic!("Cannot convert {:?} to Vec3, integeroverflow", self);
        }
        unsafe {
            let ptr: *const (usize, usize, usize) = self;
            &*(ptr as *const Vec3)
        }
    }
}


impl AsRef<Vec3> for (isize, isize, isize) {


    fn as_ref(&self) -> &Vec3 {
        unsafe {
            let ptr: *const (isize, isize, isize) = self;
            &*(ptr as *const Vec3)
        }
    }
}


impl AsMut<Vec3> for (usize, usize, usize) {


    fn as_mut(&mut self) -> &mut Vec3 {
        if self.0 > i32::MAX as usize || self.1 > i32::MAX as usize || self.2 > i32::MAX as usize {
            panic!("Cannot convert {:?} to Vec3, integeroverflow", self);
        }
        unsafe {
            let ptr: *mut (usize, usize, usize) = self;
            &mut *(ptr as *mut Vec3)
        }
    }
}


impl AsMut<Vec3> for (isize, isize, isize) {


    fn as_mut(&mut self) -> &mut Vec3 {
        unsafe {
            let ptr: *mut (isize, isize, isize) = self;
            &mut *(ptr as *mut Vec3)
        }
    }
}


impl Into<Vec3> for (isize, isize, isize) {


    fn into(self) -> Vec3 {
        Vec3::new(self.0 as i32, self.1 as i32, self.2 as i32)
    }
}


impl Into<Vec3> for (usize, usize, usize) {


    fn into(self) -> Vec3 {
        Vec3::new(self.0 as i32, self.1 as i32, self.2 as i32)
    }
}


impl AsRef<Vec3> for Vec3 {

    fn as_ref(&self) -> &Vec3 {
        self
    }
}


impl AsMut<Vec3> for Vec3 {

    fn as_mut(&mut self) -> &mut Vec3 {
        self
    }
}


impl From<Vec3f> for Vec3 {

    fn from(value: Vec3f) -> Self {
        Vec3::new(value.x as i32, value.y as i32, value.z as i32)
    }
}


#[macro_export]
macro_rules! vec2f {
    ($x:expr, $y:expr) => {Vec2f::new($x as f32, $y as f32)};
}


#[macro_export]
macro_rules! vec3f {
    ($x:expr, $y:expr, $z:expr) => {Vec3f::new($x as f32, $y as f32, $z as f32)};
}


pub type Vec2f = Vector2<f32>;
pub type Vec3f = Vector3<f32>;


impl Vec2f {
    pub const ZERO  : Vec2f = Vec2f::new(0.0, 0.0);
    pub const UNIT_X: Vec2f = Vec2f::new(1.0, 0.0);
    pub const UNIT_Y: Vec2f = Vec2f::new(0.0, 1.0);
    pub const ONE   : Vec2f = Vec2f::new(1.0, 1.0);


    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }
}

impl Eq for Vec2f {}


impl From<Vec2> for Vec2f {

    fn from(value: Vec2) -> Self {
        Vec2f::new(value.x as f32, value.y as f32)
    }
}



impl Vec3f {
    pub const ZERO  : Vec3f = Vec3f::new(0.0, 0.0, 0.0);
    pub const UNIT_X: Vec3f = Vec3f::new(1.0, 0.0, 0.0);
    pub const UNIT_Y: Vec3f = Vec3f::new(0.0, 1.0, 0.0);
    pub const UNIT_Z: Vec3f = Vec3f::new(0.0, 0.0, 1.0);
    pub const ONE   : Vec3f = Vec3f::new(1.0, 1.0, 1.0);

    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }
}


impl Eq for Vec3f {}


impl From<Vec3> for Vec3f {

    fn from(value: Vec3) -> Self {
        Vec3f::new(value.x as f32, value.y as f32, value.z as f32)
    }
}