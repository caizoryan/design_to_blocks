// - [ ] keep track of which block is being edited
// - [ ] use that in egui to update the data
// - [ ] in update block make sure you read the correct data from variables array

mod egui;
mod setup;
mod spawn_block;
mod update_block;
mod update_settings;

use egui::update_egui;
use setup::setup;
use spawn_block::spawn_block;
use update_block::update_block;

use bevy::{core_pipeline::experimental::taa::TemporalAntiAliasPlugin, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use update_settings::update_settings;

#[derive(Resource)]
pub struct SelectedIndex(Option<usize>);

#[derive(Resource)]
pub struct ChunkStates(Vec<ChunkState>);

#[derive(Clone)]
pub struct ChunkState {
    pub playing: bool,
    pub life_time: i32,
    pub base_color: Color,
    pub scale: f32,
    pub inter_color: ColorChannels,
    pub perceptual_roughness: f32,
    pub bounds: Bounds,
}

#[derive(Clone)]
pub struct Bounds(Vec3, Vec3);

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Into<Bounds> for Rect {
    fn into(self) -> Bounds {
        Bounds(
            Vec3::new(self.x, self.y, 0.0),
            Vec3::new(self.x + self.w, self.y + self.h, 0.0),
        )
    }
}

#[derive(Component)]
pub struct AutoCube {
    pub index: usize,
    pub life_time: i32,
}

struct Temp(f32, f32, f32, f32);

impl Into<Temp> for Color {
    fn into(self) -> Temp {
        Temp(self.r(), self.g(), self.b(), self.a())
    }
}

impl Into<Color> for Temp {
    fn into(self) -> Color {
        Color::rgba(self.0, self.1, self.2, self.3)
    }
}

impl Default for AutoCube {
    fn default() -> Self {
        AutoCube {
            index: 0,
            life_time: LIFETIME,
        }
    }
}

const LIFETIME: i32 = 100;
pub const SCALE: f32 = 3.;

#[derive(PartialEq, Eq, Clone)]
pub enum ColorChannels {
    R,
    G,
    B,
    A,
}

fn main() {
    let block_1 = ChunkState {
        playing: true,
        life_time: LIFETIME,
        scale: SCALE,
        perceptual_roughness: 0.5,
        base_color: Color::rgb(0.09, 0.0, 0.0),
        inter_color: ColorChannels::R,
        bounds: Rect {
            x: -1.,
            y: -1.,
            w: 2.,
            h: 4.,
        }
        .into(),
    };

    let block_2 = ChunkState {
        playing: true,
        life_time: LIFETIME,
        scale: SCALE,
        perceptual_roughness: 0.5,
        base_color: Color::rgb(0.0, 0.09, 0.0),
        inter_color: ColorChannels::G,
        bounds: Rect {
            x: 1.5,
            y: 2.5,
            w: 1.,
            h: 3.,
        }
        .into(),
    };

    let chunk_states = ChunkStates(vec![block_1, block_2]);

    App::new()
        .insert_resource(AmbientLight {
            brightness: 3.0,
            ..default()
        })
        .insert_resource(SelectedIndex(None))
        .insert_resource(chunk_states)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(TemporalAntiAliasPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_block)
        .add_systems(FixedUpdate, update_block)
        .add_systems(Update, update_egui)
        .add_systems(Update, update_settings)
        .insert_resource(FixedTime::new_from_secs(0.02))
        .run();
}
