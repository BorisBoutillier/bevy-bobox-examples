use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2dPlugin;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};

use bevy::prelude::*;
use bevy::window::{WindowId, WindowResized};
use rand::{thread_rng, Rng};

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 800.0;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BackgroundPlugin {})
        .add_startup_system(setup)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

// Plugin that will insert a background at Z = -10.0, use the custom 'Star Nest' shader
pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<BackgroundMaterial>::default())
            .add_startup_system(spawn_background)
            .add_system(update_background_material)
            .add_system(update_background_quad_on_resizing);
    }
}

// Spawn a simple stretched quad that will use of backgound shader
fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    // Choose a random f32 for start_time, to have different background
    let mut rng = thread_rng();
    let start_time = rng.gen_range(0.0..100.0f32);

    // The support for the shader is a Quad scaled to the window width/height
    // Using our custom BackgroundMaterial
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::new(WIDTH, HEIGHT, 1.0)),
        material: materials.add(BackgroundMaterial { time: start_time }),
        ..Default::default()
    });
}

// Time is passed through our BackgroundMaterial
// So we need to update its time attribute
fn update_background_material(
    time: Res<Time>,
    mut background_materials: ResMut<Assets<BackgroundMaterial>>,
) {
    for (_id, mut background_material) in background_materials.iter_mut() {
        background_material.time += time.delta_seconds();
    }
}

fn update_background_quad_on_resizing(
    windows: Res<Windows>,
    mut resize_events: EventReader<WindowResized>,
    mut background_transforms: Query<&mut Transform, With<Handle<BackgroundMaterial>>>,
) {
    for resize_event in resize_events.iter() {
        if resize_event.id == WindowId::primary() {
            let window = windows.primary();
            for mut background_transform in background_transforms.iter_mut() {
                background_transform.scale = Vec3::new(window.width(), window.height(), 1.0)
            }
        }
    }
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "d1776d38-712a-11ec-90d6-0242ac120003"]
struct BackgroundMaterial {
    #[uniform(0)]
    time: f32,
}
impl Material2d for BackgroundMaterial {
    fn vertex_shader() -> ShaderRef {
        "shader_background.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shader_background.wgsl".into()
    }
}
