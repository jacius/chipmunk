use chip::cpVect;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(feature="cgmath")]
use cgmath;

#[cfg(feature="nalgebra")]
use nalgebra;

/// Two-dimensional vector.
///
/// `CpVect` is a 2D vector type used by Chipmunk. It is suitable for simple 2D
/// vector math, but for more advanced situations you should use a linear
/// algebra crate such as [cgmath](https://crates.io/crates/cgmath)
/// or [nalgebra](https://crates.io/crates/nalgebra).
///
/// # Conversion
///
/// To make it easy to use the chipmunk crate with other crates, the `CpVect`
/// supports conversion to and from a variety of similar types, using the
/// standard `From` and `Into` traits.
///
/// You can always convert `CpVect` to and from `(f64, f64)` and `[f64; 2]`.
/// You can also convert to and from `(f32, f32)` and `[f32; 2]`,
/// but it is recommended to use `f64` if possible, to avoid loss of precision.
///
/// ```
/// # use chipmunk::CpVect;
/// let cpvect = CpVect::new(1.2, 3.4);
///
/// let cpvect_from_tuple = CpVect::from((1.2, 3.4));
/// let cpvect_into_tuple: (f64, f64) = cpvect.into();
///
/// let tuple_from_cpvect = <(f64, f64)>::from(cpvect);
/// let tuple_into_cpvect: CpVect = (1.2, 3.4).into();
///
/// let cpvect_from_array = CpVect::from([1.2, 3.4]);
/// let cpvect_into_array: [f64; 2] = cpvect.into();
///
/// let array_from_cpvect = <[f64; 2]>::from(cpvect);
/// let array_into_cpvect: CpVect = [1.2, 3.4].into();
/// ```
///
/// If you compile the chipmunk crate with the "cgmath" feature, you can also
/// convert `CpVect` to and from `Vector2<f64>` and `Vector2<f32>` from
/// the [cgmath](https://crates.io/crates/cgmath) crate:
///
/// ```rust
/// # // Fallback main function in case cgmath is not available:
/// # #[cfg(not(feature="cgmath"))]
/// # fn main(){}
//
/// # #[cfg(feature="cgmath")]
/// extern crate cgmath;
/// extern crate chipmunk;
/// # #[cfg(feature="cgmath")]
/// use cgmath::Vector2;
/// use chipmunk::CpVect;
///
/// # #[cfg(feature="cgmath")]
/// fn main() {
///     let cpvect = CpVect::new(1.2, 3.4);
///     let vector2 = Vector2::new(1.2, 3.4);
///
///     let cpvect_from_vector2 = CpVect::from(vector2);
///     let cpvect_into_vector2: Vector2<f64> = cpvect.into();
///
///     let vector2_from_cpvect = Vector2::<f64>::from(cpvect);
///     let vector2_into_cpvect: CpVect = vector2.into();
/// }
/// ```
///
/// If you compile the chipmunk crate with the "nalgebra" feature,
/// you can also convert `CpVect` to and from `Point2<f64>`, `Point2<f32>`,
/// `Vector2<f64>`, and `Vector2<f32>` from
/// the [nalgebra](https://crates.io/crates/nalgebra) crate:
///
/// ```rust
/// # // Fallback main function in case nalgebra is not available:
/// # #[cfg(not(feature="nalgebra"))]
/// # fn main(){}
//
/// extern crate chipmunk;
/// # #[cfg(feature="nalgebra")]
/// extern crate nalgebra;
/// use chipmunk::CpVect;
/// # #[cfg(feature="nalgebra")]
/// use nalgebra::{Point2, Vector2};
///
/// # #[cfg(feature="nalgebra")]
/// fn main() {
///     let cpvect = CpVect::new(1.2, 3.4);
///     let point2 = Point2::new(1.2, 3.4);
///     let vector2 = Vector2::new(1.2, 3.4);
///
///     let cpvect_from_point2 = CpVect::from(point2);
///     let cpvect_into_point2: Point2<f64> = cpvect.into();
///
///     let point2_from_cpvect = Point2::<f64>::from(cpvect);
///     let point2_into_cpvect: CpVect = point2.into();
///
///     let cpvect_from_vector2 = CpVect::from(vector2);
///     let cpvect_into_vector2: Vector2<f64> = cpvect.into();
///
///     let vector2_from_cpvect = Vector2::<f64>::from(cpvect);
///     let vector2_into_cpvect: CpVect = vector2.into();
/// }
/// ```
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CpVect {
    pub x: f64,
    pub y: f64,
}

