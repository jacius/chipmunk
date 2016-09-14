use chip;
use super::CpVect;

/// Calculate the area for a circle (possibly hollow).
/// A solid circle has an inner_radius of 0.
pub fn area_for_circle(inner_radius: f64, outer_radius: f64) -> f64 {
    unsafe { chip::cpAreaForCircle(inner_radius, outer_radius) }
}

/// Calculate the moment of inertia for a circle (possibly hollow).
/// A solid circle has an inner_radius of 0.
pub fn moment_for_circle(mass: f64,
                         inner_radius: f64,
                         outer_radius: f64,
                         offset: (f64, f64))
                         -> f64 {
    unsafe { chip::cpMomentForCircle(mass, inner_radius, outer_radius, offset.into()) }
}


/// Calculate the area for a fattened (capsule shaped) line segment.
pub fn area_for_segment(a: (f64, f64), b: (f64, f64), radius: f64) -> f64 {
    unsafe { chip::cpAreaForSegment(a.into(), b.into(), radius) }
}

/// Calculate the moment of inertia for a fattened (capsule shaped) line segment.
pub fn moment_for_segment(mass: f64, a: (f64, f64), b: (f64, f64), radius: f64) -> f64 {
    unsafe { chip::cpMomentForSegment(mass, a.into(), b.into(), radius) }
}


/// Calculate the signed area of a polygon.
/// A clockwise winding gives positive area.
/// This is probably backwards from what you expect,
/// but matches Chipmunk's the winding for poly shapes.
pub fn area_for_poly<'a, V: 'a>(verts: &'a [V], radius: f64) -> f64
    where CpVect: From<&'a V> {
    let verts = verts.iter().map(|v| CpVect::from(v).into()).collect::<Vec<chip::cpVect>>();
    unsafe {
        chip::cpAreaForPoly(verts.len() as i32,
                            (&verts).as_ptr() as *const chip::cpVect,
                            radius)
    }
}

/// Calculate the moment of inertia for a solid polygon shape
/// assuming its center of gravity is at its centroid.
/// The offset is added to each vertex.
pub fn moment_for_poly<'a, V: 'a>(mass: f64, verts: &'a [V], offset: (f64, f64), radius: f64) -> f64
    where CpVect: From<&'a V> {
    let verts = verts.iter().map(|v| CpVect::from(v).into()).collect::<Vec<chip::cpVect>>();
    unsafe {
        chip::cpMomentForPoly(mass,
                              verts.len() as i32,
                              (&verts).as_ptr() as *const chip::cpVect,
                              offset.into(),
                              radius)
    }
}

/// Calculate the natural centroid of a polygon.
pub fn centroid_for_poly<'a, V: 'a>(verts: &'a [V]) -> (f64, f64)
    where CpVect: From<&'a V> {
    let verts = verts.iter().map(|v| CpVect::from(v).into()).collect::<Vec<chip::cpVect>>();
    unsafe {
        chip::cpCentroidForPoly(verts.len() as i32, (&verts).as_ptr() as *const chip::cpVect).into()
    }
}

/// Calculate the moment of inertia for a solid box.
pub fn moment_for_box(mass: f64, width: f64, height: f64) -> f64 {
    unsafe { chip::cpMomentForBox(mass, width, height) }
}
