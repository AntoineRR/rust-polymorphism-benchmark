pub trait Shape {
    fn get_area(&self) -> f64;
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new() -> Self {
        Self { width: 2.0, height: 3.0 }
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }
}

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new() -> Self {
        Self { radius: 2.0 }
    }
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// First method, using the full `where` syntax:
pub fn print_area_where<T>(_shape: &T)
    where T: Shape + ?Sized
{
    // println!("{}", shape.get_area());
}

// Second method using a simplified syntax:
pub fn print_area_simplified<T: Shape + ?Sized>(_shape: &T) {
    // println!("{}", shape.get_area());
}

// Last method using the syntactic sugar keyword `impl`:
pub fn print_area_impl(_shape: &(impl Shape + ?Sized)) {
    // println!("{}", shape.get_area());
}

// A method using a trait object
pub fn print_area_dyn(_shape: &dyn Shape) {
    // println!("{}", shape.get_area());
}
