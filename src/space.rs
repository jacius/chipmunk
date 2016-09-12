use chip;
use chip::cpVect;

use super::body::BodyHandle;
use super::shape::ShapeHandle;


pub struct Space {
    pointer: *mut chip::cpSpace,
    bodies: Vec<BodyHandle>,
    shapes: Vec<ShapeHandle>,
}

impl Drop for Space {
    fn drop(&mut self) {
        unsafe {
            chip::cpSpaceFree(self.pointer);
        }
    }
}

use std::fmt;
impl fmt::Debug for Space {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Space")
            .field("gravity", &self.gravity())
            .field("damping", &self.damping())
            .finish()
    }
}

impl Space {
    pub fn new() -> Space {
        Space {
            pointer: unsafe { chip::cpSpaceNew() },
            bodies: Vec::new(),
            shapes: Vec::new(),
        }
    }

    /// Return a raw pointer to the internal `cpSpace`. Use with caution.
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const chip::cpSpace {
        self.pointer as *const chip::cpSpace
    }

    /// Return a raw mutable pointer to the internal `cpSpace`. Use with caution.
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut chip::cpSpace {
        self.pointer
    }

    pub fn add_body(&mut self, body: &mut BodyHandle) {
        self.bodies.push(body.clone());
        unsafe {
            chip::cpSpaceAddBody(self.as_mut_ptr(), body.write().as_mut_ptr());
        }
    }

    pub fn add_shape(&mut self, shape: &mut ShapeHandle) {
        self.shapes.push(shape.clone());
        unsafe {
            chip::cpSpaceAddShape(self.as_mut_ptr(), shape.write().as_mut_ptr());
        }
    }

    pub fn remove_body(&mut self, body: &mut BodyHandle) {
        unsafe {
            let pos = self.bodies.iter_mut().position(|b| b.read().as_ptr() == body.read().as_ptr());
            if let Some(pos) = pos {
                self.bodies.remove(pos);
            }
            chip::cpSpaceRemoveBody(self.as_mut_ptr(), body.write().as_mut_ptr());
        }
    }

    pub fn remove_shape(&mut self, shape: &mut ShapeHandle) {
        unsafe {
            let pos = self.shapes.iter_mut().position(|s| s.read().as_ptr() == shape.read().as_ptr());
            if let Some(pos) = pos {
                self.shapes.remove(pos);
            }
            chip::cpSpaceRemoveShape(self.as_mut_ptr(), shape.write().as_mut_ptr());
        }
    }

    /// Step the simulation forward by the given number of seconds.
    pub fn step(&mut self, dt: f64) {
        unsafe {
            chip::cpSpaceStep(self.as_mut_ptr(), dt);
        }
    }

    /// Returns the global gravity for all rigid bodies in this space.
    ///
    /// Default is `(0.0, 0.0)`.
    pub fn gravity(&self) -> (f64, f64) {
        unsafe {
            chip::cpSpaceGetGravity(self.as_ptr()).into()
        }
    }

    /// Sets the global gravity for all rigid bodies in this space.
    ///
    /// Default is `<0, 0>` (no gravity).
    pub fn set_gravity(&mut self, grav: (f64, f64)) {
        unsafe {
            chip::cpSpaceSetGravity(self.as_mut_ptr(), cpVect::from(grav));
        }
    }

    /// Returns the global velocity damping.
    ///
    /// Defaults to 1.0 (no damping).
    ///
    /// This value is the fraction of velocity a body should have after 1
    /// second.  A value of 0.9 would mean that each second, a body would have
    /// 90% of the velocity it had the previous second.
    pub fn damping(&self) -> f64 {
        unsafe {
            chip::cpSpaceGetDamping(self.as_ptr())
        }
    }

    /// Sets the global velocity damping.
    ///
    /// Defaults to 1.0 (no damping).
    ///
    /// See `damping()` for a description of this property.
    pub fn set_damping(&mut self, damping: f64) {
        unsafe {
            chip::cpSpaceSetDamping(self.as_mut_ptr(), damping);
        }
    }

    /// Returns the amount of encouraged penetration between colliding shapes.
    ///
    /// This is used to reduce oscillating contacts and keep the collision cache
    /// warm.  Defaults to 1.0.
    ///
    /// If you have poor simulation quality, increase this number as much as
    /// possible without allowing visible amounts of overlap.
    pub fn collision_slop(&self) -> f64 {
        unsafe {
            chip::cpSpaceGetCollisionSlop(self.as_ptr())
        }
    }

