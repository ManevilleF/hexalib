use super::{utils::Hexagon, MeshInfo, BASE_FACING};
use crate::{Hex, HexLayout, InsetOptions, UVOptions};
use glam::{Quat, Vec3};

/// Builder struct to customize hex plane mesh generation.
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
pub struct PlaneMeshBuilder<'l> {
    /// The hexagonal layout, used to compute vertex positions
    pub layout: &'l HexLayout,
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
    /// Optional inset options for the plane face
    pub inset_options: Option<InsetOptions>,
}

impl<'l> PlaneMeshBuilder<'l> {
    /// Setup a new builder using the given `layout`
    #[must_use]
    pub const fn new(layout: &'l HexLayout) -> Self {
        Self {
            layout,
            pos: Hex::ZERO,
            rotation: None,
            offset: None,
            scale: None,
            uv_options: UVOptions::new(),
            center_aligned: false,
            inset_options: None,
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

    /// Specify insetting option for the hexagonal face
    ///
    /// # Arguments
    ///
    /// * `scale` the scale of the new insetted vertices,
    /// * `keep_inner_face` - If set to true the insetted face will be kept, otherwise
    /// it will be removed
    #[must_use]
    #[inline]
    pub const fn with_inset_face(mut self, scale: f32, keep_inner_face: bool) -> Self {
        self.inset_options = Some(InsetOptions {
            keep_inner_face,
            scale,
        });
        self
    }

    /// Comsumes the builder to return the computed mesh data
    #[must_use]
    pub fn build(self) -> MeshInfo {
        // We compute the mesh at the origin and no offset to allow scaling
        let face = Hexagon::center_aligned(self.layout);
        // We store the offset to match the `self.pos`
        let pos = if self.center_aligned {
            self.layout.hex_to_center_aligned_world_pos(self.pos)
        } else {
            self.layout.hex_to_world_pos(self.pos)
        };
        let mut offset = Vec3::new(pos.x, 0.0, pos.y);
        // We apply optional insetting
        let mut mesh = if let Some(inset) = self.inset_options {
            face.inset(inset.scale, inset.keep_inner_face)
        } else {
            face.into()
        };
        // **S** - We apply optional scale
        if let Some(scale) = self.scale {
            mesh = mesh.with_scale(scale);
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
