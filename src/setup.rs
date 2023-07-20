use bevy::{
    core_pipeline::{
        bloom::{BloomPrefilterSettings, BloomSettings},
        experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    },
    pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings},
    prelude::*,
};
use bevy_panorbit_camera::PanOrbitCamera;

pub fn setup(mut commands: Commands) {
    // camera
    commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 100.0, 15.0) * crate::SCALE)
                    .looking_at(Vec3::default(), Vec3::Y),
                ..Default::default()
            },
            BloomSettings {
                intensity: -4.2,
                prefilter_settings: BloomPrefilterSettings {
                    threshold: 0.5,
                    threshold_softness: 0.5,
                    ..default()
                },
                ..default()
            },
        ))
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