impl CpVect {
    pub fn new(x: f64, y: f64) -> CpVect {
        CpVect { x: x, y: y }
    }

    /// Returns the unit length vector for the given angle (in radians).
    pub fn new_for_angle(a: f64) -> CpVect {
        CpVect {
            x: a.cos(),
            y: a.sin(),
        }
    }

    /// Clamp this vector so its length does not exceed `len`.
    pub fn clamp(self, len: f64) -> CpVect {
        if self.dot(self) > (len * len) {
            self.normalize() * len
        } else {
            self
        }
    }

    /// Returns the cross product analog of this vector and other.
    ///
    /// The cross product of 2D vectors results in a 3D vector with only a z component.
    /// This function returns the magnitude of the z value.
    pub fn cross(self, other: CpVect) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// Returns the distance between this vector and other.
    pub fn dist(self, other: CpVect) -> f64 {
        (self - other).length()
    }

    /// Returns the squared distance between this vector and other.
    /// Faster than `dist` when you only need to compare distances.
    pub fn distsq(self, other: CpVect) -> f64 {
        (self - other).lengthsq()
    }

    /// Returns the dot product of this vector and other.
    #[inline]
    pub fn dot(self, other: CpVect) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the length of this vector.
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Returns the squared length of this vector.
    /// Faster than `length` when you only need to compare lengths.
    pub fn lengthsq(self) -> f64 {
        self.dot(self)
    }

    /// Linearly interpolate between this vector and other.
    pub fn lerp(self, other: CpVect, t: f64) -> CpVect {
        (self * (1.0 - t)) + (other * t)
    }

    /// Linearly interpolate between this vector towards other by distance `d`.
    pub fn lerp_const(self, other: CpVect, d: f64) -> CpVect {
        self + (other - self).clamp(d)
    }

    /// Returns true if the distance between this vector and other is less than dist.
    pub fn near(self, other: CpVect, dist: f64) -> bool {
        self.distsq(other) < (dist * dist)
    }

    /// Returns a normalized copy of this vector.
    #[inline]
    pub fn normalize(self) -> CpVect {
        // Neat trick to avoid dividing by 0.
        self / (self.length() + ::std::f64::MIN)
    }

    /// Returns a perpendicular vector. (90 degree rotation)
    pub fn perp(self) -> CpVect {
        CpVect {
            x: -self.y,
            y: self.x,
        }
    }

    /// Returns a perpendicular vector. (-90 degree rotation)
    pub fn rperp(self) -> CpVect {
        CpVect {
            x: self.y,
            y: -self.x,
        }
    }

    /// Returns the vector projection of this vector onto another vector.
    pub fn project(self, other: CpVect) -> CpVect {
        self * (self.dot(other) / other.dot(other))
    }

