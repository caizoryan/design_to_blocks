use bevy::prelude::*;
use rand::Rng;

use crate::{AutoCube, Bounds, Rect, SCALE};

pub fn spawn_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let bounds: [Bounds; 2] = [
        Rect {
            x: -1.,
            y: -1.,
            w: 2.,
            h: 4.,
        }
        .into(),
        Rect {
            x: 1.5,
            y: 2.5,
            w: 1.,
            h: 3.,
        }
        .into(),
    ];

    for _ in 0..4 {
        let mut index = 0;
        for bound in &bounds {
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
                    bounds: bound.clone(),
                    index,
                    ..default()
                });
            index += 1;
        }
    }
}
