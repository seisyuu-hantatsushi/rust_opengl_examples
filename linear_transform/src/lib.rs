mod vector;
pub mod matrix;

#[cfg(test)]
mod tests {
    use super::*;
    use vector::Vector2;
    use matrix::Matrix4x4;
    use matrix::*;

    #[test]
    fn vector2_add(){
	let a = Vector2(1.0, 2.0);
	let b = Vector2(2.0, 1.0);
	assert_eq!(a+b, Vector2(3.0,3.0));
    }

    #[test]
    fn vector2_sub(){
	let a = Vector2(1.0, 2.0);
	let b = Vector2(2.0, 1.0);
	assert_eq!(a-b, Vector2(-1.0,1.0));
    }

    #[test]
    fn vector2_mul(){
	let a = Vector2(1.0, 2.0);
	let b = Vector2(2.0, 1.0);
	assert_eq!(a*b, b*a);
    }

    #[test]
    fn vector2_lenght(){
	let id = Vector2(3.0, 4.0);
	assert_eq!(id.square(), 25.0);
	assert_eq!(id.length(), 5.0);
    }

    #[test]
    fn matrix_test(){
	let m1 = Matrix4x4{
	    v: [ [1.0, 1.0, 1.0, 1.0],
		  [2.0, 2.0, 2.0, 2.0],
		  [3.0, 3.0, 3.0, 3.0],
		  [4.0, 4.0, 4.0, 4.0],
	    ]
	};
	let m2 = Matrix4x4 {
	    v: [ [4.0, 4.0, 4.0, 4.0],
		  [3.0, 3.0, 3.0, 3.0],
		  [2.0, 2.0, 2.0, 2.0],
		  [1.0, 1.0, 1.0, 1.0],
	    ]
	};
	let m3 = Matrix4x4 {
	    v: [ [5.0, 5.0, 5.0, 5.0],
		  [5.0, 5.0, 5.0, 5.0],
		  [5.0, 5.0, 5.0, 5.0],
		  [5.0, 5.0, 5.0, 5.0] ]
	};
	assert_eq!(m1+m2, m3);
	assert_eq!(m3-m1, m2);
	assert_eq!(m1*matrix4x4_identity(), matrix4x4_identity()*m1);
    }
}
