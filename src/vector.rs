#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Vector { x, y }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn scale(&self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Vector {
        let mag = self.magnitude();
        if mag != 0.0 {
            self.scale(1.0 / mag)
        } else {
            *self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: why this deprecated?
    use std::f64::EPSILON;

    #[test]
    fn test_new() {
        let v = Vector::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vector::new(1.0, 2.0);
        let v2 = Vector::new(3.0, 4.0);
        let result = v1.add(&v2);
        assert_eq!(result.x, 4.0);
        assert_eq!(result.y, 6.0);
    }

    #[test]
    fn test_scale() {
        let v = Vector::new(1.0, 2.0);
        let result = v.scale(2.0);
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vector::new(1.0, 2.0);
        let v2 = Vector::new(3.0, 4.0);
        let result = v1.dot(&v2);
        assert_eq!(result, 11.0);
    }

    #[test]
    fn test_magnitude() {
        let v = Vector::new(3.0, 4.0);
        let result = v.magnitude();
        assert!((result - 5.0).abs() < EPSILON);
    }

    #[test]
    fn test_normalize() {
        let v = Vector::new(3.0, 4.0);
        let result = v.normalize();
        assert!((result.x - 0.6).abs() < EPSILON);
        assert!((result.y - 0.8).abs() < EPSILON);
    }

    #[test]
    fn test_normalize_zero_vector() {
        let v = Vector::new(0.0, 0.0);
        let result = v.normalize();
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
    }
}
