use crate::{Direction, Hex, HexOrientation, SQRT_3};
use glam::Vec2;

/// Hexagonal layout. This type is the bridge between your *world*/*pixel*
/// coordinate system and the hexagonal coordinate system.
///
/// # Axis
///
/// By default, the [`Hex`] `y` axis is pointing down and the `x` axis is
/// pointing right but you have the option to invert them using `invert_x` and
/// `invert_y` This may be useful depending on the coordinate system of your
/// display.
///
/// # Example
///
/// ```rust
/// # use hexx::*;
///
/// let layout = HexLayout {
///     // We want flat topped hexagons
///     orientation: HexOrientation::Flat,
///     // We define the world space origin equivalent of `Hex::ZERO` in hex space
///     origin: Vec2::new(1.0, 2.0),
///     // We define the world space size of the hexagons
///     hex_size: Vec2::new(1.0, 1.0),
///     // We invert the y axis
///     invert_y: true,
///     // But not the x axis
///     invert_x: false,
/// };
/// // You can now find the world positon (center) of any given hexagon
/// let world_pos = layout.hex_to_world_pos(Hex::ZERO);
/// // You can also find which hexagon is at a given world/screen position
/// let hex_pos = layout.world_pos_to_hex(Vec2::new(1.23, 45.678));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct HexLayout {
    /// The hexagonal orientation of the layout (usually "flat" or "pointy")
    pub orientation: HexOrientation,
    /// The origin of the hexagonal representation in world/pixel space, usually
    /// [`Vec2::ZERO`]
    pub origin: Vec2,
    /// The size of individual hexagons in world/pixel space. The size can be
    /// irregular
    pub hex_size: Vec2,
    /// If set to `true`, the `Hex` `x` axis will be inverted
    pub invert_x: bool,
    /// If set to `true`, the `Hex` `y` axis will be inverted
    pub invert_y: bool,
}

impl HexLayout {
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    /// Computes hexagonal coordinates `hex` into world/pixel coordinates
    pub fn hex_to_world_pos(&self, hex: Hex) -> Vec2 {
        let matrix = self.orientation.forward_matrix;
        Vec2::new(
            matrix[0].mul_add(hex.x() as f32, matrix[1] * hex.y() as f32),
            matrix[2].mul_add(hex.x() as f32, matrix[3] * hex.y() as f32),
        ) * self.hex_size
            * self.axis_scale()
            + self.origin
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    #[must_use]
    /// Computes world/pixel coordinates `pos` into hexagonal coordinates
    pub fn world_pos_to_hex(&self, pos: Vec2) -> Hex {
        let matrix = self.orientation.inverse_matrix;
        let point = (pos - self.origin) * self.axis_scale() / self.hex_size;
        Hex::round([
            matrix[0].mul_add(point.x, matrix[1] * point.y),
            matrix[2].mul_add(point.x, matrix[3] * point.y),
        ])
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    /// Retrieves all 6 corner coordinates of the given hexagonal coordinates
    /// `hex`
    pub fn hex_corners(&self, hex: Hex) -> [Vec2; 6] {
        let center = self.hex_to_world_pos(hex);
        Direction::ALL_DIRECTIONS.map(|dir| {
            let angle = dir.angle_pointy() + self.orientation.angle_offset;
            center + Vec2::new(self.hex_size.x * angle.cos(), self.hex_size.y * angle.sin())
        })
    }

    #[inline]
    /// Returns a signum axis coefficient, allowing for inverted axis
    const fn axis_scale(&self) -> Vec2 {
        let x = if self.invert_x { -1.0 } else { 1.0 };
        let y = if self.invert_y { 1.0 } else { -1.0 };
        Vec2::new(x, y)
    }

    #[inline]
    #[must_use]
    /// Returns the size of the bounding box/rect of an hexagon
    /// This uses both the `hex_size` and `orientation` of the layout.
    pub fn rect_size(&self) -> Vec2 {
        self.hex_size
            * match self.orientation {
                HexOrientation::Pointy => Vec2::new(SQRT_3, 2.0),
                HexOrientation::Flat => Vec2::new(2.0, SQRT_3),
            }
    }
}

impl Default for HexLayout {
    fn default() -> Self {
        Self {
            orientation: HexOrientation::default(),
            origin: Vec2::ZERO,
            hex_size: Vec2::ONE,
            invert_x: false,
            invert_y: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_corners() {
        let point = Hex::new(0, 0);
        let layout = HexLayout {
            orientation: HexOrientation::Flat,
            origin: Vec2::ZERO,
            hex_size: Vec2::new(10., 10.),
            ..Default::default()
        };
        let corners = layout.hex_corners(point).map(Vec2::round);
        assert_eq!(
            corners,
            [
                Vec2::new(10.0, 0.0),
                Vec2::new(5.0, 9.0),
                Vec2::new(-5.0, 9.0),
                Vec2::new(-10.0, -0.0),
                Vec2::new(-5.0, -9.0),
                Vec2::new(5.0, -9.0)
            ]
        );
    }

    #[test]
    fn pointy_corners() {
        let point = Hex::new(0, 0);
        let layout = HexLayout {
            orientation: HexOrientation::Pointy,
            origin: Vec2::ZERO,
            hex_size: Vec2::new(10., 10.),
            ..Default::default()
        };
        let corners = layout.hex_corners(point).map(Vec2::round);
        assert_eq!(
            corners,
            [
                Vec2::new(9.0, 5.0),
                Vec2::new(-0.0, 10.0),
                Vec2::new(-9.0, 5.0),
                Vec2::new(-9.0, -5.0),
                Vec2::new(0.0, -10.0),
                Vec2::new(9.0, -5.0)
            ]
        );
    }
}
