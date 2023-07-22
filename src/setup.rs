use bevy::{
    core_pipeline::{
        bloom::{BloomPrefilterSettings, BloomSettings},
        experimental::taa::TemporalAntiAliasBundle,
    },
    pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings},
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_panorbit_camera::PanOrbitCamera;

pub fn setup(mut commands: Commands) {
    // camera
    commands
        .spawn((
            Camera3dBundle {
                projection: OrthographicProjection {
                    scale: 3.0,
                    scaling_mode: ScalingMode::FixedVertical(2.0),
                    ..default()
                }
                .into(),
                camera: Camera {
                    hdr: true,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 10.0) * crate::SCALE,
                    ..Default::default()
                },
                ..Default::default()
            },
            BloomSettings { ..default() },
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
