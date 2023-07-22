use bevy::prelude::*;
use rand::Rng;

use crate::{AutoCube, Bounds, ChunkState, ChunkStates, ColorChannels, Rect, SCALE};

fn _spawn_block(
    mut commands: Commands,
    block: AutoCube,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_states: ResMut<ChunkStates>,
) {
    let c = ChunkState {
        playing: true,
        life_time: 100,
        base_color: Color::rgb(1.0, 0.7, 0.0),
        scale: 1.0,
        inter_color: ColorChannels::R,
        perceptual_roughness: 0.0,
        bounds: Rect {
            x: -1.,
            y: -1.,
            w: 2.,
            h: 4.,
        }
        .into(),
    };

    chunk_states.0.push(c);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {
                size: rand::thread_rng().gen_range(0.01..0.5),
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.7, 0.0),
                perceptual_roughness: 0.08,
                reflectance: 0.1,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0) * SCALE),
            ..default()
        })
        .insert(block);
}

pub fn spawn_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chunks: Res<ChunkStates>,
) {
    let chunks = &chunks.0;
    for _ in 0..4 {
        let mut index = 0;
        for chunk in chunks.iter() {
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
                .insert(AutoCube { index, ..default() });
            index += 1;
        }
    }
}
