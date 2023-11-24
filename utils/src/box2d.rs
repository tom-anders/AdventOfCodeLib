use crate::vec2d::Vec2D;

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

impl Box2D {
    pub fn new(lower: Vec2D, upper: Vec2D) -> Self {
        Self { lower, upper }
    }

    pub fn from_points(points: impl IntoIterator<Item = Vec2D>) -> Self {
        let mut box2d = Self::new(
            Vec2D::new(i64::MAX, i64::MAX),
            Vec2D::new(i64::MIN, i64::MIN),
        );
        for point in points.into_iter() {
            box2d.extend(point);
        }
        box2d
    }

    pub fn contains(&self, point: Vec2D) -> bool {
        point.x >= self.lower.x
            && point.y >= self.lower.y
            && point.x <= self.upper.x
            && point.y <= self.upper.y
    }

    pub fn extend(&mut self, point: Vec2D) {
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
        let points = vec![
            Vec2D::new(0, 0),
            Vec2D::new(2, 2),
            Vec2D::new(1, 1),
            Vec2D::new(1, 0),
        ];
        let box2d = Box2D::from_points(points);
        assert_eq!(box2d, Box2D::new(Vec2D::new(0, 0), Vec2D::new(2, 2)));
    }
}
