use bevy::prelude::*;
use rand::Rng;

use crate::{AutoCube, LIFETIME, SCALE};
// a block that will have x lifetime
// it will spawn a block next to it which will have x life time
// every iteration the blocks that have full life will spawn a new box
// the blocks that have 0 life will be removed
pub fn update_block(
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
                        size: rand::thread_rng().gen_range(0.01 * SCALE..0.4 * SCALE),
                        ..default()
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(1., 1.0, 1.0),
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
            base_color: Color::rgb(life_percent, 1., 1.),
            // emissive: Color::rgb(life_percent, 1.0, 1.0),
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
    let range: f32 = rand::thread_rng().gen_range(-0.4 * SCALE..0.4 * SCALE);
    match c + range {
        y if y > bound => bound,
        y if y < -bound => -bound,
        _ => c + range,
    }
}
