pub fn mult_two(x: f32) -> f32 {
    x * 2.
}


#[repr(C)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32, 
}

impl Point {
    pub fn mult_two(&mut self) {
        self.x *= 2.;
        self.y *= 2.;
        self.z *= 2.;
    }
}