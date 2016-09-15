//! Collision shapes.

use chip;
use std::fmt;

use super::CpVect;
use super::body::{Body, BodyHandle};
use super::handle::{Handle, WeakHandle};


/// ShapeHandle provides several shortcuts for creating a new Shape and putting it in a Handle.
///
/// - `ShapeHandle::new_circle(body, radius, offset)` is the same `ShapeHandle::from(Shape::new_circle(body, radius, offset))`
/// - `ShapeHandle::new_segment(body, a, b, radius)` is the same as `ShapeHandle::from(Shape::new_segment(body, a, b, radius))`
/// - `ShapeHandle::new_poly_raw(body, verts, radius)` is the same as `ShapeHandle::from(Shape::new_poly_raw(body, verts, radius))`
/// - `ShapeHandle::new_box(body, width, height, radius)` is the same as `ShapeHandle::from(Shape::new_box(body, width, height, radius))`
pub type ShapeHandle = Handle<Shape>;

impl ShapeHandle {
    pub fn new_circle<V>(body: &mut BodyHandle, radius: f64, offset: V) -> ShapeHandle
        where CpVect: From<V> {
        ShapeHandle::from(Shape::new_circle(body, radius, offset))
    }

    pub fn new_segment<V1, V2>(body: &mut BodyHandle, a: V1, b: V2, radius: f64) -> ShapeHandle
        where CpVect: From<V1>, CpVect: From<V2> {
        ShapeHandle::from(Shape::new_segment(body, a, b, radius))
    }

    pub fn new_poly_raw<'a, V: 'a>(body: &mut BodyHandle, verts: &'a [V], radius: f64) -> ShapeHandle
        where CpVect: From<&'a V> {
        ShapeHandle::from(Shape::new_poly_raw(body, verts, radius))
    }

    pub fn new_box(body: &mut BodyHandle, width: f64, height: f64, radius: f64) -> ShapeHandle {
        ShapeHandle::from(Shape::new_box(body, width, height, radius))
    }
}


/// Collision shape. Wrapper around `cpShape`.
///
/// Shapes define the collision shape of a Body.
/// A Body can own many Shapes.
/// If the Shape's mass is greater than zero,
/// the Body's mass, moment of inertia, etc. are automatically calculated based on its Shapes.
///
/// Chipmunk supports three kinds of shape: circles, line segments, and convex polygons.
///
/// The three kinds of shapes have some attributes is common, such as mass, friction, and elasticity.
/// These attributes are accessed via methods of the `Shape` enum.
///
/// Each kind of shape also has some unique attributes.
/// These attributes are accessed via methods of the specific shape struct: `CircleShape`, `SegmentShape`, or `PolyShape`.
#[derive(Debug)]
pub enum Shape {
    Circle(CircleShape),
    Segment(SegmentShape),
    Poly(PolyShape)
}

impl Shape {
    /// Creates a new `Shape::Circle`
    /// with the given radius and offset (in local coordinates).
    /// The new Shape will be automatically added to the Body when the Shape is added to a Space.
    pub fn new_circle<V>(body: &mut BodyHandle, radius: f64, offset: V) -> Shape
        where CpVect: From<V> {
        let pointer = unsafe {
            chip::cpCircleShapeNew(
                body.write().unwrap().as_mut_ptr(),
                radius,
                CpVect::from(offset).into()
            )
        };

        Shape::Circle(CircleShape{
            pointer: pointer,
            _attached_body: body.downgrade(),
        })
    }

    /// Creates a new `Shape::Segment`
    /// going from point `a` to point `b` (in local coordinates),
    /// with the given radius (i.e. thickness).
    /// The new Shape will be automatically added to the Body when the Shape is added to a Space.
    pub fn new_segment<V1, V2>(body: &mut BodyHandle, a: V1, b: V2, radius: f64) -> Shape
        where CpVect: From<V1>, CpVect: From<V2> {
        let pointer = unsafe {
            chip::cpSegmentShapeNew(
                body.write().unwrap().as_mut_ptr(),
                CpVect::from(a).into(),
                CpVect::from(b).into(),
                radius
            )
        };

        Shape::Segment(SegmentShape {
            pointer: pointer,
            _attached_body: body.downgrade(),
        })
    }

    /// Creates a new `Shape::Poly`
    /// with the given vertices (points in local coordinates)
    /// and radius (i.e. thickness).
    /// The vertices must be convex with a counter-clockwise winding.
    /// The new Shape will be automatically added to the Body when the Shape is added to a Space.
    pub fn new_poly_raw<'a, V: 'a>(body: &mut BodyHandle, verts: &'a [V], radius: f64) -> Shape
        where CpVect: From<&'a V> {
        let verts = verts.iter().map(|v| CpVect::from(v).into()).collect::<Vec<chip::cpVect>>();

        let pointer = unsafe { chip::cpPolyShapeAlloc() };
        unsafe {
            let _ = chip::cpPolyShapeInitRaw(
                pointer,
                body.write().unwrap().as_mut_ptr(),
                verts.len() as i32,
                (&verts).as_ptr() as *const chip::cpVect,
                radius
            );
        }

        Shape::Poly(PolyShape{
            pointer: pointer as *mut chip::cpShape,
            _attached_body: body.downgrade(),
        })
    }

