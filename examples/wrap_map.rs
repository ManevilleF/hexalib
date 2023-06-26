use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    utils::HashMap,
    window::PrimaryWindow,
};
use hexx::*;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(15.0);
const MAP_RADIUS: u32 = 10;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(setup_grid)
        .add_system(handle_input)
        .run();
}

#[derive(Debug, Resource)]
struct HexGrid {
    pub entities: HashMap<Hex, Entity>,
    pub layout: HexLayout,
    pub bounds: HexBounds,
    pub default_mat: Handle<ColorMaterial>,
    pub selected_mat: Handle<ColorMaterial>,
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };
    let mesh = meshes.add(hexagonal_plane(&layout));
    let default_mat = materials.add(Color::WHITE.into());
    let selected_mat = materials.add(Color::RED.into());
    let bounds = HexBounds::new(Hex::ZERO, MAP_RADIUS);
    let entities = bounds
        .all_coords()
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let entity = commands
                .spawn(ColorMesh2dBundle {
                    mesh: mesh.clone().into(),
                    material: default_mat.clone(),
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                    ..default()
                })
                .id();
            (hex, entity)
        })
        .collect();
    commands.insert_resource(HexGrid {
        entities,
        layout,
        bounds,
        default_mat,
        selected_mat,
    })
}

/// Input interaction
fn handle_input(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    grid: Res<HexGrid>,
    mut current_hex: Local<Hex>,
) {
    let window = windows.single();
    if let Some(pos) = window.cursor_position() {
        let pos = pos - Vec2::new(window.width(), window.height()) / 2.0;
        let hex_pos = grid.layout.world_pos_to_hex(pos);
        if hex_pos == *current_hex {
            return;
        }
        let wrapped = grid.bounds.wrap(hex_pos);
        commands
            .entity(grid.entities[&current_hex])
            .insert(grid.default_mat.clone());
        commands
            .entity(grid.entities[&wrapped])
            .insert(grid.selected_mat.clone());
        *current_hex = wrapped;
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout).facing(Vec3::Z).build();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}
