use bevy::prelude::*;
use rand::Rng;

use crate::{AutoCube, Bounds, ColorChannels, Temp, LIFETIME, SCALE};
// a block that will have x lifetime
// it will spawn a block next to it which will have x life time
// every iteration the blocks that have full life will spawn a new box
// the blocks that have 0 life will be removed

pub fn update_block(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut blocks: Query<(Entity, &mut AutoCube, &Transform, &Handle<StandardMaterial>)>,
    variables: Res<crate::ChunkStates>,
) {
    for (entity, mut block, transform, material) in blocks.iter_mut() {
        let variables = variables.0[block.index].clone();

        if variables.playing {
            if block.life_time == variables.life_time {
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube {
                            size: rand::thread_rng().gen_range(0.01 * SCALE..0.4 * SCALE),
                            ..default()
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: variables.base_color,
                            ..default()
                        }),
                        transform: Transform {
                            translation: get_random_direction(
                                transform.translation,
                                &block.bounds,
                                variables.scale,
                            ),
                            // rotation: random_rotation,
                            ..default()
                        },
                        ..Default::default()
                    })
                    .insert(AutoCube {
                        bounds: block.bounds.clone(),
                        life_time: variables.life_time,
                        index: block.index,
                    });
            }

            let mut c = variables.base_color;
            let life_percent = block.life_time as f32 / variables.life_time as f32;

            let _ = match variables.inter_color {
                ColorChannels::R => c.set_r(life_percent),
                ColorChannels::G => c.set_g(life_percent),
                ColorChannels::B => c.set_b(life_percent),
                ColorChannels::A => c.set_a(life_percent),
            };

            let m = StandardMaterial {
                base_color: c,
                perceptual_roughness: variables.perceptual_roughness,
                ..default()
            };
            let _ = materials.set(material, m);

            block.life_time -= 1;
            if block.life_time == 0 {
                commands.get_entity(entity).unwrap().despawn_recursive();
            }
        }
    }
}

fn get_random_direction(cur: Vec3, bounds: &Bounds, scale: f32) -> Vec3 {
    let x = get_random_f32(cur.x, bounds.0.x, bounds.1.x, scale);
    let y = get_random_f32(cur.y, bounds.0.y, bounds.1.y, scale);
    let z = get_random_f32(cur.z, bounds.0.z, bounds.1.z, scale);
    Vec3::new(x, y, z)
}

fn get_random_f32(c: f32, bound_min: f32, bound_max: f32, scale: f32) -> f32 {
    let range: f32 = rand::thread_rng().gen_range(-0.4 * scale..0.4 * scale);
    let value = range + c;
    value.clamp(bound_min * scale, bound_max * scale)
}
