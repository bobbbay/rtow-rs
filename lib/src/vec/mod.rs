pub mod vec3;

pub struct Vec<const SIZE: usize> {
    e: [i32; SIZE],
}
