use chip::cpVect;
use std::ops::{Add, Div, Mul, Neg, Sub};


/// Two-dimensional vector.
///
/// `CpVect` is a 2D vector type used by Chipmunk. It is suitable for simple 2D
/// vector math, but for more advanced situations you should use a linear
/// algebra crate such as [cgmath](https://crates.io/crates/cgmath)
/// or [nalgebra](https://crates.io/crates/nalgebra).
///
/// # Conversion
///
/// You can convert `CpVect` to and from `(f64, f64)` or `[f64; 2]`
/// using the standard `From` and `Into` traits:
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


// Converting CpVect to and from a tuple
impl From<CpVect> for (f64, f64) {
    fn from(vect: CpVect) -> (f64, f64) {
        (vect.x, vect.y)
    }
}
impl From<(f64, f64)> for CpVect {
    fn from(tuple: (f64, f64)) -> CpVect {
        CpVect {
            x: tuple.0,
            y: tuple.1,
        }
    }
}


// Converting CpVect to and from an array
impl From<CpVect> for [f64; 2] {
    fn from(vect: CpVect) -> [f64; 2] {
        [vect.x, vect.y]
    }
}
impl From<[f64; 2]> for CpVect {
    fn from(array: [f64; 2]) -> CpVect {
        CpVect {
            x: array[0],
            y: array[1],
        }
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
    fn test_cpvect_from_into_tuple() {
        let v = CpVect::from((1.2, 3.4));
        assert_eq!(1.2, v.x);
        assert_eq!(3.4, v.y);
        assert_eq!((1.2, 3.4), From::from(v));

        let v2: CpVect = (5.6, 7.8).into();
        assert_eq!(5.6, v2.x);
        assert_eq!(7.8, v2.y);
        assert_eq!((5.6, 7.8), v2.into());
    }

    #[test]
    fn test_cpvect_from_into_array() {
        let v = CpVect::from([1.2, 3.4]);
        assert_eq!(1.2, v.x);
        assert_eq!(3.4, v.y);
        assert_eq!([1.2, 3.4], <[f64; 2]>::from(v));

        let v2: CpVect = [5.6, 7.8].into();
        assert_eq!(5.6, v2.x);
        assert_eq!(7.8, v2.y);
        let array: [f64; 2] = v2.into();
        assert_eq!([5.6, 7.8], array);
    }
}