    /// Creates a new `Shape::Poly`
    /// that forms a box (rectangle) with the given width and height,
    /// and has the given radius (i.e. thickness).
    /// There is no box shape type; this is just an easy way to create a rectangular `Shape::Poly`.
    /// The new Shape will be automatically added to the Body when the Shape is added to a Space.
    pub fn new_box(body: &mut BodyHandle, width: f64, height: f64, radius: f64) -> Shape {
        let pointer = unsafe {
            chip::cpBoxShapeNew(
                body.write().unwrap().as_mut_ptr(),
                width,
                height,
                radius
            )
        };

        Shape::Poly(PolyShape{
            pointer: pointer,
            _attached_body: body.downgrade(),
        })
    }


    /// Returns a raw pointer to the internal `cpShape`. Use with caution.
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const chip::cpShape {
        (match self {
            &Shape::Circle(ref shape) => shape.pointer,
            &Shape::Segment(ref shape) => shape.pointer,
            &Shape::Poly(ref shape) => shape.pointer
        }) as *const chip::cpShape
    }

    /// Returns a raw mutable pointer to the internal `cpShape`. Use with caution.
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut chip::cpShape {
        match self {
            &mut Shape::Circle(ref shape) => shape.pointer,
            &mut Shape::Segment(ref shape) => shape.pointer,
            &mut Shape::Poly(ref shape) => shape.pointer
        }
    }


    /// Return the calculated area of the Shape.
    pub fn area(&self) -> f64 {
        unsafe {
            chip::cpShapeGetArea(self.as_ptr())
        }
    }

    /// Return the center of gravity for the Shape (in local coordinates).
    pub fn center_of_gravity(&self) -> CpVect {
        unsafe {
            chip::cpShapeGetCenterOfGravity(self.as_ptr()).into()
        }
    }

    /// Returns the density of the Shape, i.e. its mass divided by its area.
    pub fn density(&self) -> f64 {
        unsafe {
            chip::cpShapeGetDensity(self.as_ptr())
        }
    }

    /// Sets the density of the Shape.
    /// This actually sets the Shape's mass, calculated using the Shape's area.
    /// This causes the Shape's Body to recalculate the Body's total mass.
    pub fn set_density(&mut self, density: f64) {
        unsafe {
            chip::cpShapeSetDensity(self.as_mut_ptr(), density);
        }
    }

    /// Returns the elasticity of the Shape.
    pub fn elasticity(&self) -> f64 {
        unsafe {
            chip::cpShapeGetElasticity(self.as_ptr())
        }
    }

    /// Sets the elasticity of the Shape.
    /// Elasticity must be >= 0.
    /// High values are more "bouncy" than low values.
    ///
    /// # Panics
    ///
    /// This function panics if elasticity is less than zero.
    pub fn set_elasticity(&mut self, elasticity: f64) {
        // Assert in Rust land instead of C, so users get a better backtrace.
        // The assertion message in C says "positive and non-zero", but that
        // does not match the assertion being performed.
        // TODO: Should the function return a Result instead of panicking?
        assert!(elasticity >= 0.0, "Elasticity must be non-negative.");
        unsafe {
            chip::cpShapeSetElasticity(self.as_mut_ptr(), elasticity);
        }
    }

    /// Return the friction of the Shape.
    pub fn friction(&self) -> f64 {
        unsafe {
            chip::cpShapeGetFriction(self.as_ptr())
        }
    }

    /// Set the friction of the Shape.
    /// Friction must be >= 0.
    /// High values are more "rough" (less "slippery") than low values.
    ///
    /// # Panics
    ///
    /// This function panics if friction is less than zero.
    pub fn set_friction(&mut self, friction: f64) {
        // Assert in Rust land instead of C, so users get a better backtrace.
        // The assertion message in C says "positive and non-zero", but that
        // does not match the assertion being performed.
        // TODO: Should the function return a Result instead of panicking?
        assert!(friction >= 0.0, "Friction must be non-negative.");
        unsafe {
            chip::cpShapeSetFriction(self.as_mut_ptr(), friction);
        }
    }

    /// Return the mass of the Shape.
    pub fn mass(&self) -> f64 {
        unsafe {
            chip::cpShapeGetMass(self.as_ptr())
        }
    }

    /// Set the mass of the Shape.
    /// This causes the Shape's Body to recalculate the Body's total mass.
    pub fn set_mass(&mut self, mass: f64) {
        unsafe {
            chip::cpShapeSetMass(self.as_mut_ptr(), mass);
        }
    }

    /// Return the calculated moment of inertia of the Shape.
    pub fn moment(&self) -> f64 {
        unsafe {
            chip::cpShapeGetMoment(self.as_ptr())
        }
    }

