extern crate chipmunk_sys as chip;

#[cfg(feature="cgmath")]
extern crate cgmath;

#[cfg(feature="nalgebra")]
extern crate nalgebra;

pub mod arbiter;
pub mod body;
pub mod handle;
pub mod shape;
pub mod space;
pub mod util;
mod cp_vect;

pub use self::arbiter::{Arbiter, ContactPoint, ContactPointSet};
pub use self::body::{Body, BodyHandle};
pub use self::handle::{Handle, WeakHandle};
pub use self::shape::{Shape, ShapeHandle, CircleShape, SegmentShape, PolyShape};
pub use self::space::Space;
pub use self::util::*;
pub use self::cp_vect::CpVect;
