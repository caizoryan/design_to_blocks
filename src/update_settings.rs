use bevy::{
    core_pipeline::bloom::BloomSettings,
    prelude::{Input, KeyCode, Query, Res, ResMut},
    time::Time,
};

use crate::SelectedIndex;

pub fn update_settings(
    mut camera: Query<&mut BloomSettings>,
    mut index: ResMut<SelectedIndex>,
    mut variables: ResMut<crate::ChunkStates>,
    keycode: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut bloom_settings = camera.single_mut();

    let dt = time.delta_seconds();

    // ----------------------------------------
    // Hide and show ui
    // ----------------------------------------
    if keycode.just_pressed(KeyCode::H) {
        index.0 = match index.0 {
            Some(_) => None,
            None => Some(0),
        };
    }

    // ----------------------------------------
    // Bloom
    // ----------------------------------------
    if keycode.pressed(KeyCode::Q) {
        bloom_settings.prefilter_settings.threshold -= dt;
    }
    if keycode.pressed(KeyCode::W) {
        bloom_settings.prefilter_settings.threshold += dt;
    }

    if keycode.pressed(KeyCode::E) {
        bloom_settings.prefilter_settings.threshold_softness -= dt;
    }
    if keycode.pressed(KeyCode::R) {
        bloom_settings.prefilter_settings.threshold_softness += dt;
    }

    if keycode.pressed(KeyCode::D) {
        bloom_settings.intensity -= dt;
    }
    if keycode.pressed(KeyCode::F) {
        bloom_settings.intensity += dt;
    }

    // ----------------------------------------
    // make selection to which block
    // ----------------------------------------
    let shift = keycode.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    if keycode.pressed(KeyCode::Q) && shift {
        index.0 = Some(0);
    }
    if keycode.pressed(KeyCode::Y) && shift {
        index.0 = Some(1);
    }

    // ----------------------------------------
    //  Move bounds
    // ----------------------------------------
    let _ = match index.0 {
        Some(index) => {
            if keycode.pressed(KeyCode::Left) {
                variables.0[index].bounds.0.x -= 0.05;
                variables.0[index].bounds.1.x -= 0.05;
            }
            if keycode.pressed(KeyCode::Right) {
                variables.0[index].bounds.0.x += 0.05;
                variables.0[index].bounds.1.x += 0.05;
            }
            if keycode.pressed(KeyCode::Up) {
                variables.0[index].bounds.0.y += 0.05;
                variables.0[index].bounds.1.y += 0.05;
            }
            if keycode.pressed(KeyCode::Down) {
                variables.0[index].bounds.0.y -= 0.05;
                variables.0[index].bounds.1.y -= 0.05;
            }
        }
        None => (),
    };
}
