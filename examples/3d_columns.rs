use bevy::{
    color::palettes::css::{WHITE, YELLOW},
    prelude::*,
    render::{
        extract_component::ExtractComponent, mesh::Indices, render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
    time::common_conditions::on_timer,
};
use hexx::{shapes, *};
use std::{collections::HashMap, time::Duration};

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(1.0);
/// World space height of hex columns
const COLUMN_HEIGHT: f32 = 10.0;
/// Map radius
const MAP_RADIUS: u32 = 20;
/// Animation time step
const TIME_STEP: Duration = Duration::from_millis(100);

pub fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 200.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, animate_rings.run_if(on_timer(TIME_STEP)))
        .run();
}

#[derive(
    Component, Clone, Debug, Default, Deref, DerefMut, Reflect, PartialEq, Eq, ExtractComponent,
)]
#[reflect(Component, Default)]
pub struct StandardMaterialHandle(pub Handle<StandardMaterial>);

impl From<Handle<StandardMaterial>> for StandardMaterialHandle {
    fn from(handle: Handle<StandardMaterial>) -> Self {
        Self(handle)
    }
}

impl From<StandardMaterialHandle> for AssetId<StandardMaterial> {
    fn from(material: StandardMaterialHandle) -> Self {
        material.id()
    }
}

impl From<&StandardMaterialHandle> for AssetId<StandardMaterial> {
    fn from(material: &StandardMaterialHandle) -> Self {
        material.id()
    }
}

#[derive(Debug, Resource)]
struct Map {
    entities: HashMap<Hex, Entity>,
    highlighted_material: StandardMaterialHandle,
    default_material: StandardMaterialHandle,
}

#[derive(Debug, Default, Resource)]
struct HighlightedHexes {
    ring: u32,
    hexes: Vec<Hex>,
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 60.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(60.0, 60.0, 00.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Hex grid setup
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };
    // materials
    let default_material = materials.add(Color::Srgba(WHITE));
    let highlighted_material = materials.add(Color::Srgba(YELLOW));
    // mesh
    let mesh = hexagonal_column(&layout);
    let mesh_handle = meshes.add(mesh);

    let entities = shapes::hexagon(Hex::ZERO, MAP_RADIUS)
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn((
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(default_material.clone()),
                    Transform::from_xyz(pos.x, hex.length() as f32 / 2.0, pos.y),
                ))
                .id();
            (hex, id)
        })
        .collect();
    commands.insert_resource(Map {
        entities,
        highlighted_material: highlighted_material.into(),
        default_material: default_material.into(),
    });
}

fn animate_rings(
    mut commands: Commands,
    map: Res<Map>,
    mut highlighted_hexes: Local<HighlightedHexes>,
) {
    // Clear highlighted hexes materials
    for entity in highlighted_hexes
        .hexes
        .iter()
        .filter_map(|h| map.entities.get(h))
    {
        commands
            .entity(*entity)
            .insert(map.default_material.clone());
    }
    highlighted_hexes.ring += 1;
    if highlighted_hexes.ring > MAP_RADIUS {
        highlighted_hexes.ring = 0;
    }
    highlighted_hexes.hexes = Hex::ZERO.ring(highlighted_hexes.ring).collect();
    // Draw a ring
    for h in &highlighted_hexes.hexes {
        if let Some(e) = map.entities.get(h) {
            commands.entity(*e).insert(map.highlighted_material.clone());
        }
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_column(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, COLUMN_HEIGHT)
        .without_bottom_face()
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
