use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Widget},
    EguiContexts,
};

use crate::AutoCube;

pub fn update_egui(
    mut contexts: EguiContexts,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut main_cube_query: Query<&Handle<StandardMaterial>, With<AutoCube>>,
) {
    let handle = main_cube_query.single();
    let mut material = materials.get(handle).unwrap().clone();

    let ctx = contexts.ctx_mut();

    egui::Window::new("Cube material preview").show(ctx, |ui| {
        egui::Grid::new("preview").show(ui, |ui| {
            ui.label("Base color:");
            color_picker_widget(ui, &mut material.base_color);
            ui.end_row();

            ui.label("Emissive:");
            color_picker_widget(ui, &mut material.emissive);
            ui.end_row();

            ui.label("Perceptual roughness:");
            egui::Slider::new(&mut material.perceptual_roughness, 0.089..=1.0).ui(ui);
            ui.end_row();

            ui.label("Reflectance:");
            egui::Slider::new(&mut material.reflectance, 0.0..=1.0).ui(ui);
            ui.end_row();

            ui.label("Unlit:");
            ui.checkbox(&mut material.unlit, "");
            ui.end_row();
        });
    });

    for set_material in main_cube_query.iter_mut() {
        let _ = materials.set(set_material, material.clone());
    }
}

fn color_picker_widget(ui: &mut egui::Ui, color: &mut Color) -> egui::Response {
    let [r, g, b, a] = color.as_rgba_f32();
    let mut egui_color: egui::Rgba = egui::Rgba::from_srgba_unmultiplied(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    );
    let res = egui::widgets::color_picker::color_edit_button_rgba(
        ui,
        &mut egui_color,
        egui::color_picker::Alpha::Opaque,
    );
    let [r, g, b, a] = egui_color.to_srgba_unmultiplied();
    *color = [
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    ]
    .into();
    res
}
