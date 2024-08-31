use crate::rigid_body::RigidBody;
use crate::vector::Vector;

pub struct World {
    bodies: Vec<RigidBody>,
    gravity: Vector,
}

impl World {
    pub fn new(gravity: Vector) -> Self {
        World {
            bodies: Vec::new(),
            gravity,
        }
    }

    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.push(body);
    }

    pub fn step(&mut self, dt: f64) {
        for body in &mut self.bodies {
            // Apply gravity
            let gravity_force = self.gravity.scale(body.mass);
            body.apply_force(gravity_force, dt);

            // Update position
            body.update(dt);
        }

        // TODO: Implement collision detection and resolution here
    }

    pub fn body_count(&self) -> usize {
        self.bodies.len()
    }

    pub fn get_body(&self, index: usize) -> Option<&RigidBody> {
        self.bodies.get(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::rigid_body::Shape;

    use super::*;

    #[test]
    fn test_world_creation() {
        let gravity = Vector::new(0.0, -9.81);
        let world = World::new(gravity);
        assert_eq!(world.body_count(), 0);
        assert_eq!(world.gravity.x, 0.0);
        assert_eq!(world.gravity.y, -9.81);
    }

    #[test]
    fn test_add_body() {
        let mut world = World::new(Vector::new(0.0, -9.81));
        let body = RigidBody::new(Vector::new(0.0, 0.0), 1.0, Shape::Circle { radius: 1.0 });
        world.add_body(body);
        assert_eq!(world.body_count(), 1);
    }

    #[test]
    fn test_step() {
        let mut world = World::new(Vector::new(0.0, -9.81));
        let body = RigidBody::new(Vector::new(0.0, 10.0), 1.0, Shape::Circle { radius: 1.0 });
        world.add_body(body);

        let dt = 0.1;
        world.step(dt);

        let body = world.get_body(0).unwrap();
        assert!(body.velocity.y < 0.0, "Body should be falling");
        assert!(body.position.y < 10.0, "Body should have moved down");

        println!("Body position: {:?}", body.position);
    }
}