    /// Sets the amount of encouraged penetration between colliding shapes.
    ///
    /// See `collision_slop()` for a description of the property.
    pub fn set_collision_slop(&mut self, slop: f64) {
        unsafe {
            chip::cpSpaceSetCollisionSlop(self.as_mut_ptr(), slop);
        }
    }

    /// Returns how fast overlapping shapes are pushed apart.
    ///
    /// Defaults to `pow(1 - 0.1, 60)` meaning that chipmunk fixes 10% of
    /// overlap each frame at 60Hz.
    pub fn collision_bias(&self) -> f64 {
        unsafe {
            chip::cpSpaceGetCollisionBias(self.as_ptr())
        }
    }

    /// Sets how fast overlapping shapes are pushed apart.
    ///
    /// Defaults to pow(1.0 - 0.1, 60) meaning that chipmunk fixes
    /// 10% of overlap each frame at 60Hz.
    pub fn set_collision_bias(&mut self, bias: f64) {
        unsafe {
            chip::cpSpaceSetCollisionBias(self.as_mut_ptr(), bias);
        }
    }

    /// Returns the number of frames that contact information should remain.
    ///
    /// Defaults to 3.
    pub fn collision_persistence(&self) -> u32 {
        unsafe {
            chip::cpSpaceGetCollisionPersistence(self.as_ptr())
        }
    }

    /// Sets the number of frames that contact information should remain.
    ///
    /// Default is 3.
    pub fn set_collision_persistence(&mut self, persistence: u32) {
        unsafe {
            chip::cpSpaceSetCollisionPersistence(self.as_mut_ptr(), persistence);
        }
    }

    /// Returns the minimum speed to be considered idle.
    /// Defaults to 0.0.
    pub fn idle_speed_threshold(&self) -> f64 {
        unsafe {
            chip::cpSpaceGetIdleSpeedThreshold(self.as_ptr())
        }
    }

    /// Sets the minimum speed to be considered idle.
    ///
    /// Default is 0.0.
    pub fn set_idle_speed_threshold(&mut self, threshold: f64) {
        unsafe {
            chip::cpSpaceSetIdleSpeedThreshold(self.as_mut_ptr(), threshold);
        }
    }

    /// Gets the number of solver passes that the engine uses.
    ///
    /// Defaults to 10.
    ///
    /// Fewer iterations means less CPU usage, but lower quality physics.
    pub fn iterations(&self) -> i32 {
        unsafe {
            chip::cpSpaceGetIterations(self.as_ptr())
        }
    }

    /// Sets the number of solver passes that the engine uses.
    ///
    /// Default is 10.  Fewer iterations means less CPU usage but
    /// lower quality physics.
    pub fn set_iterations(&mut self, iterations: i32) {
        unsafe {
            chip::cpSpaceSetIterations(self.as_mut_ptr(), iterations);
        }
    }

    /// Update the collision detection info for the static shapes in the space.
    pub fn reindex_static(&mut self) {
        unsafe {
            chip::cpSpaceReindexStatic(self.as_mut_ptr())
        }
    }

    /// Update the collision detection data for a specific shape in the space.
    pub fn reindex_shape(&mut self, shape: &mut ShapeHandle) {
        unsafe {
            chip::cpSpaceReindexShape(self.as_mut_ptr(), shape.write().as_mut_ptr())
        }
    }

    /// Update the collision detection data for all shapes attached to a body.
    pub fn reindex_shapes_for_body(&mut self, body: &mut BodyHandle) {
        unsafe {
            chip::cpSpaceReindexShapesForBody(self.as_mut_ptr(), body.write().as_mut_ptr())
        }
    }

    /// Returns the ellapsed time before a group of idle bodies is put to sleep.
    ///
    /// Defaults to infinity (no sleeping).
    pub fn sleep_time_threshold(&self) -> f64 {
        unsafe {
            chip::cpSpaceGetSleepTimeThreshold(self.as_ptr())
        }
    }

    /// Sets the ellapsed time before a group of idle bodies is put to sleep.
    ///
    /// Unless this method is called, this property will default to infinity
    /// which disables sleeping.
    pub fn set_sleep_time_threshold(&mut self, threshold: f64) {
        unsafe {
            chip::cpSpaceSetSleepTimeThreshold(self.as_mut_ptr(), threshold);
        }
    }

    /// Switch the space to use a spatial hash as its spatial index.
    pub fn use_spatial_hash(&mut self, dim: f64, count: u32) {
        unsafe {
            chip::cpSpaceUseSpatialHash(self.as_mut_ptr(), dim, count as i32)
        }
    }
}
