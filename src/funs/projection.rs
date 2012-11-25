use funs::trig::*;
use angle::Angle;
use mat::Mat4;
use num::cast::cast;
use num::consts::pi;
use num::ext::FloatExt;

//
//  Create a perspective projection matrix
//
//  fov is in degrees
//  http://www.opengl.org/wiki/GluPerspective_code
//
#[inline(always)]
pure fn perspective<T:Copy FloatExt>(fovy: Radians<T>, aspectRatio: T, near: T, far: T) -> Mat4<T> {
    let ymax = near * tan(&fovy);
    let xmax = ymax * aspectRatio;
    return frustum(-xmax, xmax, -ymax, ymax, near, far);
}

//
//  Define a view frustrum
//  http://www.felixgers.de/teaching/jogl/perspectiveProjection.html
//  http://www.opengl.org/wiki/GluPerspective_code
//
//  TODO: double check algorithm
//
#[inline(always)]
pure fn frustum<T:Copy FloatExt>(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Mat4<T> {
    let _0: T = cast(0);
    let _2: T = cast(2);
    let neg_1 = cast(-1);
    
    let c0r0 = (_2 * near) / (right - left);
    let c0r1 = _0;
    let c0r2 = _0;
    let c0r3 = _0;
    let c1r0 = _0;
    let c1r1 = (_2 * near) / (top - bottom);
    let c1r2 = _0;
    let c1r3 = _0;
    let c2r0 = (right + left) / (right - left);
    let c2r1 = (top + bottom) / (top - bottom);
    let c2r2 = -(far + near) / (far - near);
    let c2r3 = neg_1;
    let c3r0 = _0;
    let c3r1 = _0;
    let c3r2 = -(_2 * far * near) / (far - near);
    let c3r3 = _0;
    
    return Mat4::new(c0r0, c0r1, c0r2, c0r3,
                     c1r0, c1r1, c1r2, c1r3,
                     c2r0, c2r1, c2r2, c2r3,
                     c3r0, c3r1, c3r2, c3r3);
}