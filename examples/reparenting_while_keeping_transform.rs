//! Shows how to modify a child [`Transform`] when reparenting so that its [`GlobalTransform`] remain unchanged
//! Tap 'Space' to pause.
//! Tap 'Left' or 'Right' to change the parent of the satellite

use std::f32::consts::PI;

use bevy::prelude::*;

const ROTATION_SPEED: f32 = PI; // Rotation speed in rad/s
const COLORS: &[Color] = &[Color::RED, Color::GREEN];

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(body_rotation)
        .add_system(interaction)
        .run();
}

#[derive(Component)]
pub struct Body;
#[derive(Component)]
pub struct Satellite;

#[derive(Resource)]
pub struct State {
    paused: bool,
    cur_parent_id: usize,
    bodies: Vec<Entity>,
    sat: Entity,
    sat_material: Handle<StandardMaterial>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn lights and camera for the scene
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(-2.0, 2.0, -1.0),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(2.0, 2.0, 1.0),
        ..Default::default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 7.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    // Spawn the two main balls, 'body0' and 'body1', around which the small ball 'satellite' will rotate.
    // Start by having the 'satellite' rotate around 'body0'

    // Bodies
    let stripes = asset_server.load("stripes.png");
    let body_mesh = meshes.add(Mesh::from(shape::UVSphere {
        radius: 0.5,
        ..default()
    }));
    let bodies = (0..=1)
        .map(|i| {
            let material = materials.add(StandardMaterial {
                base_color: COLORS[i],
                base_color_texture: Some(stripes.clone()),
                ..default()
            });
            commands
                .spawn((
                    MaterialMeshBundle {
                        mesh: body_mesh.clone(),
                        material,
                        transform: Transform::from_xyz(-1.0 + 2.0 * i as f32, 0.0, 0.0)
                            .with_rotation(Quat::from_rotation_x(PI / 2.0)), // Rotation so that 'north pole' is 'up'
                        ..default()
                    },
                    Body,
                    Name::new(format!("Body{}", i)),
                ))
                .id()
        })
        .collect::<Vec<_>>();
    // Satellite
    let sat_mesh = meshes.add(Mesh::from(shape::UVSphere {
        radius: 0.1,
        ..default()
    }));
    let sat_material = materials.add(StandardMaterial {
        base_color: COLORS[0],
        ..default()
    });
    let sat = commands
        .spawn((
            MaterialMeshBundle {
                mesh: sat_mesh,
                material: sat_material.clone(),
                transform: Transform::from_xyz(1.0, 0.0, 0.0),
                ..default()
            },
            Satellite,
            Name::new("Satellite"),
        ))
        .id();
    commands.entity(bodies[0]).add_child(sat);

    // Keep a resource with everything handy for systems
    commands.insert_resource(State {
        paused: false,
        cur_parent_id: 0,
        bodies,
        sat,
        sat_material,
    });

    // Instructions in stdout
    println!("");
    println!("");
    println!("-----------------------------");
    println!("[SPACE] To pause");
    println!("[LEFT ARROW or RIGHT ARROW] To change the parent of the satellite");
    println!("-----------------------------");
}

pub fn body_rotation(
    time: Res<Time>,
    state: Res<State>,
    mut bodies: Query<&mut Transform, With<Body>>,
) {
    if !state.paused {
        let rot = Quat::from_rotation_y(ROTATION_SPEED * time.delta_seconds());
        for mut transform in bodies.iter_mut() {
            transform.rotate(rot);
        }
    }
}

pub fn interaction(
    mut commands: Commands,
    mut state: ResMut<State>,
    input: Res<Input<KeyCode>>,
    transforms: Query<&Transform>,
    global_transforms: Query<&GlobalTransform>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Space to pause/unpause
    if input.just_pressed(KeyCode::Space) {
        state.paused = !state.paused;
    }
    // Any of Left/Right arrow will reparent the satellite to the other body
    if input.just_pressed(KeyCode::Left) || input.just_pressed(KeyCode::Right) {
        let new_parent_id = usize::from(state.cur_parent_id == 0);
        let new_parent = state.bodies[new_parent_id];
        let cur_parent = state.bodies[state.cur_parent_id];
        // Update satellite color to match new parent color
        let mut sat_material = materials
            .get_mut(&state.sat_material)
            .expect("Where is it ?");
        sat_material.base_color = COLORS[new_parent_id];
        // Update satellite transform so that its global transform remains invariant after reparenting.
        // After reparent we will have  : sat_Global  = new_parent_Transform * sat_new_Transform
        // left multiplying be new_parent_Transform.inverse() we get
        // new_parent_Transform.inverse() * sat_Global = sat_new_Transform
        let sat_global_mat4 = global_transforms
            .get(state.sat)
            .expect("Where is it ?")
            .compute_matrix();
        let new_parent_mat4 = transforms
            .get(new_parent)
            .expect("Where is it ?")
            .compute_matrix();
        commands.entity(state.sat).insert(Transform::from_matrix(
            new_parent_mat4.inverse() * sat_global_mat4,
        ));
        // Update Parent/Children components
        commands.entity(cur_parent).remove_children(&[state.sat]);
        commands.entity(new_parent).add_child(state.sat);

        state.cur_parent_id = new_parent_id;
    }
}
