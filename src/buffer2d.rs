use crate::math::Vec2i;

use std::ops::{Index, IndexMut};



#[derive(Clone)]
pub struct Buffer2D<A: Clone> {
    data: Vec<A>,
    size: Vec2i
}



impl<A: Clone> Buffer2D<A> {
    pub fn new(size: Vec2i, default: A) -> Self {
        Self {
            data: vec![default; (size.x * size.y) as usize],
            size: size
        }
    }


    pub fn size(&self) -> Vec2i {
        self.size
    }


    pub fn fill(&mut self, value: A) {
        for i in 0..self.data.len() {
            self.data[i] = value.clone();
        }
    }

    /// Resizes the buffer to the new size. Positional information will be lost during the resizing process
    pub fn raw_resize(&mut self, new_size: Vec2i, default: A) {
        self.size = new_size;
        self.data.resize((self.size.x * self.size.y) as usize, default);
        self.data.shrink_to_fit();
    }


    // TODO: implement resize
    pub fn resize(&mut self, new_size: Vec2i, default: A) {
        self.raw_resize(new_size, default)
    }


    pub fn get(&self, index: Vec2i, default: A) -> A {
        if !self.is_out_of_range(index) {
            self[index].clone()
        } else {
            default
        }
    }


    fn is_out_of_range(&self, p: Vec2i) -> bool {
        p.x < 0 || p.y < 0 || p.x >= self.size.x || p.y >= self.size.y
    }
}


impl<A: Clone> Index<Vec2i> for Buffer2D<A> {
    type Output = A;

    fn index(&self, p: Vec2i) -> &Self::Output {
        if !self.is_out_of_range(p) {
            &self.data[(p.x + p.y * self.size.x) as usize]
        } else {
            panic!("Buffer2D index out of range {:?} out of {:?}", p, &self.size);
        }
    }
}


impl<A: Clone> IndexMut<Vec2i> for Buffer2D<A> {

    fn index_mut(&mut self, p: Vec2i) -> &mut Self::Output {
        if !self.is_out_of_range(p) {
            &mut self.data[(p.x + p.y * self.size.x) as usize]
        } else {
            panic!("Buffer2D index out of range {:?} out of {:?}", p, &self.size);
        }
    }
}