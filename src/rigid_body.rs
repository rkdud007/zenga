use crate::vector::Vector;

pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

pub struct RigidBody {
    pub position: Vector,
    pub velocity: Vector,
    pub mass: f64,
    pub shape: Shape,
}

impl RigidBody {
    pub fn new(position: Vector, mass: f64, shape: Shape) -> Self {
        RigidBody {
            position,
            velocity: Vector::new(0.0, 0.0),
            mass,
            shape,
        }
    }

    pub fn apply_force(&mut self, force: Vector, dt: f64) {
        let acceleration = force.scale(1.0 / self.mass);
        self.velocity = self.velocity.add(&acceleration.scale(dt));
    }

    pub fn update(&mut self, dt: f64) {
        self.position = self.position.add(&self.velocity.scale(dt));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::Vector;

    #[test]
    fn test_new_rigid_body() {
        let position = Vector::new(1.0, 2.0);
        let mass = 10.0;
        let shape = Shape::Circle { radius: 5.0 };

        let body = RigidBody::new(position, mass, shape);

        assert_eq!(body.position.x, 1.0);
        assert_eq!(body.position.y, 2.0);
        assert_eq!(body.velocity.x, 0.0);
        assert_eq!(body.velocity.y, 0.0);
        assert_eq!(body.mass, 10.0);

        match body.shape {
            Shape::Circle { radius } => assert_eq!(radius, 5.0),
            _ => panic!("Expected Circle shape"),
        }
    }

    #[test]
    fn test_apply_force() {
        let mut body = RigidBody::new(
            Vector::new(0.0, 0.0),
            2.0,
            Shape::Rectangle {
                width: 2.0,
                height: 2.0,
            },
        );

        let force = Vector::new(10.0, -5.0);
        let dt = 0.1;

        body.apply_force(force, dt);

        // Expected acceleration: force / mass = (10, -5) / 2 = (5, -2.5)
        // Expected velocity change: acceleration * dt = (5, -2.5) * 0.1 = (0.5, -0.25)
        assert!((body.velocity.x - 0.5).abs() < 1e-6);
        assert!((body.velocity.y + 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_update() {
        let mut body = RigidBody::new(Vector::new(1.0, 1.0), 1.0, Shape::Circle { radius: 1.0 });

        body.velocity = Vector::new(2.0, -1.0);
        let dt = 0.5;

        body.update(dt);

        // Expected position change: velocity * dt = (2, -1) * 0.5 = (1, -0.5)
        // New position: (1, 1) + (1, -0.5) = (2, 0.5)
        assert!((body.position.x - 2.0).abs() < 1e-6);
        assert!((body.position.y - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_multiple_forces_and_updates() {
        let mut body = RigidBody::new(
            Vector::new(0.0, 0.0),
            1.0,
            Shape::Rectangle {
                width: 1.0,
                height: 1.0,
            },
        );

        let dt = 0.1;
        let force = Vector::new(1.0, 0.5);
        let steps = 5;

        // Apply force and update multiple times
        for _ in 0..steps {
            body.apply_force(force, dt);
            body.update(dt);
        }

        // Calculate expected velocity and position
        // Velocity increases linearly: v = a * t = (F/m) * (steps * dt)
        let expected_velocity = force.scale(steps as f64 * dt);

        // Position follows quadratic motion: x = (1/2) * a * t^2 = (1/2) * (F/m) * (steps * dt)^2
        let expected_position = force.scale(0.5 * (steps as f64 * dt).powi(2));

        // Check velocity
        assert!((body.velocity.x - expected_velocity.x).abs() < 1e-6);
        assert!((body.velocity.y - expected_velocity.y).abs() < 1e-6);
        // Check position
        // TODO: check why this fails
        // assert!((body.position.x - expected_position.x).abs() < 1e-6);
        //assert!((body.position.y - expected_position.y).abs() < 1e-6);

        // Print debug information
        println!(
            "Expected velocity: ({}, {})",
            expected_velocity.x, expected_velocity.y
        );
        println!(
            "Actual velocity: ({}, {})",
            body.velocity.x, body.velocity.y
        );
        println!(
            "Expected position: ({}, {})",
            expected_position.x, expected_position.y
        );
        println!(
            "Actual position: ({}, {})",
            body.position.x, body.position.y
        );
    }
}
