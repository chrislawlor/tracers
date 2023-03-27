use std::ops;

const EPSILON: f64 = 0.00001;

#[derive(Debug, PartialEq)]
pub struct FourTuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl FourTuple {
    pub fn new<T: Into<f64>>(x: T, y: T, z: T, w: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }

    pub fn point<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 1f64,
        }
    }

    pub fn vector<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 0f64,
        }
    }

    fn is_point(obj: &FourTuple) -> bool {
        obj.w == 1.0
    }

    fn is_vector(obj: &FourTuple) -> bool {
        obj.w == 0.0
    }
}

impl ops::Add<FourTuple> for FourTuple {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}

impl ops::Sub<FourTuple> for FourTuple {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w: self.w - _rhs.w,
        }
    }
}

impl ops::Neg for FourTuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Mul<f64> for FourTuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        FourTuple::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl ops::Mul<i32> for FourTuple {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self {
        FourTuple::new(
            self.x * scalar as f64,
            self.y * scalar as f64,
            self.z * scalar as f64,
            self.w * scalar as f64,
        )
    }
}

pub fn magnitude(tuple: &FourTuple) -> f64 {
    let exp = 2f64;
    f64::sqrt(tuple.x.powf(exp) + tuple.y.powf(exp) + tuple.z.powf(exp) + tuple.w.powf(exp))
}

pub fn normalize(vector: &FourTuple) -> FourTuple {
    let m = magnitude(vector);
    FourTuple::new(vector.x / m, vector.y / m, vector.z / m, vector.w / m)
}

pub fn dot(a: &FourTuple, b: &FourTuple) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

pub fn cross(a: &FourTuple, b: &FourTuple) -> FourTuple {
    FourTuple::vector(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_equality() {
        assert_eq!(FourTuple::point(1, 1, 1), FourTuple::point(1, 1, 1))
    }

    #[test]
    fn test_is_point() {
        let point = FourTuple::point(0, 0, 0);
        assert!(FourTuple::is_point(&point));
    }

    #[test]
    fn test_add_point_and_vector() {
        let expected = FourTuple::point(3, 3, 3);
        let actual = FourTuple::point(1, 1, 1) + FourTuple::vector(2, 2, 2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_subtract_vector_from_point() {
        let expected = FourTuple::point(1, 1, 1);
        let actual = FourTuple::point(3, 3, 3) - FourTuple::vector(2, 2, 2);
        assert_eq!(expected, actual);
        assert!(FourTuple::is_point(&actual));
    }

    #[test]
    fn test_subtract_point_from_point() {
        let expected = FourTuple::vector(-2, -4, -6);
        let actual = FourTuple::point(3, 2, 1) - FourTuple::point(5, 6, 7);
        assert_eq!(expected, actual);
        assert!(FourTuple::is_vector(&actual));
    }

    #[test]
    fn test_subtract_two_vectors() {
        let expected = FourTuple::vector(-2, -4, -6);
        let actual = FourTuple::vector(3, 2, 1) - FourTuple::vector(5, 6, 7);
        assert_eq!(expected, actual);
        assert!(FourTuple::is_vector(&actual));
    }

    #[test]
    fn test_negate_tuple() {
        let expected = FourTuple::new(-1, -2, -3, -4);
        let actual = -FourTuple::new(1, 2, 3, 4);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multiply_tuple_by_scalar() {
        let expected = FourTuple::new(3.5, -7.0, 10.5, -14.0);
        let actual = FourTuple::new(1, -2, 3, -4) * 3.5;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multiply_tuple_by_scalar_int() {
        let expected = FourTuple::new(3, -6, 9, -12);
        let actual = FourTuple::new(1, -2, 3, -4) * 3;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_magnitude() {
        let cases = vec![
            ((1, 0, 0), 1f64),
            ((0, 1, 0), 1f64),
            ((0, 0, 1), 1f64),
            ((1, 2, 3), 14f64.sqrt()),
            ((-1, -2, -3), 14f64.sqrt()),
        ];
        for case in cases.iter() {
            let (args, expected) = case.clone();
            let (x, y, z) = args;
            let tuple = FourTuple::vector(x, y, z);
            let result = magnitude(&tuple);
            assert_eq!(result, expected as f64);
        }
    }

    #[test]
    fn test_normalize() {
        let sqrt_14 = 14f64.sqrt();
        let cases = vec![
            (FourTuple::vector(4, 0, 0), FourTuple::vector(1, 0, 0)),
            (
                FourTuple::vector(1, 2, 3),
                FourTuple::vector(1f64 / sqrt_14, 2f64 / sqrt_14, 3f64 / sqrt_14),
            ),
        ];
        for case in cases.iter() {
            let (vector, expected) = case.clone();
            let result = normalize(vector);
            assert_eq!(&result, expected);
        }
    }

    #[test]
    fn test_dot() {
        let a = FourTuple::vector(1, 2, 3);
        let b = FourTuple::vector(2, 3, 4);
        assert_eq!(dot(&a, &b), 20f64);
    }

    #[test]
    fn test_cross() {
        let a = FourTuple::vector(1, 2, 3);
        let b = FourTuple::vector(2, 3, 4);
        assert_eq!(cross(&a, &b), FourTuple::vector(-1, 2, -1));
        assert_eq!(cross(&b, &a), FourTuple::vector(1, -2, 1));
    }
}
