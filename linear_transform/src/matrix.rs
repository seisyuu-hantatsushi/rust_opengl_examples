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

#[allow(dead_code)]
pub fn matrix4x4_identity() -> Matrix4x4 {
    Matrix4x4 {
	v : [ [1.0, 0.0, 0.0, 0.0],
	      [0.0, 1.0, 0.0, 0.0],
	      [0.0, 0.0, 1.0, 0.0],
	      [0.0, 0.0, 0.0, 1.0] ]
    }
}

impl ops::Add for Matrix4x4 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
	let mut m = matrix4x4_identity();
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
	let mut m = matrix4x4_identity();
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
	    v : [
		[matrix4x4_entry(self, other, 0, 0), matrix4x4_entry(self, other, 0, 1), matrix4x4_entry(self, other, 0, 2), matrix4x4_entry(self, other, 0, 3)],
		[matrix4x4_entry(self, other, 1, 0), matrix4x4_entry(self, other, 1, 1), matrix4x4_entry(self, other, 1, 2), matrix4x4_entry(self, other, 1, 3)],
		[matrix4x4_entry(self, other, 2, 0), matrix4x4_entry(self, other, 2, 1), matrix4x4_entry(self, other, 2, 2), matrix4x4_entry(self, other, 2, 3)],
		[matrix4x4_entry(self, other, 3, 0), matrix4x4_entry(self, other, 3, 1), matrix4x4_entry(self, other, 3, 2), matrix4x4_entry(self, other, 3, 3)]
	    ]
	}
    }
}
