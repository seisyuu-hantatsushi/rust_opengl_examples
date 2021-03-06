use linear_transform::{vector::*,matrix::*};

use std::{f64};
use std::{f64::consts::PI};

/* 平行移動行列 */
pub fn translate(position:Vector3) -> Matrix4x4 {
    Matrix4x4 {
	v:
	[
	    [ 1.0 , 0.0, 0.0, position.0 ],
	    [ 0.0 , 1.0, 0.0, position.1 ],
	    [ 0.0 , 0.0, 1.0, position.2 ],
	    [ 0.0 , 0.0, 0.0,        1.0 ],
	]
    }
}


/* 拡大縮小行列 */
pub fn scale(scale:Vector3) -> Matrix4x4 {
    Matrix4x4 {
	v:
	[
	    [ scale.0,     0.0,     0.0,  0.0 ],
	    [     0.0, scale.1,     0.0,  0.0 ],
	    [     0.0,     0.0, scale.2,  0.0 ],
	    [     0.0,     0.0,     0.0,  1.0 ]
	]
    }
}

/* 任意軸での回転行列 */
/* ロドリゲスの回転公式から */
pub fn rotate(axis:Vector3, angle_in_degree:f64) -> Matrix4x4 {
    let theta = 2.0*PI*angle_in_degree/360.0;
    let n = axis.normalize();
    Matrix4x4 {
	v:
	[
	    [ n.0*n.0*(1.0-theta.cos())+1.0*theta.cos(), n.0*n.1*(1.0-theta.cos())-n.2*theta.sin(), n.0*n.2*(1.0-theta.cos())+n.1*theta.sin(), 0.0 ],
	    [ n.0*n.1*(1.0-theta.cos())+n.2*theta.sin(), n.1*n.1*(1.0-theta.cos())+1.0*theta.cos(), n.1*n.2*(1.0-theta.cos())-n.0*theta.sin(), 0.0 ],
	    [ n.0*n.2*(1.0-theta.cos())-n.1*theta.sin(), n.1*n.2*(1.0-theta.cos())+n.0*theta.sin(), n.2*n.2*(1.0-theta.cos())+1.0*theta.cos(), 0.0 ],
	    [                                       0.0,                                       0.0,                                       0.0, 1.0 ],
	]
    }
}

/* 視野変換行列 */
pub fn look_at(eye:Vector3, center:Vector3, up:Vector3) -> Matrix4x4 {
    /*
    z' = (z'_x,z'_y,z'_z) = (1/|eye-center|)(eye_x-center_x,eye_y-center_y,eye_z-center_z)
    x' = (x'_x,x'_y,x'_z) = (up×z'/|up×z'|) = (1/|up×z'|)(up_y*z'_z-up_z*z'_y,up_z*z'_x-up_x*z'_z,up_x*z'_y-up_y*z'_x)
    y' = (y'_x,y'_y,y'_z) = z'×x' = (z'_y*x'_z-z'_z*x'_y, z'_z*x'_x-z'_x*x'_z, z'_x*x'_y-z'_y*x'_x)
    | x'_x, x'_y ,x'_z, -eye*x |
    | y'_x, y'_y ,y'_z, -eye*y |
    | z'_x, z'_y ,z'_z, -eye*z |
    |  0.0,  0.0,  0.0,    1.0 |
     */
    let z = (eye-center).normalize();
    let x = Vector3::cross_product(up,z)/Vector3::cross_product(up,z).length();
    let y = Vector3::cross_product(z,x);
    Matrix4x4 {
	v :
	[
	    [x.0, x.1, x.2, -eye*x ],
	    [y.0, y.1, y.2, -eye*y ],
	    [z.0, z.1, z.2, -eye*z ],
	    [0.0, 0.0, 0.0,    1.0 ]
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

pub fn frustum(left:f64, right:f64, bottom:f64, top:f64, near:f64, far:f64) -> Matrix4x4 {
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

/* fovyはdegree */
pub fn perspective(fovy:f64, aspect:f64, near:f64, far:f64) -> Matrix4x4 {
    /*
    透視投影変換行列
    fovy_rad = fovy*PI/180.0;
    f = 1/tan(fovy_rad/2)
    |         f/aspect,                       0,                          0,                         0 |
    |                   0,                    f,                          0,                         0 |
    |                   0,                    0,     -(far+near)/(far-near),  -2*far*near/(far - near) |
    |                   0,                    0,                         -1,                         0 |
     */
    let theta = fovy*0.5*PI/180.0;
    let f = theta.tan();
    Matrix4x4 {
	v:
	[
	    [ 1.0/(aspect*f),         0.0,                         0.0,                       0.0 ],
	    [            0.0,       1.0/f,                         0.0,                       0.0 ],
	    [            0.0,         0.0,      -(far+near)/(far-near),  -2.0*far*near/(far-near) ],
	    [            0.0,         0.0,                        -1.0,                       0.0 ]
	]
    }
}

pub fn normal_matrix(view:Matrix4x4) -> Matrix3x3 {
    Matrix3x3 {
	v:
	[
	    [ view[0][0], view[1][0], view[2][0] ],
	    [ view[0][1], view[1][1], view[2][1] ],
	    [ view[0][2], view[1][2], view[2][2] ]
	]
    }
}

pub fn vector3_to_vector4_pos(v:Vector3) -> Vector4 {
    Vector4(v.0, v.1, v.2, 1.0)
}
