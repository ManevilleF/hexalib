use super::{MeshInfo, BASE_FACING};
use crate::{Hex, HexLayout, UVOptions};
use glam::{Quat, Vec3};

/// Builder struct to customize hex outline mesh generation.
///
/// The mesh will be anchored at the center of the hexagon, use offsets to
/// cutomize anchor/pivot position.
///
/// # Note
///
/// Transform operations (Scale, Rotate, Translate) through the methods
///
/// - Scale: [`Self::with_scale`]
/// - Rotate: [`Self::with_rotation`], [`Self::facing`]
/// - Translate: [`Self::with_offset`], [`Self::at`]
///
/// Are executed in that order, or **SRT**
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct OutlineMeshBuilder<'l> {
    /// The hexagonal layout, used to compute vertex positions
    pub layout: &'l HexLayout,
    /// The larger hexagonal layout
    pub larger_layout: HexLayout,
    /// Custom hex position, will apply an offset if not [`Hex::ZERO`]
    pub pos: Hex,
    /// Optional custom offset for the mesh vertex positions
    pub offset: Option<Vec3>,
    /// Optional custom scale factor for the mesh vertex positions
    pub scale: Option<Vec3>,
    /// Optional custom rotation, useful to have the mesh already
    /// rotated
    ///
    /// By default the mesh is *facing* up (**Y** axis)
    pub rotation: Option<Quat>,
    /// UV mapping options
    pub uv_options: UVOptions,
    /// If set to `true`, the mesh will ignore [`HexLayout::origin`]
    pub center_aligned: bool,
}

impl<'l> OutlineMeshBuilder<'l> {
    /// Setup a new builder using the given `layout`
    #[must_use]
    pub fn new(layout: &'l HexLayout) -> Self {
        let larger_layout = HexLayout {
            orientation: layout.orientation,
            origin: layout.origin,
            hex_size: layout.hex_size * 1.1,
            invert_x: layout.invert_x,
            invert_y: layout.invert_y,
        };
        Self {
            layout,
            larger_layout,
            pos: Hex::ZERO,
            rotation: None,
            offset: None,
            scale: None,
            uv_options: UVOptions::new(),
            center_aligned: false,
        }
    }

    /// Specifies a custom `pos`, which will apply an offset to the whole mesh.
    ///
    /// ## Note
    ///
    /// It might be more optimal to generate only one mesh at [`Hex::ZERO`] and
    /// offset it later than have one mesh per hex position
    #[must_use]
    pub const fn at(mut self, pos: Hex) -> Self {
        self.pos = pos;
        self
    }

    /// Specify a custom *facing* direction for the mesh, by default the column
    /// is vertical (facing up)
    ///
    /// # Panics
    ///
    /// Will panic if `facing` is zero length
    #[must_use]
    pub fn facing(mut self, facing: Vec3) -> Self {
        self.rotation = Some(Quat::from_rotation_arc(BASE_FACING, facing.normalize()));
        self
    }

    /// Specify a custom rotation for the whole mesh
    #[must_use]
    pub const fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = Some(rotation);
        self
    }

    /// Specify a custom offset for the whole mesh
    #[must_use]
    pub const fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Specify a custom scale factor for the whole mesh
    #[must_use]
    pub const fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = Some(scale);
        self
    }

    /// Specify custom UV mapping options
    #[must_use]
    pub const fn with_uv_options(mut self, uv_options: UVOptions) -> Self {
        self.uv_options = uv_options;
        self
    }

    #[must_use]
    #[inline]
    /// Ignores the [`HexLayout::origin`] offset, generating a mesh centered
    /// around `(0.0, 0.0)`.
    pub const fn center_aligned(mut self) -> Self {
        self.center_aligned = true;
        self
    }

    /// Comsumes the builder to return the computed mesh data
    #[must_use]
    pub fn build(self) -> MeshInfo {
        // We compute the mesh at the origin and no offset to allow scaling
        let mut mesh = MeshInfo::center_aligned_hexagonal_outline(self.layout, &self.larger_layout);
        // We store the offset to match the `self.pos`
        let pos = if self.center_aligned {
            self.layout.hex_to_center_aligned_world_pos(self.pos)
        } else {
            self.layout.hex_to_world_pos(self.pos)
        };
        let mut offset = Vec3::new(pos.x, 0.0, pos.y);
        // **S** - We apply optional scale
        if let Some(scale) = self.scale {
            mesh.vertices.iter_mut().for_each(|p| *p *= scale);
        }
        // **R** - We rotate the mesh to face the given direction
        if let Some(rotation) = self.rotation {
            mesh = mesh.rotated(rotation);
        }
        // **T** - We offset the vertex positions after scaling and rotating
        if let Some(custom_offset) = self.offset {
            offset += custom_offset;
        }
        mesh = mesh.with_offset(offset);
        self.uv_options.alter_uvs(&mut mesh.uvs);
        mesh
    }
}
