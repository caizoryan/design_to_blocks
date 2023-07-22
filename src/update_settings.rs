use bevy::{
    core_pipeline::bloom::BloomSettings,
    prelude::{Input, KeyCode, Query, Res, ResMut},
    time::Time,
};

use crate::SelectedIndex;

pub fn update_settings(
    mut camera: Query<&mut BloomSettings>,
    mut index: ResMut<SelectedIndex>,
    keycode: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut bloom_settings = camera.single_mut();

    let dt = time.delta_seconds();

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
    if keycode.pressed(KeyCode::Key1) {
        index.0 = Some(0);
    }
    if keycode.pressed(KeyCode::Key2) {
        index.0 = Some(1);
    }
}
