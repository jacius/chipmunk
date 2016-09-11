#![allow(dead_code)]

use chip;

/// Collision response arbiter. Wrapper around `cpArbiter`.
pub struct Arbiter {
    pointer: *mut chip::cpArbiter,
}

pub struct ContactPointSet {
    pub count: u32,
    pub normal: (f64, f64),
    pub points: [ContactPoint; 2],
}

pub struct ContactPoint {
    pub a: (f64, f64),
    pub b: (f64, f64),
    pub dist: f64,
}


impl Arbiter {
    /// Create an Arbiter that wraps the given `cpArbiter` pointer.
    pub fn wrap(pointer: *mut chip::cpArbiter) -> Arbiter {
        Arbiter {
            pointer: pointer
        }
    }

    /// Return a raw pointer to the internal `cpArbiter`. Use with caution.
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const chip::cpArbiter {
        self.pointer as *const chip::cpArbiter
    }

    /// Return a raw mutable pointer to the internal `cpArbiter`. Use with caution.
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut chip::cpArbiter {
        self.pointer
    }

    /// Returns the set of contact points.
    pub fn contact_point_set(&self) -> ContactPointSet {
        let cps = unsafe {
            chip::cpArbiterGetContactPointSet(self.as_ptr())
        };

        ContactPointSet {
            count: cps.count as u32,
            normal: (cps.normal.x, cps.normal.y),
            points: [
                ContactPoint {
                    a: cps.points[0].pointA.into(),
                    b: cps.points[0].pointB.into(),
                    dist: cps.points[0].distance
                },
                ContactPoint {
                    a: cps.points[1].pointA.into(),
                    b: cps.points[1].pointB.into(),
                    dist: cps.points[1].distance
                }
            ]

        }
    }

    /// Returns the number of points of contact.
    pub fn count(&self) -> u32 {
        unsafe {
            chip::cpArbiterGetCount(self.as_ptr()) as u32
        }
    }

    /// Returns the depth of the penetration for a point of contact.
    pub fn depth(&self, i: u32) -> f64 {
        unsafe {
            chip::cpArbiterGetDepth(self.as_ptr(), i as i32)
        }
    }

    /// Returns the friction of the contact.
    pub fn friction(&self) -> f64 {
        unsafe {
            chip::cpArbiterGetFriction(self.as_ptr())
        }
    }

    /// Sets the friction for this collision.
    pub fn set_friction(&mut self, friction: f64) {
        unsafe {
            chip::cpArbiterSetFriction(self.as_mut_ptr(), friction);
        }
    }

    /// Returns the normal vector of the collision.
    pub fn normal(&self) -> (f64, f64) {
        unsafe {
            chip::cpArbiterGetNormal(self.as_ptr()).into()
        }
    }

    /// Returns a point on object `a` in the colision for a point of intersection.
    pub fn point_a(&self, i: u32) -> (f64, f64) {
        unsafe {
            chip::cpArbiterGetPointA(self.as_ptr(), i as i32).into()
        }
    }

    /// Returns a point on object `b` in the colision for a point of intersection.
    pub fn point_b(&self, i: u32) -> (f64, f64) {
        unsafe {
            chip::cpArbiterGetPointB(self.as_ptr(), i as i32).into()
        }
    }

    /// Returns the restitution for this collision.
    pub fn restitution(&self) -> f64 {
        unsafe {
            chip::cpArbiterGetRestitution(self.as_ptr())
        }
    }

    /// Sets the restitutionfor this collision.
    pub fn set_restitution(&mut self, restitution: f64) {
        unsafe {
            chip::cpArbiterSetRestitution(self.as_mut_ptr(), restitution);
        }
    }

    /// Returns the surface velocity of this collision.
    pub fn surface_velocity(&self) -> (f64, f64) {
        unsafe {
            chip::cpArbiterGetSurfaceVelocity(self.as_ptr()).into()
        }
    }

    /// Sets the surface velocity for this collision.
    pub fn set_surface_velocity(&mut self, vx: f64, vy: f64) {
        unsafe {
            chip::cpArbiterSetSurfaceVelocity(self.as_mut_ptr(), chip::cpv(vx, vy));
        }
    }
}
