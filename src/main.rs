use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use egui::Widget;
use rand::Rng;

#[derive(Resource)]
struct KeyPressed {
    pressed: bool,
}

fn main() {
    App::new()
        .insert_resource(KeyPressed { pressed: false })
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_block)
        .add_systems(FixedUpdate, update_block)
        .add_systems(Update, keyboard_input_system)
        .insert_resource(FixedTime::new_from_secs(0.02))
        .run();
}

fn setup(mut commands: Commands) {
    // camera
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 15.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(PanOrbitCamera::default());
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut pressed: ResMut<KeyPressed>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Space pressed");
        let cur = pressed.pressed;
        pressed.pressed = !cur;
    }
}

#[derive(Component)]
struct AutoCube {
    life_time: i32,
}

impl Default for AutoCube {
    fn default() -> Self {
        AutoCube { life_time: 100 }
    }
}

// a block that will have x lifetime
// it will spawn a block next to it which will have x life time
// every iteration the blocks that have full life will spawn a new box
// the blocks that have 0 life will be removed
fn spawn_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(AutoCube::default());
}

fn update_block(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut blocks: Query<(Entity, &mut AutoCube, &Transform)>,
) {
    for (entity, mut block, transform) in blocks.iter_mut() {
        if block.life_time == 100 {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
                    material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                    transform: Transform::from_translation(get_random_direction(
                        transform.translation,
                    )),
                    ..Default::default()
                })
                .insert(AutoCube { ..default() });
        }
        block.life_time -= 1;
        if block.life_time == 0 {
            commands.get_entity(entity).unwrap().despawn_recursive();
        }
    }
}

fn get_random_direction(cur: Vec3) -> Vec3 {
    let x = get_random_f32(cur.x);
    let y = get_random_f32(cur.y);
    let z = get_random_f32(cur.z);
    Vec3::new(x, y, z)
}

fn get_random_f32(c: f32) -> f32 {
    let range: f32 = rand::thread_rng().gen_range(-1.0..1.0);
    c + range
}
