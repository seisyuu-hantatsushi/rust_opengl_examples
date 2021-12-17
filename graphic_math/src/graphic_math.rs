use linear_transform::{vector::*,matrix::{Matrix4x4}};

use std::{f64};

/* 視野変換行列 */
pub fn look_at(eye:Vector3, center:Vector3, up:Vector3) -> Matrix4x4 {
    /*
    z' = (z'_x,z'_y,z'_z) = (1/|eye-center|)(eye_x-center_x,eye_y-center_y,eye_z-center_z)
    x' = (x'_x,x'_y,x'_z) = (up×z'/|up×z'|) = (1/|up×z'|)(up_y*z'_z-up_z*z'_y,up_z*z'_x-up_x*z'_z,up_x*z'_y-up_y*z'_x)
    y' = (y'_x,y'_y,y'_z) = z'×x' = (z'_y*x'_z-z'_z*x'_y, z'_z*x'_x-z'_x*x'_z, z'_x*x'_y-z'_y*x'_x)
    | x'_x, x'_y ,x'_z, -eye_x |
    | y'_x, y'_y ,y'_z, -eye_y |
    | z'_x, z'_y ,z'_z, -eye_z |
     */
    let z = (1.0/(eye-center).length())*(eye-center);
    let x = cross_product(up,z)/cross_product(up,z).length();
    Matrix4x4 {
	v :
	[
	    [x.0, x.1, x.2, -eye.0 ],
	    [0.0, 0.0, 0.0, -eye.1 ],
	    [z.0, z.1, z.2, -eye.2 ],
	    [0.0, 0.0, 0.0,    0.0 ]
	]
    }
}

pub fn orthogonal(left:f64, right:f64, bottom:f64, top:f64, near:f64, far:f64) -> Matrix4x4 {
/*
    平行投影変換行列
    | 2/(right-left),                    0,                 0,             -(right+left)/(right-left) |
    |              0,       2/(top-bottom),                 0,             -(top+bottom)/(top-bottom) |
    |              0,                    0,     -2/(far-near),                 -(far+near)/(far-near) |
    |              0,                    0,                 0,                                     1  |
*/
    Matrix4x4 {
	v :
	[
	    [   2.0/(right-left),                        0.0,                    0.0,   -(right+left)/(right-left) ],
	    [                0.0,           2.0/(top-bottom),                    0.0,   -(top+bottom)/(top-bottom) ],
	    [                0.0,                        0.0,        -2.0/(far-near),       -(far+near)/(far-near) ],
	    [                0.0,                        0.0,                    0.0,                          1.0 ]
	]
    }
}

pub fn frustum_project(left:f64, right:f64, bottom:f64, top:f64, near:f64, far:f64) -> Matrix4x4 {
    /*
    透視投影変換行列
    | 2*near/(right-left),                    0,    (right+left)/(right-left),                         0 |
    |                   0,  2*near/(top-bottom),    (top+bottom)/(top-bottom),                         0 |
    |                   0,                    0,     -(far+near)/(far-near),    -2*far*near/(far - near) |
    |                   0,                    0,                           -1,                         0 |
     */
    Matrix4x4 {
	v:
	[
	    [  2.0*near/(right-left),                       0.0,   (right+left)/(right-left),                       0.0 ],
	    [                    0.0,     2.0*near/(top-bottom),   (top+bottom)/(top-bottom),                       0.0 ],
	    [                    0.0,                       0.0,      -(far+near)/(far-near),  -2.0*far*near/(far-near) ],
	    [                    0.0,                       0.0,                        -1.0,                       0.0 ]
	]
    }
}