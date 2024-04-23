use crate::compare_float;

fn cross_product(vector1: &Vector, vector2: &Vector) -> Vector {
    let x = vector1.y * vector2.z - vector1.z * vector2.y;
    let y = vector1.z * vector2.x - vector1.x * vector2.z;
    let z = vector1.x * vector2.y - vector1.y * vector2.x;

    Vector::new(x, y, z)
}

pub struct Point {
    pub position: Vector,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vector { x, y, z },
        }
    }
    pub fn from_vector(vector: &Vector) -> Self {
        Self {
            position: Vector {
                x: vector.x,
                y: vector.y,
                z: vector.z,
            },
        }
    }
    pub fn zero() -> Self {
        Self {
            position: Vector::zero(),
        }
    }

    pub fn direction(&self, point: &Point) -> Vector {
        let x_dir = self.position.x - point.position.x;
        let y_dir = self.position.y - point.position.y;
        let z_dir = self.position.z - point.position.z;
        Vector::new(x_dir, y_dir, z_dir)
    }

    pub fn compare(&self, point: &Point) -> bool {
        self.position.is_same(&point.position)
    }
}
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    pub fn copy(vector: &Vector) -> Self {
        Self::new(vector.x, vector.y, vector.z)
    }
    pub fn clone(&self) -> Vector {
        Vector::copy(self)
    }
    pub fn invert(&mut self) {
        let mut inverted = Vector::zero();
        inverted.subtract(self);
        self.overwrite(&inverted);
    }
    pub fn subtract(&mut self, vector: &Vector) {
        self.x -= vector.x;
        self.y -= vector.y;
        self.z -= vector.z;
    }
    pub fn apply(&mut self, vector: &Vector) -> Point {
        self.x += vector.x;
        self.y += vector.y;
        self.z += vector.z;

        Point {
            position: Vector::new(self.x, self.y, self.z),
        }
    }
    pub fn overwrite(&mut self, vector: &Vector) {
        self.x = vector.x;
        self.y = vector.y;
        self.z = vector.z;
    }
    pub fn is_same(&self, vector: &Vector) -> bool {
        if !compare_float(self.x, vector.x) {
            return false;
        }
        if !compare_float(self.y, vector.y) {
            return false;
        }
        if !compare_float(self.z, vector.z) {
            return false;
        }
        println!(
            "self:{}{}{} compare: {}{}{}",
            self.x, self.z, self.y, vector.x, vector.z, vector.y
        );
        true
    }
    pub fn scale(&mut self, scale: f32) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }
    pub fn magnitude(&self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z))
    }
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }
    pub fn dot_product(&self, vector: &Vector) -> f32 {
        let x = self.x * vector.x;
        let y = self.y * vector.y;
        let z = self.z * vector.z;

        x + y + z
    }
}

#[cfg(test)]
mod tests {
    use crate::points::{compare_float, cross_product, Point, Vector};

    #[test]
    fn test_compare() {
        let a = 2.4;
        let b = 2.2;

        assert!(!compare_float(a, b));

        let a = 1.1;
        let b = 1.1;

        assert!(compare_float(a, b));

        assert!(!compare_float(b, 20.0));
    }

    #[test]
    fn test_compare_location() {
        let vector = Vector::zero();
        let a = Point::from_vector(&vector);
        let b = Point::from_vector(&vector);

        let c = Point::new(2.2, 2.1, 2.2);

        assert!(a.compare(&b));
        assert_eq!(a.compare(&c), false);
    }

    #[test]
    fn test_compare_vector() {
        let vector = Vector::zero();
        let vector2 = Vector::zero();
        let vector3 = Vector::new(4.2, 2.4, 2.1);

        assert_eq!(vector.is_same(&vector2), true);
        assert_eq!(vector.is_same(&vector3), false);
    }

    #[test]
    fn test_apply_vector_to_point() {
        let mut a = Point::new(3.0, -2.0, 5.0);
        let b = Vector::new(-2.0, 3.0, 1.0);

        a.position.apply(&b);

        assert!(compare_float(a.position.x, 1.0));
        assert!(compare_float(a.position.y, 1.0));
        assert!(compare_float(a.position.z, 6.0));
    }
    #[test]
    fn test_subtract_vector_from_point() {
        let mut a = Point::new(1.0, 2.0, 3.0);
        let b = Vector::new(1.0, 2.0, 3.0);

        a.position.subtract(&b);

        assert!(compare_float(a.position.x, 0.0));
        assert!(compare_float(a.position.y, 0.0));
        assert!(compare_float(a.position.z, 0.0));
    }

    #[test]
    fn test_subtract_two_points() {
        let mut a = Point::new(2.0, 1.1, 2.2);
        let b = Point::new(2.2, 1.1, 2.2);

        a.position.subtract(&b.position);

        assert!(compare_float(a.position.x, -0.2));
        assert!(compare_float(a.position.y, 0.0));
        assert!(compare_float(a.position.z, 0.0));
    }

    #[test]
    fn test_invert_vector() {
        let mut a = Vector::new(1.1, 2.2, 3.3);
        a.invert();
        assert!(compare_float(a.x, -1.1));
        assert!(compare_float(a.y, -2.2));
        assert!(compare_float(a.z, -3.3));
    }

    #[test]
    fn test_scale_vector() {
        let mut a = Vector::new(1.0, 1.0, 1.0);
        let scale = 5.0;
        a.scale(scale);
        assert_eq!(a.x, 5.0);
        assert_eq!(a.z, 5.0);
        assert_eq!(a.y, 5.0);
    }

    #[test]
    fn test_magnitude() {
        let a = Vector::new(0.0, 1.0, 0.0);
        let b = Vector::new(2.0, 2.0, 0.0);
        let c = Vector::new(1.0, 0.0, 0.0);
        let d = Vector::new(0.0, 0.0, 1.0);
        let e = Vector::new(0.0, 0.0, 0.0);
        let f = Vector::new(4.0, 2.0, 3.0);
        assert!(compare_float(a.magnitude(), f32::sqrt(1.0)));
        assert!(compare_float(b.magnitude(), f32::sqrt(8.0)));
        assert!(compare_float(c.magnitude(), f32::sqrt(1.0)));
        assert!(compare_float(d.magnitude(), f32::sqrt(1.0)));
        assert!(compare_float(e.magnitude(), 0.0));
        assert!(compare_float(f.magnitude(), f32::sqrt(29.0)));
    }
    #[test]
    fn test_normalize() {
        let a = Vector::new(4.0, 0.0, 0.0);
        let mut b = a.clone();
        b.normalize();
        let correct_vector = Vector::new(1.0, 0.0, 0.0);

        assert!(b.is_same(&correct_vector));

        let mut c = Vector::new(1.0, 2.0, 3.0);
        println!("{}", c.magnitude());
        c.normalize();
        let correct_vector = Vector::new(0.26726, 0.53452, 0.80178);

        println!("{} {} {}", c.x, c.y, c.z);
        assert!(c.is_same(&correct_vector))
    }

    #[test]
    fn test_dot_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        let dot = a.dot_product(&b);
        assert_eq!(dot, 20.0);
    }

    #[test]
    fn test_cross_product() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        let a_b_cross = cross_product(&a, &b);
        let a_b_cross_correct = Vector::new(-1.0, 2.0, -1.0);
        assert!(a_b_cross.is_same(&a_b_cross_correct));

        let b_a_cross = cross_product(&b, &a);
        let b_a_cross_correct = Vector::new(1.0, -2.0, 1.0);
        assert!(b_a_cross.is_same(&b_a_cross_correct));
    }
}
