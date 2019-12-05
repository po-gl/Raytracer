use crate::shape::{Shape};
use std::fmt::{Debug};
use std::ops::{Index, IndexMut};
use std::borrow::BorrowMut;

/// # Shape list
/// `shape_list` is the module containing of all copies of shapes
/// indexed at their shape_id
///
/// This is a work-around to implement tree structures with
/// parents and children referencing each-other


//pub trait ShapeT {
//    fn id(&self) -> i32;
//    fn debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
//}
//impl Debug for Box<dyn ShapeT> {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//        self.debug_fmt(f)
//    }
//}
//
//#[derive(Debug, Clone)]
//pub struct Ball {
//    pub id: i32,
//}
//
//impl Ball {
//    pub fn new(sl: &mut ShapeList) -> Ball {
//        let id = get_shape_id();
//        let new_ball = Ball {id};
//        sl.push(Box::new(new_ball.clone()));
//        new_ball
//    }
//}
//
//impl ShapeT for Ball {
//    fn id(&self) -> i32 {
//        self.id
//    }
//
//    fn debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//        write!(f, "Box {:?}", self)
//    }
//}

#[derive(Debug)]
pub struct ShapeList {
    pub shapes: Vec<Box<dyn Shape>>,
}

impl ShapeList {
    pub fn new() -> ShapeList {
        ShapeList {shapes: vec![]}
    }

    pub fn get_id(&self) -> i32 {
        self.shapes.len() as i32
    }

    pub fn push(&mut self, val: Box<dyn Shape>) {
        self.shapes.push(val);
    }

    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    pub fn get(&self, id: i32) -> Box<dyn Shape> {
        self.shapes[id as usize].clone()
    }

    pub fn update(&mut self, val: Box<dyn Shape>) {
        std::mem::replace(self.shapes[val.id() as usize].borrow_mut(), val);
    }
}

impl Index<usize> for ShapeList {
    type Output = Box<dyn Shape>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.shapes[index]
    }
}

impl IndexMut<usize> for ShapeList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.shapes[index]
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use crate::shape::plane::Plane;
//
//    #[test]
//    fn shape_list_test() {
//
//        let mut sl = ShapeList::new();
//
//        let b1 = Ball::new(&mut sl);
//        let b2 = Ball::new(&mut sl);
//
//        println!("{:?}", b1);
//        println!("{:?}", b2);
//        println!("{:?}", sl);
//
//        assert!(false);
//    }
//}