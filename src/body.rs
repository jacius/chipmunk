use chip;
use chip::cpv;

use super::handle::Handle;


/// BodyHandle provides several shortcuts for creating a new Body and putting it in a Handle.
///
/// - `BodyHandle::new(mass, moment)` is the same as `BodyHandle::from(Body::new(mass, moment))`
/// - `BodyHandle::new_kinematic()` is the same as `BodyHandle::from(Body::new_kinematic())`
/// - `BodyHandle::new_static()` is the same as `BodyHandle::from(Body::new_static())`
pub type BodyHandle = Handle<Body>;

impl BodyHandle {
    pub fn new(mass: f64, moment: f64) -> BodyHandle {
        BodyHandle::from(Body::new(mass, moment))
    }

    pub fn new_kinematic() -> BodyHandle {
        BodyHandle::from(Body::new_kinematic())
    }

    pub fn new_static() -> BodyHandle {
        BodyHandle::from(Body::new_static())
    }
}


/// A rigid body. Wrapper around `cpBody`.
///
/// From the Chipmunk docs:
///
/// > Rigid bodies hold the physical properties of an object like it's mass, and position and velocity of it's center of gravity.
/// > They don't have an shape on their own. They are given a shape by creating collision shapes (`cpShape`) that point to the body.
pub struct Body {
    pointer: *mut chip::cpBody
}

impl Drop for Body {
    fn drop(&mut self) {
        unsafe {
            chip::cpBodyDestroy(self.pointer);
        }
    }
}

use std::fmt;
impl fmt::Debug for Body {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Body")
            .field("position", &self.position())
            .field("velocity", &self.velocity())
            .field("angle_deg", &self.angle_deg())
            .field("mass", &self.mass())
            .finish()
    }
}

impl Body {
    /// Create a new dynamic Body with the given mass and moment of inertia.
    pub fn new(mass: f64, moment: f64) -> Body {
        Body {
            pointer: unsafe { chip::cpBodyNew(mass, moment) }
        }
    }

    /// Create a new kinematic Body. From the Chipmunk docs:
    ///
    /// > A kinematic body is an infinite mass, user controlled body that is not affected by gravity, forces or collisions.
    /// >
    /// > Instead the body only moves based on it's velocity.
    /// > Dynamic bodies collide normally with kinematic bodies, though the kinematic body will be unaffected.
    /// > Collisions between two kinematic bodies, or a kinematic body and a static body produce collision callbacks, but no collision response.
    pub fn new_kinematic() -> Body {
        Body {
            pointer: unsafe { chip::cpBodyNewKinematic() }
        }
    }

    /// Create a new static Body. From the Chipmunk docs:
    ///
    /// > A static body is a body that never (or rarely) moves.
    /// >
    /// > If you move a static body, you must call one of the `cpSpaceReindex*()` functions.
    /// > Chipmunk uses this information to optimize the collision detection.
    /// > Static bodies do not produce collision callbacks when colliding with other static bodies.
    pub fn new_static() -> Body {
        Body {
            pointer: unsafe { chip::cpBodyNewStatic() }
        }
    }

    /// Return a raw pointer to the internal `cpBody`. Use with caution.
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const chip::cpBody {
        self.pointer as *const chip::cpBody
    }

    /// Return a raw mutable pointer to the internal `cpBody`. Use with caution.
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut chip::cpBody {
        self.pointer
    }

    /// Returns the angle of the body in radians.
    pub fn angle_rad(&self) -> f64 {
        unsafe {
            chip::cpBodyGetAngle(self.as_ptr())
        }
    }

    /// Sets the angle of the body in radians.
    pub fn set_angle_rad(&mut self, angle: f64) {
        unsafe {
            chip::cpBodySetAngle(self.as_mut_ptr(), angle);
        }
    }

    /// Returns the angle of the body in degrees.
    pub fn angle_deg(&self) -> f64 {
        self.angle_rad().to_degrees()
    }

    /// Sets the angle of the body in degrees.
    pub fn set_angle_deg(&mut self, angle: f64) {
        self.set_angle_rad(angle.to_radians());
    }