    /// Return true if the Shape is a sensor.
    pub fn is_sensor(&self) -> bool {
        unsafe {
            1 == chip::cpShapeGetSensor(self.as_ptr())
        }
    }

    /// Set whether the Shape is a sensor or not.
    /// A sensor is a Shape that dosn't participate in collisions, but
    /// still calls callbacks.
    pub fn set_is_sensor(&mut self, is_sensor: bool) {
        unsafe {
            let is_sensor = if is_sensor {1} else {0};
            chip::cpShapeSetSensor(self.as_mut_ptr(), is_sensor);
        }
    }

    /// Return the surface velocity of the Shape.
    pub fn surface_velocity(&self) -> CpVect {
        unsafe {
            chip::cpShapeGetSurfaceVelocity(self.as_ptr()).into()
        }
    }

    /// Set the surface velocity of the Shape.
    /// Useful for creating conveyor belts or players that move around.
    pub fn set_surface_velocity(&mut self, surface_velocity: (f64, f64)) {
        unsafe {
            chip::cpShapeSetSurfaceVelocity(
                self.as_mut_ptr(),
                CpVect::from(surface_velocity).into()
            );
        }
    }
}


/// Circle collision shape. Wrapper around `cpShape` / `cpCircleShape`.
///
/// Use `Shape::new_circle` to create a `CircleShape`.
pub struct CircleShape {
    pointer: *mut chip::cpShape,
    _attached_body: WeakHandle<Body>,
}

impl CircleShape {
    /// Returns the offset of the circle, in local coordinates.
    pub fn offset(&self) -> CpVect {
        unsafe {
            chip::cpCircleShapeGetOffset(self.pointer).into()
        }
    }

    /// Returns the radius of the circle.
    pub fn radius(&self) -> f64 {
        unsafe {
            chip::cpCircleShapeGetRadius(self.pointer)
        }
    }
}

impl Drop for CircleShape {
    fn drop(&mut self) {
        unsafe { chip::cpShapeDestroy(self.pointer); }
    }
}

impl fmt::Debug for CircleShape {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("CircleShape")
            .field("radius", &self.radius())
            .field("offset", &self.offset())
            .finish()
    }
}


/// Line segment collision shape. Wrapper around `cpShape` / `cpSegmentShape`.
///
/// Use `Shape::new_segment` to create a `SegmentShape`.
pub struct SegmentShape {
    pointer: *mut chip::cpShape,
    _attached_body: WeakHandle<Body>,
}

impl SegmentShape {
    /// Return the first point in the segment, in local coordinates.
    pub fn a(&self) -> CpVect {
        unsafe {
            chip::cpSegmentShapeGetA(self.pointer).into()
        }
    }

    /// Return the second point in the segment, in local coordinates.
    pub fn b(&self) -> CpVect {
        unsafe {
            chip::cpSegmentShapeGetB(self.pointer).into()
        }
    }

    /// Return the normal vector of this segment, in local coordinates.
    pub fn normal(&self) -> CpVect {
        unsafe {
            chip::cpSegmentShapeGetNormal(self.pointer).into()
        }
    }

    /// Return the radius or "thickness" of the line segment.
    pub fn radius(&self) -> f64 {
        unsafe {
            chip::cpSegmentShapeGetRadius(self.pointer)
        }
    }
}

impl Drop for SegmentShape {
    fn drop(&mut self) {
        unsafe { chip::cpShapeDestroy(self.pointer); }
    }
}

impl fmt::Debug for SegmentShape {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("SegmentShape")
            .field("a", &self.a())
            .field("b", &self.b())
            .field("radius", &self.radius())
            .field("normal", &self.normal())
            .finish()
    }
}


/// Convex polygon collision shape. Wrapper around `cpShape` / `cpPolyShape`.
///
/// Use `Shape::new_poly_raw` to create a `PolyShape`.
pub struct PolyShape {
    pointer: *mut chip::cpShape,
    _attached_body: WeakHandle<Body>,
}

impl PolyShape {
    /// Return the number of vertices in the polygon.
    pub fn count(&self) -> usize {
        unsafe {
            chip::cpPolyShapeGetCount(self.pointer) as usize
        }
    }

    /// Return the radius or "thickness" of the polygon's edges.
    pub fn radius(&self) -> f64 {
        unsafe {
            chip::cpPolyShapeGetRadius(self.pointer)
        }
    }

    /// Return the i-th vertex in the shape.
    pub fn vert(&self, i: usize)  -> CpVect {
        unsafe {
            chip::cpPolyShapeGetVert(self.pointer, i as i32).into()
        }
    }
}

impl Drop for PolyShape {
    fn drop(&mut self) {
        unsafe { chip::cpShapeDestroy(self.pointer); }
    }
}

impl fmt::Debug for PolyShape {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PolyShape")
            .field("count", &self.count())
            .field("radius", &self.radius())
            .finish()
    }
}
