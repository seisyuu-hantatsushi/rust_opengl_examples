mod vector;


#[cfg(test)]
mod tests {
    use super::*;
    use vector::Vector2;
    
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
}
