use bevy::{
    core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use egui::Widget;
use rand::Rng;

#[derive(Resource)]
struct KeyPressed {
    pressed: bool,
}

const LIFETIME: i32 = 2000;
const SCALE: f32 = 3.;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 3.0,
            ..default()
        })
        .insert_resource(KeyPressed { pressed: false })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(TemporalAntiAliasPlugin)
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
    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 15.0) * SCALE)
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(ScreenSpaceAmbientOcclusionBundle {
            settings: ScreenSpaceAmbientOcclusionSettings {
                quality_level: bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
                ..default()
            },
            ..default()
        })
        .insert(TemporalAntiAliasBundle::default())
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
    bounds: Vec3,
}

impl Default for AutoCube {
    fn default() -> Self {
        AutoCube {
            life_time: LIFETIME,
            bounds: Vec3::new(10.0, 10.0, 10.0) * SCALE,
        }
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
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: rand::thread_rng().gen_range(0.01..0.5),
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.7, 0.0),
                // emissive: Color::rgb(0.8, 0.7, 0.7),
                perceptual_roughness: 0.08,
                reflectance: 0.1,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0) * SCALE),
            ..default()
        })
        .insert(AutoCube {
            bounds: Vec3::new(2.0, 8.0, 2.0) * SCALE,
            ..default()
        });
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: rand::thread_rng().gen_range(0.01 * SCALE..0.5 * SCALE),
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.7, 0.0),
                // emissive: Color::rgb(0.8, 0.7, 0.7),
                perceptual_roughness: 0.08,
                reflectance: 0.1,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(4.0, 2.0, 2.0 * SCALE)),
            ..default()
        })
        .insert(AutoCube {
            bounds: Vec3::new(8.0, 2.0, 2.0) * SCALE,
            ..default()
        });
}

fn update_block(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut blocks: Query<(Entity, &mut AutoCube, &Transform, &Handle<StandardMaterial>)>,
) {
    // let mut rng = rand::thread_rng();
    for (entity, mut block, transform, material) in blocks.iter_mut() {
        if block.life_time == LIFETIME {
            let mut r = rand::thread_rng();
            let r: f32 = r.gen_range(-90.0..90.0);
            let random_rotation = Quat::from_euler(
                EulerRot::XYZ,
                (0.0_f32).to_radians(),
                (r).to_radians(),
                (r).to_radians(),
            );
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube {
                        size: rand::thread_rng().gen_range(0.01 * SCALE..0.3 * SCALE),
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(1., 1.0, 1.0),
                        emissive: Color::rgb(0.2, 0.2, 0.2),
                        ..default()
                    }),
                    transform: Transform {
                        translation: get_random_direction(transform.translation, block.bounds),
                        rotation: random_rotation,
                        ..default()
                    },
                    ..Default::default()
                })
                .insert(AutoCube {
                    bounds: block.bounds,
                    ..default()
                });
        }
        let life_percent = block.life_time as f32 / LIFETIME as f32;

        let m = StandardMaterial {
            base_color: Color::rgb(life_percent, 0., 0.),
            emissive: Color::rgb(life_percent, 0.0, 0.0),
            perceptual_roughness: 0.7,
            ..default()
        };
        let _ = materials.set(material, m);

        block.life_time -= 1;
        if block.life_time == 0 {
            commands.get_entity(entity).unwrap().despawn_recursive();
        }
    }
}

fn get_random_direction(cur: Vec3, bounds: Vec3) -> Vec3 {
    let x = get_random_f32(cur.x, bounds.x);
    let y = get_random_f32(cur.y, bounds.y);
    let z = get_random_f32(cur.z, bounds.z);
    Vec3::new(x, y, z)
}

fn get_random_f32(c: f32, bound: f32) -> f32 {
    let range: f32 = rand::thread_rng().gen_range(-0.3 * SCALE..0.3 * SCALE);
    match c + range {
        y if y > bound => bound,
        y if y < -bound => -bound,
        _ => c + range,
    }
}