    /// Uses complex number multiplication to rotate this vector by another vector.
    /// Scaling will occur if this vector is not a unit vector.
    pub fn rotate(self, other: CpVect) -> CpVect {
        CpVect {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }

    /// Inverse of `rotate`.
    pub fn unrotate(self, other: CpVect) -> CpVect {
        CpVect {
            x: self.x * other.x + self.y * other.y,
            y: self.y * other.x - self.x - other.y,
        }
    }

    /// Spherical linearly interpolate between this vector and other.
    pub fn slerp(self, other: CpVect, t: f64) -> CpVect {
        let dot = self.normalize().dot(other.normalize());
        let omega = dot.max(-1.0).min(1.0);

        if omega < 1e-3 {
            // If the angle between two vectors is very small,
            // lerp instead to avoid precision issues.
            self.lerp(other, t)
        } else {
            let denom = 1.0 / omega.sin();
            (self * ((1.0 - t) * omega).sin() * denom) + (other * (t * omega).sin() * denom)
        }
    }

    /// Spherical linearly interpolate between this vector towards other
    /// by no more than angle `a` radians.
    pub fn slerp_const(self, other: CpVect, a: f64) -> CpVect {
        let dot = self.normalize().dot(other.normalize());
        let omega = dot.max(-1.0).min(1.0);
        self.slerp(other, a.min(omega) / omega)
    }

    /// Returns the angular direction this vector is pointing in (in radians).
    pub fn to_angle(self) -> f64 {
        self.y.atan2(self.x)
    }
}


impl Default for CpVect {
    fn default() -> CpVect {
        CpVect { x: 0.0, y: 0.0 }
    }
}


// Converting CpVect to and from cpVect.
#[doc(hidden)]
impl From<CpVect> for cpVect {
    fn from(vect: CpVect) -> cpVect {
        cpVect {
            x: vect.x,
            y: vect.y,
        }
    }
}
#[doc(hidden)]
impl From<cpVect> for CpVect {
    fn from(cpv: cpVect) -> CpVect {
        CpVect {
            x: cpv.x,
            y: cpv.y,
        }
    }
}

/// If chipmunk is compiled with the "cgmath" feature, `CpVect` can be
/// converted to/from `cgmath::Vector2<f64>`.
#[cfg(feature="cgmath")]
impl From<cgmath::Vector2<f64>> for CpVect {
    fn from(v: cgmath::Vector2<f64>) -> CpVect {
        CpVect { x: v.x, y: v.y }
    }
}
#[cfg(feature="cgmath")]
impl From<CpVect> for cgmath::Vector2<f64> {
    fn from(v: CpVect) -> cgmath::Vector2<f64> {
        cgmath::Vector2 { x: v.x, y: v.y }
    }
}
/// If chipmunk is compiled with the "cgmath" feature, `CpVect` can be
/// converted to/from `cgmath::Vector2<f32>`.
/// Be aware that converting from `f64` to `f32` may result in a loss of precision.
#[cfg(feature="cgmath")]
impl From<cgmath::Vector2<f32>> for CpVect {
    fn from(v: cgmath::Vector2<f32>) -> CpVect {
        CpVect {
            x: v.x as f64,
            y: v.y as f64,
        }
    }
}
#[cfg(feature="cgmath")]
impl From<CpVect> for cgmath::Vector2<f32> {
    fn from(v: CpVect) -> cgmath::Vector2<f32> {
        cgmath::Vector2 {
            x: v.x as f32,
            y: v.y as f32,
        }
    }
}

/// If chipmunk is compiled with the "nalgebra" feature, `CpVect` can be
/// converted to/from `nalgebra::Vector2<f64>`.
#[cfg(feature="nalgebra")]
impl From<nalgebra::Vector2<f64>> for CpVect {
    fn from(v: nalgebra::Vector2<f64>) -> CpVect {
        CpVect { x: v.x, y: v.y }
    }
}
#[cfg(feature="nalgebra")]
impl From<CpVect> for nalgebra::Vector2<f64> {
    fn from(v: CpVect) -> nalgebra::Vector2<f64> {
        nalgebra::Vector2 { x: v.x, y: v.y }
    }
}

/// If chipmunk is compiled with the "nalgebra" feature, `CpVect` can be
/// converted to/from `nalgebra::Vector2<f32>`.
/// Be aware that converting from `f64` to `f32` may result in a loss of precision.
#[cfg(feature="nalgebra")]
impl From<nalgebra::Vector2<f32>> for CpVect {
    fn from(v: nalgebra::Vector2<f32>) -> CpVect {
        CpVect {
            x: v.x as f64,
            y: v.y as f64,
        }
    }
}
#[cfg(feature="nalgebra")]
impl From<CpVect> for nalgebra::Vector2<f32> {
    fn from(v: CpVect) -> nalgebra::Vector2<f32> {
        nalgebra::Vector2 {
            x: v.x as f32,
            y: v.y as f32,
        }
    }
}

/// If chipmunk is compiled with the "nalgebra" feature, `CpVect` can be
/// converted to/from `nalgebra::Point2<f64>`.
#[cfg(feature="nalgebra")]
impl From<nalgebra::Point2<f64>> for CpVect {
    fn from(v: nalgebra::Point2<f64>) -> CpVect {
        CpVect { x: v.x, y: v.y }
    }
}
#[cfg(feature="nalgebra")]
impl From<CpVect> for nalgebra::Point2<f64> {
    fn from(v: CpVect) -> nalgebra::Point2<f64> {
        nalgebra::Point2 { x: v.x, y: v.y }
    }
}

/// If chipmunk is compiled with the "nalgebra" feature, `CpVect` can be
/// converted to/from `nalgebra::Point2<f32>`.
/// Be aware that converting from `f64` to `f32` may result in a loss of precision.
#[cfg(feature="nalgebra")]
impl From<nalgebra::Point2<f32>> for CpVect {
    fn from(v: nalgebra::Point2<f32>) -> CpVect {
        CpVect {
            x: v.x as f64,
            y: v.y as f64,
        }
    }
}
#[cfg(feature="nalgebra")]
impl From<CpVect> for nalgebra::Point2<f32> {
    fn from(v: CpVect) -> nalgebra::Point2<f32> {
        nalgebra::Point2 {
            x: v.x as f32,
            y: v.y as f32,
        }
    }
}


/// `CpVect` can be converted to and from `(f64, f64)`.
impl From<(f64, f64)> for CpVect {
    fn from(tuple: (f64, f64)) -> CpVect {
        CpVect {
            x: tuple.0,
            y: tuple.1,
        }
    }
}
impl From<CpVect> for (f64, f64) {
    fn from(vect: CpVect) -> (f64, f64) {
        (vect.x, vect.y)
    }
}
/// `CpVect` can be converted to and from `(f32, f32)`.
/// Be aware that converting from `f64` to `f32` may result in a loss of precision.
impl From<(f32, f32)> for CpVect {
    fn from(tuple: (f32, f32)) -> CpVect {
        CpVect {
            x: tuple.0 as f64,
            y: tuple.1 as f64,
        }
    }
}
impl From<CpVect> for (f32, f32) {
    fn from(vect: CpVect) -> (f32, f32) {
        (vect.x as f32, vect.y as f32)
    }
}


/// `CpVect` can be converted to and from `[f64; 2]`.
impl From<[f64; 2]> for CpVect {
    fn from(array: [f64; 2]) -> CpVect {
        CpVect {
            x: array[0],
            y: array[1],
        }
    }
}
impl From<CpVect> for [f64; 2] {
    fn from(vect: CpVect) -> [f64; 2] {
        [vect.x, vect.y]
    }
}
/// `CpVect` can be converted to and from `[f32; 2]`.
/// Be aware that converting from `f64` to `f32` may result in a loss of precision.
impl From<[f32; 2]> for CpVect {
    fn from(array: [f32; 2]) -> CpVect {
        CpVect {
            x: array[0] as f64,
            y: array[1] as f64,
        }
    }
}
impl From<CpVect> for [f32; 2] {
    fn from(vect: CpVect) -> [f32; 2] {
        [vect.x as f32, vect.y as f32]
    }
}


impl Add for CpVect {
    type Output = CpVect;
    fn add(self, rhs: CpVect) -> CpVect {
        CpVect {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Div<f64> for CpVect {
    type Output = CpVect;
    fn div(self, rhs: f64) -> CpVect {
        CpVect {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Mul<f64> for CpVect {
    type Output = CpVect;
    fn mul(self, rhs: f64) -> CpVect {
        CpVect {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Neg for CpVect {
    type Output = CpVect;
    fn neg(self) -> CpVect {
        CpVect {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl PartialEq<CpVect> for CpVect {
    /// Returns true if `self.x == other.x && self.y == other.y`.
    fn eq(&self, other: &CpVect) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Sub for CpVect {
    type Output = CpVect;
    fn sub(self, rhs: CpVect) -> CpVect {
        CpVect {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::CpVect;

    #[test]
    fn cpvect_from_into_tuple_f64() {
        let v = CpVect::from((2.0f64, 3.0f64));
        assert_eq!(2.0f64, v.x);
        assert_eq!(3.0f64, v.y);
        assert_eq!((2.0f64, 3.0f64), <(f64, f64)>::from(v));

        let v2: CpVect = (4.0f64, 5.0f64).into();
        assert_eq!(4.0f64, v2.x);
        assert_eq!(5.0f64, v2.y);
        assert_eq!((4.0f64, 5.0f64), v2.into());
    }

    #[test]
    fn cpvect_from_into_tuple_f32() {
        let v = CpVect::from((2.0f32, 3.0f32));
        assert_eq!(2.0f64, v.x);
        assert_eq!(3.0f64, v.y);
        assert_eq!((2.0f32, 3.0f32), <(f32, f32)>::from(v));

        let v2: CpVect = (4.0f32, 5.0f32).into();
        assert_eq!(4.0f64, v2.x);
        assert_eq!(5.0f64, v2.y);
        assert_eq!((4.0f32, 5.0f32), v2.into());
    }

    #[test]
    fn cpvect_from_into_array_f64() {
        let v = CpVect::from([2.0f64, 3.0f64]);
        assert_eq!(2.0f64, v.x);
        assert_eq!(3.0f64, v.y);
        assert_eq!([2.0f64, 3.0f64], <[f64; 2]>::from(v));

        let v2: CpVect = [4.0f64, 5.0f64].into();
        assert_eq!(4.0f64, v2.x);
        assert_eq!(5.0f64, v2.y);
        let array: [f64; 2] = v2.into();
        assert_eq!([4.0f64, 5.0f64], array);
    }

    #[test]
    fn cpvect_from_into_array_f32() {
        let v = CpVect::from([2.0f32, 3.0f32]);
        assert_eq!(2.0f64, v.x);
        assert_eq!(3.0f64, v.y);
        assert_eq!([2.0f32, 3.0f32], <[f32; 2]>::from(v));

        let v2: CpVect = [4.0f32, 5.0f32].into();
        assert_eq!(4.0f64, v2.x);
        assert_eq!(5.0f64, v2.y);
        let array: [f32; 2] = v2.into();
        assert_eq!([4.0f32, 5.0f32], array);
    }

    #[cfg(feature="cgmath")]
    #[test]
    fn cpvect_from_into_cgmath_vector2_f64() {
        use cgmath::Vector2;

        let cpv = CpVect::from(Vector2::new(2.0f64, 3.0f64));
        assert_eq!(2.0f64, cpv.x);
        assert_eq!(3.0f64, cpv.y);

        let cgmv = Vector2::<f64>::from(cpv);
        assert_eq!(2.0f64, cgmv.x);
        assert_eq!(3.0f64, cgmv.y);

        let cpv: CpVect = Vector2::new(4.0f64, 5.0f64).into();
        assert_eq!(4.0f64, cpv.x);
        assert_eq!(5.0f64, cpv.y);

        let cgmv2: Vector2<f64> = cpv.into();
        assert_eq!(4.0f64, cgmv2.x);
        assert_eq!(5.0f64, cgmv2.y);
    }

    #[cfg(feature="cgmath")]
    #[test]
    fn cpvect_from_into_cgmath_vector2_f32() {
        use cgmath::Vector2;

        let cpv = CpVect::from(Vector2::new(2.0f32, 3.0f32));
        assert_eq!(2.0f64, cpv.x);
        assert_eq!(3.0f64, cpv.y);

        let cgmv = Vector2::<f32>::from(cpv);
        assert_eq!(2.0f32, cgmv.x);
        assert_eq!(3.0f32, cgmv.y);

        let cpv: CpVect = Vector2::new(4.0f32, 5.0f32).into();
        assert_eq!(4.0f64, cpv.x);
        assert_eq!(5.0f64, cpv.y);

        let cgmv2: Vector2<f32> = cpv.into();
        assert_eq!(4.0f32, cgmv2.x);
        assert_eq!(5.0f32, cgmv2.y);
    }

    #[cfg(feature="nalgebra")]
    #[test]
    fn cpvect_from_into_nalgebra_vector2_f64() {
        use nalgebra::Vector2;

        let cpv = CpVect::from(Vector2::new(2.0f64, 3.0f64));
        assert_eq!(2.0f64, cpv.x);
        assert_eq!(3.0f64, cpv.y);

        let nalv = Vector2::<f64>::from(cpv);
        assert_eq!(2.0f64, nalv.x);
        assert_eq!(3.0f64, nalv.y);

        let cpv: CpVect = Vector2::new(4.0f64, 5.0f64).into();
        assert_eq!(4.0f64, cpv.x);
        assert_eq!(5.0f64, cpv.y);

        let nalv2: Vector2<f64> = cpv.into();
        assert_eq!(4.0f64, nalv2.x);
        assert_eq!(5.0f64, nalv2.y);
    }

    #[cfg(feature="nalgebra")]
    #[test]
    fn cpvect_from_into_nalgebra_vector2_f32() {
        use nalgebra::Vector2;

        let cpv = CpVect::from(Vector2::new(2.0f32, 3.0f32));
        assert_eq!(2.0f64, cpv.x);
        assert_eq!(3.0f64, cpv.y);

        let nalv = Vector2::<f32>::from(cpv);
        assert_eq!(2.0f32, nalv.x);
        assert_eq!(3.0f32, nalv.y);

        let cpv: CpVect = Vector2::new(4.0f32, 5.0f32).into();
        assert_eq!(4.0f64, cpv.x);
        assert_eq!(5.0f64, cpv.y);

        let nalv2: Vector2<f32> = cpv.into();
        assert_eq!(4.0f32, nalv2.x);
        assert_eq!(5.0f32, nalv2.y);
    }

    #[cfg(feature="nalgebra")]
    #[test]
    fn cpvect_from_into_nalgebra_point2_f64() {
        use nalgebra::Point2;

        let cpv = CpVect::from(Point2::new(2.0f64, 3.0f64));
        assert_eq!(2.0f64, cpv.x);
        assert_eq!(3.0f64, cpv.y);

        let nalv = Point2::<f64>::from(cpv);
        assert_eq!(2.0f64, nalv.x);
        assert_eq!(3.0f64, nalv.y);

        let cpv: CpVect = Point2::new(4.0f64, 5.0f64).into();
        assert_eq!(4.0f64, cpv.x);
        assert_eq!(5.0f64, cpv.y);

        let nalv2: Point2<f64> = cpv.into();
        assert_eq!(4.0f64, nalv2.x);
        assert_eq!(5.0f64, nalv2.y);
    }

    #[cfg(feature="nalgebra")]
    #[test]
    fn cpvect_from_into_nalgebra_point2() {
        use nalgebra::Point2;

        let cpv = CpVect::from(Point2::new(2.0f32, 3.0f32));
        assert_eq!(2.0f64, cpv.x);
        assert_eq!(3.0f64, cpv.y);

        let nalv = Point2::<f32>::from(cpv);
        assert_eq!(2.0f32, nalv.x);
        assert_eq!(3.0f32, nalv.y);

        let cpv: CpVect = Point2::new(4.0f32, 5.0f32).into();
        assert_eq!(4.0f64, cpv.x);
        assert_eq!(5.0f64, cpv.y);

        let nalv2: Point2<f32> = cpv.into();
        assert_eq!(4.0f32, nalv2.x);
        assert_eq!(5.0f32, nalv2.y);
    }
}
