use crate::math::Vec2D;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Box2D {
    pub lower: Vec2D,
    pub upper: Vec2D,
}

impl From<(Vec2D, Vec2D)> for Box2D {
    fn from((lower, upper): (Vec2D, Vec2D)) -> Self {
        Self { lower, upper }
    }
}

impl FromIterator<Vec2D> for Box2D {
    fn from_iter<I: IntoIterator<Item = Vec2D>>(iter: I) -> Self {
        let mut box2d = Self::new((i64::MAX, i64::MAX), (i64::MIN, i64::MIN));
        for point in iter {
            box2d.extend(point);
        }
        box2d
    }
}

impl Box2D {
    pub fn new(lower: impl Into<Vec2D>, upper: impl Into<Vec2D>) -> Self {
        Self {
            lower: lower.into(),
            upper: upper.into(),
        }
    }

    pub fn lower(&self) -> Vec2D {
        self.lower
    }

    pub fn upper(&self) -> Vec2D {
        self.upper
    }

    pub fn contains(&self, point: impl Into<Vec2D>) -> bool {
        let point = point.into();
        point.x >= self.lower.x
            && point.y >= self.lower.y
            && point.x <= self.upper.x
            && point.y <= self.upper.y
    }

    pub fn extend(&mut self, point: impl Into<Vec2D>) {
        let point = point.into();
        self.lower = Vec2D::new(self.lower.x.min(point.x), self.lower.y.min(point.y));
        self.upper = Vec2D::new(self.upper.x.max(point.x), self.upper.y.max(point.y));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let box2d = Box2D::new(Vec2D::new(0, 0), Vec2D::new(2, 2));
        assert!(box2d.contains(Vec2D::new(0, 0)));
        assert!(box2d.contains(Vec2D::new(1, 1)));
        assert!(box2d.contains(Vec2D::new(2, 2)));
        assert!(!box2d.contains(Vec2D::new(-1, 0)));
        assert!(!box2d.contains(Vec2D::new(0, -1)));
        assert!(!box2d.contains(Vec2D::new(3, 0)));
        assert!(!box2d.contains(Vec2D::new(0, 3)));

        let box2d = Box2D::new(Vec2D::new(-1, -1), Vec2D::new(2, 2));
        assert!(box2d.contains(Vec2D::new(-1, -1)));
        assert!(box2d.contains(Vec2D::new(0, 0)));
        assert!(box2d.contains(Vec2D::new(1, 1)));
        assert!(box2d.contains(Vec2D::new(2, 2)));
        assert!(!box2d.contains(Vec2D::new(-2, 0)));
        assert!(!box2d.contains(Vec2D::new(0, -2)));
        assert!(!box2d.contains(Vec2D::new(3, 0)));
        assert!(!box2d.contains(Vec2D::new(0, 3)));
    }

    #[test]
    fn test_extend() {
        let mut box2d = Box2D::new(Vec2D::new(0, 0), Vec2D::new(2, 2));
        box2d.extend(Vec2D::new(3, 3));
        assert_eq!(box2d, Box2D::new(Vec2D::new(0, 0), Vec2D::new(3, 3)));
        box2d.extend(Vec2D::new(-1, -1));
        assert_eq!(box2d, Box2D::new(Vec2D::new(-1, -1), Vec2D::new(3, 3)));
    }

    #[test]
    fn test_from_points() {
        let box2d: Box2D = [
            Vec2D::new(0, 0),
            Vec2D::new(2, 2),
            Vec2D::new(1, 1),
            Vec2D::new(1, 0),
        ]
        .iter()
        .copied()
        .collect();
        assert_eq!(box2d, Box2D::new(Vec2D::new(0, 0), Vec2D::new(2, 2)));
    }
}