    /// Returns the angular velocity in radians / second.
    pub fn angular_velocity_rad(&self) -> f64 {
        unsafe {
            chip::cpBodyGetAngularVelocity(self.as_ptr())
        }
    }

    /// Sets the angular velocity in radians / second.
    pub fn set_angular_velocity_rad(&mut self, ang_vel: f64) {
        unsafe {
            chip::cpBodySetAngularVelocity(self.as_mut_ptr(), ang_vel)
        }
    }

    /// Returns the angular velocity in degrees / second.
    pub fn angular_velocity_deg(&self) -> f64 {
        self.angular_velocity_rad().to_degrees()
    }

    /// Sets the angular velocity in degrees / second.
    pub fn set_angular_velocity_deg(&mut self, ang_vel: f64) {
        self.set_angular_velocity_rad(ang_vel.to_radians())
    }

    /// Returns the location of the center of gravity in local coordinates.
    pub fn center_of_gravity(&self) -> (f64, f64) {
        unsafe {
            chip::cpBodyGetCenterOfGravity(self.as_ptr()).into()
        }
    }

    /// Sets the position of the center of gravity on this body.
    ///
    /// The center of gravity is in local coordinates.
    pub fn set_center_of_gravity(&mut self, x: f64, y: f64) {
        unsafe {
            chip::cpBodySetCenterOfGravity(self.as_mut_ptr(), cpv(x, y));
        }
    }

    /// Returns the force acting on the body.
    pub fn force(&self) -> (f64, f64) {
        unsafe {
            chip::cpBodyGetForce(self.as_ptr()).into()
        }
    }

    /// Sets the force applied to the body.
    ///
    /// The force is not reset during each physics step.  If you want
    /// to reset the force, you must do that manually.
    pub fn set_force(&mut self, x: f64, y: f64) {
        unsafe {
            chip::cpBodySetForce(self.as_mut_ptr(), cpv(x, y));
        }
    }

    /// Returns the mass of the body.
    pub fn mass(&self) -> f64 {
        unsafe {
            chip::cpBodyGetMass(self.as_ptr())
        }
    }

    /// Sets the mass of the body.
    pub fn set_mass(&mut self, mass: f64) {
        unsafe {
            chip::cpBodySetMass(self.as_mut_ptr(), mass);
        }
    }

    /// Returns the moment of inertia of the body.
    pub fn moment(&self) -> f64 {
        unsafe {
            chip::cpBodyGetMoment(self.as_ptr())
        }
    }

    /// Sets the moment of inertia of the body.
    ///
    /// The moment of inertia is the rotational mass of the body.
    pub fn set_moment(&mut self, moment: f64) {
        unsafe {
            chip::cpBodySetMoment(self.as_mut_ptr(), moment);
        }
    }

    /// Returns the position of the body in world space.
    pub fn position(&self) -> (f64, f64) {
        unsafe {
            chip::cpBodyGetPosition(self.as_ptr()).into()
        }
    }

    /// Sets the position of the body in world coordinates.
    pub fn set_position(&mut self, x: f64, y: f64) {
        unsafe {
            chip::cpBodySetPosition(self.as_mut_ptr(), cpv(x, y));
        }
    }

    /// Returns the torque acting on the body.
    pub fn torque(&self) -> f64 {
        unsafe {
            chip::cpBodyGetTorque(self.as_ptr())
        }
    }

    /// Sets the torque applied to the body.
    pub fn set_torque(&mut self, torque: f64) {
        unsafe {
            chip::cpBodySetTorque(self.as_mut_ptr(), torque);
        }
    }

    /// Returns the velocity of the body.
    pub fn velocity(&self) -> (f64, f64) {
        unsafe {
            chip::cpBodyGetVelocity(self.as_ptr()).into()
        }
    }

    /// Directly sets the velocity of the body.
    pub fn set_velocity(&mut self, vx: f64, vy: f64) {
        unsafe {
            chip::cpBodySetVelocity(self.as_mut_ptr(), cpv(vx, vy));
        }
    }
}
