
use std::{f64,ops,fmt};

#[derive(Debug,Copy,Clone, PartialEq)]
pub struct Matrix4x4 {
    pub v : [[f64; 4]; 4]
}

#[allow(dead_code)]
impl fmt::Display for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let mut str = "Matrix4x4:\n".to_string();
	for i in 0..4 {
	    let v_str = format!("[{},{},{},{}]\n",
				self.v[i][0],self.v[i][1],self.v[i][2],self.v[i][3]);
	    str.push_str(&v_str);
	}
	write!(f,"{}", str)
    }
}

impl ops::Add for Matrix4x4 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
	let mut m = Self::identity();
	for i in 0..4 {
	    for j in 0..4 {
		m.v[i][j] = self.v[i][j]+other.v[i][j]
	    }
	}
	m
    }
}

impl ops::Sub for Matrix4x4 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
	let mut m = Self::identity();
	for i in 0..4 {
	    for j in 0..4 {
		m.v[i][j] = self.v[i][j]-other.v[i][j]
	    }
	}
	m
    }
}

fn matrix4x4_entry(l:Matrix4x4, r:Matrix4x4, i:usize, j:usize) -> f64 {
    let v:[f64; 4] = [ l.v[i][0]*r.v[0][j], l.v[i][1]*r.v[1][j], l.v[i][2]*r.v[2][j], l.v[i][3]*r.v[3][j] ];
    v.iter().fold(0.0, |acc, e| acc+e)
}

impl ops::Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
	Matrix4x4 {
	    v :
	    [
		[matrix4x4_entry(self, other, 0, 0), matrix4x4_entry(self, other, 0, 1), matrix4x4_entry(self, other, 0, 2), matrix4x4_entry(self, other, 0, 3)],
		[matrix4x4_entry(self, other, 1, 0), matrix4x4_entry(self, other, 1, 1), matrix4x4_entry(self, other, 1, 2), matrix4x4_entry(self, other, 1, 3)],
		[matrix4x4_entry(self, other, 2, 0), matrix4x4_entry(self, other, 2, 1), matrix4x4_entry(self, other, 2, 2), matrix4x4_entry(self, other, 2, 3)],
		[matrix4x4_entry(self, other, 3, 0), matrix4x4_entry(self, other, 3, 1), matrix4x4_entry(self, other, 3, 2), matrix4x4_entry(self, other, 3, 3)]
	    ]
	}
    }
}

/* OpenGL側に渡すときは転置する必要がある !! */
impl Matrix4x4 {

    pub fn zero() -> Matrix4x4 {
	Matrix4x4 {
	    v :
	    [
		[0.0, 0.0, 0.0, 0.0],
		[0.0, 0.0, 0.0, 0.0],
		[0.0, 0.0, 0.0, 0.0],
		[0.0, 0.0, 0.0, 0.0]
	    ]
	}
    }

    /* 単位行列 */
    pub fn identity() -> Matrix4x4 {
	Matrix4x4 {
	    v :
	    [
		[1.0, 0.0, 0.0, 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0]
	    ]
	}
    }

    /* 転置行列 */
    pub fn transport(self) -> Matrix4x4 {
	Matrix4x4 {
	    v :
	    [
		[self.v[0][0], self.v[1][0], self.v[2][0], self.v[3][0] ],
		[self.v[0][1], self.v[1][1], self.v[2][1], self.v[3][1] ],
		[self.v[0][2], self.v[1][2], self.v[2][2], self.v[3][2] ],
		[self.v[0][3], self.v[1][3], self.v[2][3], self.v[3][3] ]
	    ]
	}
    }

    /*
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
	Matrix4x4 {
	    v :
	    [
		[0.0, 0.0, 0.0, 0.0 ],
		[0.0, 0.0, 0.0, 0.0 ],
		[0.0, 0.0, 0.0, 0.0 ],
		[0.0, 0.0, 0.0, 0.0 ]
	    ]
	}
    }
*/
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
}
