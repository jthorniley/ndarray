
extern crate ndarray;

use ndarray::Array;
//use ndarray::{Si, S};

fn main()
{
    let mat = Array::from_iter(range(0.0f32, 16.0)).reshape((2u, 4u, 2u));
    println!("{a}\n times \n{b}\nis equal to:\n{c}",
             a=mat.subview(2,1),
             b=mat.subview(0,1),
             c=mat.subview(2,1).mat_mul(&mat.subview(0,1)));

}