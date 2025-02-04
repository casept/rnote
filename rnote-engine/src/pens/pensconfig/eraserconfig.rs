use p2d::bounding_volume::Aabb;
use rnote_compose::penpath::Element;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, num_derive::FromPrimitive, num_derive::ToPrimitive,
)]
#[serde(rename = "eraser_style")]
pub enum EraserStyle {
    #[serde(rename = "trash_colliding_strokes")]
    TrashCollidingStrokes,
    #[serde(rename = "split_colliding_strokes")]
    SplitCollidingStrokes,
}

impl Default for EraserStyle {
    fn default() -> Self {
        Self::TrashCollidingStrokes
    }
}

impl TryFrom<u32> for EraserStyle {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        num_traits::FromPrimitive::from_u32(value).ok_or_else(|| {
            anyhow::anyhow!("EraserStyle try_from::<u32>() for value {} failed", value)
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default, rename = "eraser_config")]
pub struct EraserConfig {
    #[serde(rename = "width")]
    pub width: f64,
    #[serde(rename = "style")]
    pub style: EraserStyle,
}

impl Default for EraserConfig {
    fn default() -> Self {
        Self {
            width: Self::WIDTH_DEFAULT,
            style: EraserStyle::default(),
        }
    }
}

impl EraserConfig {
    pub const WIDTH_MIN: f64 = 1.0;
    pub const WIDTH_MAX: f64 = 500.0;
    pub const WIDTH_DEFAULT: f64 = 12.0;

    pub fn eraser_bounds(&self, element: Element) -> Aabb {
        Aabb::from_half_extents(
            na::Point2::from(element.pos),
            na::Vector2::repeat(self.width * 0.5),
        )
    }
}
