use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{
        mesh::Indices,
        render_resource::{AddressMode, PrimitiveTopology, SamplerDescriptor},
    },
};
use bevy_inspector_egui::{prelude::*, quick::ResourceInspectorPlugin};
use hexx::*;

#[derive(Debug, Resource)]
struct HexInfo {
    pub layout: HexLayout,
    pub mesh_entity: Entity,
    pub mesh_handle: Handle<Mesh>,
}

#[derive(Debug, Reflect)]
struct UVParams {
    pub uv_offset: Vec2,
    pub uv_scale_factor: Vec2,
    pub uv_flip: BVec2,
}

#[derive(Debug, Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct BuilderParams {
    #[inspector(min = 0.0, max = 50.0)]
    pub height: f32,
    #[inspector(min = 1, max = 50)]
    pub subdivisions: usize,
    pub top_face: bool,
    pub bottom_face: bool,
    pub sides_uvs: UVParams,
    pub caps_uvs: UVParams,
}

pub fn main() {
    App::new()
        .register_type::<BuilderParams>()
        .init_resource::<BuilderParams>()
        .insert_resource(AmbientLight {
            brightness: 0.1,
            ..default()
        })
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            // We set the textures to be repeated
            default_sampler: SamplerDescriptor {
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                address_mode_w: AddressMode::Repeat,
                ..Default::default()
            },
        }))
        .add_plugin(WireframePlugin)
        .add_plugin(ResourceInspectorPlugin::<BuilderParams>::default())
        .add_startup_system(setup)
        .add_system(animate)
        .add_system(update_mesh)
        .run();
}

/// 3D Orthogrpahic camera setup
fn setup(
    mut commands: Commands,
    params: Res<BuilderParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("uv_checker.png");
    let transform = Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn(Camera3dBundle {
        transform,
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        transform,
        ..default()
    });
    let layout = HexLayout::default();
    let mesh = ColumnMeshBuilder::new(&layout, params.height)
        .with_subdivisions(params.subdivisions)
        .with_offset(Vec3::NEG_Y * params.height / 2.0)
        .build();
    let mesh_handle = meshes.add(compute_mesh(mesh));
    let material = materials.add(texture.into());
    let mesh_entity = commands
        .spawn((
            PbrBundle {
                mesh: mesh_handle.clone(),
                material,
                ..default()
            },
            Wireframe,
        ))
        .id();
    commands.insert_resource(HexInfo {
        layout,
        mesh_entity,
        mesh_handle,
    });
}

fn animate(info: Res<HexInfo>, mut transforms: Query<&mut Transform>, time: Res<Time>) {
    let delta_time = time.delta_seconds() / 2.0;
    let mut transform = transforms.get_mut(info.mesh_entity).unwrap();
    transform.rotate_x(delta_time);
    transform.rotate_y(delta_time);
    transform.rotate_local_y(delta_time);
    transform.rotate_z(delta_time);
}

fn update_mesh(params: Res<BuilderParams>, info: Res<HexInfo>, mut meshes: ResMut<Assets<Mesh>>) {
    if !params.is_changed() {
        return;
    }
    let mut new_mesh = ColumnMeshBuilder::new(&info.layout, params.height)
        .with_subdivisions(params.subdivisions)
        .with_offset(Vec3::NEG_Y * params.height / 2.0)
        .with_caps_uv_options(UVOptions {
            scale_factor: params.caps_uvs.uv_scale_factor,
            flip_u: params.caps_uvs.uv_flip.x,
            flip_v: params.caps_uvs.uv_flip.y,
            offset: params.caps_uvs.uv_offset,
        })
        .with_sides_uv_options(UVOptions {
            scale_factor: params.sides_uvs.uv_scale_factor,
            flip_u: params.sides_uvs.uv_flip.x,
            flip_v: params.sides_uvs.uv_flip.y,
            offset: params.sides_uvs.uv_offset,
        });
    if !params.top_face {
        new_mesh = new_mesh.without_top_face();
    }
    if !params.bottom_face {
        new_mesh = new_mesh.without_bottom_face();
    }
    let new_mesh = compute_mesh(new_mesh.build());
    // println!("Mesh has {} vertices", new_mesh.count_vertices());
    let mesh = meshes.get_mut(&info.mesh_handle).unwrap();
    *mesh = new_mesh;
}

/// Compute a bevy mesh from the layout
fn compute_mesh(mesh_info: MeshInfo) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}

impl Default for BuilderParams {
    fn default() -> Self {
        Self {
            height: 10.0,
            subdivisions: 3,
            top_face: true,
            bottom_face: true,
            sides_uvs: UVParams::default(),
            caps_uvs: UVParams::default(),
        }
    }
}

impl Default for UVParams {
    fn default() -> Self {
        Self {
            uv_offset: Vec2::default(),
            uv_flip: BVec2::default(),
            uv_scale_factor: Vec2::ONE,
        }
    }
}
