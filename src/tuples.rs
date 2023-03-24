use std::ops;

const EPSILON: f64 = 0.00001;

#[derive(Debug, PartialEq)]
struct FourTuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl FourTuple {
    fn point<T: Into<f64>>(x: T, y: T, z: T) -> FourTuple {
        FourTuple {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 1f64,
        }
    }

    fn vector<T: Into<f64>>(x: T, y: T, z: T) -> FourTuple {
        FourTuple {
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
            w: self.w,
        }
    }
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
    }

    #[test]
    fn test_negate_vector() {
        let expected = FourTuple::vector(-1, -1, -1);
        let actual = -FourTuple::vector(1, 1, 1);
        assert_eq!(expected, actual);
    }
}
