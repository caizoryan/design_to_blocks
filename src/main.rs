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
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use update_settings::update_bloom_settings;

#[derive(Resource)]
struct KeyPressed {
    pressed: bool,
}

const LIFETIME: i32 = 100;
pub const SCALE: f32 = 3.;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: 3.0,
            ..default()
        })
        .insert_resource(KeyPressed { pressed: false })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(TemporalAntiAliasPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_block)
        .add_systems(FixedUpdate, update_block)
        .add_systems(Update, keyboard_input_system)
        // .add_systems(Update, update_egui)
        .add_systems(Update, update_bloom_settings)
        .insert_resource(FixedTime::new_from_secs(0.02))
        .run();
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut pressed: ResMut<KeyPressed>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Space pressed");
        let cur = pressed.pressed;
        pressed.pressed = !cur;
    }
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
            Vec3::new(self.x, self.y, 0.0) * SCALE,
            Vec3::new(self.x + self.w, self.y + self.h, 0.0) * SCALE,
        )
    }
}

#[derive(Component)]
pub struct AutoCube {
    pub life_time: i32,
    pub bounds: Bounds,
}

impl Default for AutoCube {
    fn default() -> Self {
        AutoCube {
            life_time: LIFETIME,
            bounds: Bounds(
                Vec3::new(10.0, 10.0, 10.0) * SCALE,
                Vec3::new(10.0, 10.0, 10.0) * SCALE,
            ),
        }
    }
}
