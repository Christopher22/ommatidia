mod estimates;

use serde::{self, Deserialize, Serialize};

pub use self::estimates::Estimates;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Estimate {
    Point(Point),
    Ellipse(Ellipse),
}

impl From<Point> for Estimate {
    fn from(point: Point) -> Self {
        Estimate::Point(point)
    }
}

impl From<Ellipse> for Estimate {
    fn from(ellipse: Ellipse) -> Self {
        Estimate::Ellipse(ellipse)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Radian(pub f32);

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    #[serde(flatten)]
    pub pos: Position,
    pub confidence: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ellipse {
    #[serde(flatten)]
    pub pos: Position,
    pub major: f32,
    pub minor: f32,
    pub rotation: Radian,
    pub confidence: Option<f32>,
}

#[cfg(test)]
mod tests {
    use super::{Ellipse, Estimate, Point, Position, Radian};

    #[test]
    fn test_ellipse() {
        const EXAMPLE: &str = r#"{ "type": "Ellipse", "x": 1, "y": 2, "major": 4, "minor": 3, "rotation": 0.5, "confidence": 0.1 }"#;
        let ellipse: Estimate = serde_json::from_str(EXAMPLE).expect("valid JSON");

        assert_eq!(
            ellipse,
            Estimate::Ellipse(Ellipse {
                pos: Position { x: 1, y: 2 },
                major: 4.0,
                minor: 3.0,
                rotation: Radian(0.5),
                confidence: Some(0.1)
            })
        )
    }

    #[test]
    fn test_ellipse_without_confidence() {
        const EXAMPLE: &str =
            r#"{ "type": "Ellipse", "x": 1, "y": 2, "major": 4, "minor": 3, "rotation": 0.5}"#;
        let ellipse: Estimate = serde_json::from_str(EXAMPLE).expect("valid JSON");

        assert_eq!(
            ellipse,
            Estimate::Ellipse(Ellipse {
                pos: Position { x: 1, y: 2 },
                major: 4.0,
                minor: 3.0,
                rotation: Radian(0.5),
                confidence: None
            })
        )
    }

    #[test]
    fn test_point() {
        const EXAMPLE: &str = r#"{ "type": "Point", "x": 1, "y": 2, "confidence": 0.1 }"#;
        let point: Estimate = serde_json::from_str(EXAMPLE).expect("valid JSON");

        assert_eq!(
            point,
            Estimate::Point(Point {
                pos: Position { x: 1, y: 2 },
                confidence: Some(0.1)
            })
        )
    }

    #[test]
    fn test_point_without_confidence() {
        const EXAMPLE: &str = r#"{ "type": "Point", "x": 1, "y": 2 }"#;
        let point: Estimate = serde_json::from_str(EXAMPLE).expect("valid JSON");

        assert_eq!(
            point,
            Estimate::Point(Point {
                pos: Position { x: 1, y: 2 },
                confidence: None
            })
        )
    }
}
