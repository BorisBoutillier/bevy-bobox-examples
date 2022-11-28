use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2dPlugin;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};

use bevy::prelude::*;
use bevy::window::{WindowId, WindowResized};

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 800.0;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(BackgroundPlugin {})
        .add_startup_system(setup)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Plugin that will insert a background at Z = -10.0, use the custom 'Star Nest' shader
pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<BackgroundMaterial>::default())
            .add_startup_system(spawn_background)
            .add_system(update_background_quad_on_resizing);
    }
}

// Spawn a simple stretched quad that will use of backgound shader
fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    // The support for the shader is a Quad scaled to the window width/height
    // Using our custom BackgroundMaterial
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::new(WIDTH, HEIGHT, 1.0)),
        material: materials.add(BackgroundMaterial {}),
        ..Default::default()
    });
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
struct BackgroundMaterial {}

impl Material2d for BackgroundMaterial {
    fn vertex_shader() -> ShaderRef {
        "shader_background.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shader_background.wgsl".into()
    }
}
