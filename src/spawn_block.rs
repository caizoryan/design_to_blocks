use bevy::prelude::*;
use rand::Rng;

use crate::{AutoCube, SCALE};

pub fn spawn_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in 0..20 {
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
}
