use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use crate::Variables;

pub fn update_egui(mut contexts: EguiContexts, mut variables: ResMut<Variables>) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Cube material preview").show(ctx, |ui| {
        egui::Grid::new("preview").show(ui, |ui| {
            ui.label("Base color:");
            color_picker_widget(ui, &mut variables.base_color);
            ui.end_row();

            // ui.label("Emissive:");
            // color_picker_widget(ui, &mut material.emissive);
            // ui.end_row();

            // ui.label("Perceptual roughness:");
            // egui::Slider::new(&mut material.perceptual_roughness, 0.089..=1.0).ui(ui);
            // ui.end_row();
            //
            // ui.label("Reflectance:");
            // egui::Slider::new(&mut material.reflectance, 0.0..=1.0).ui(ui);
            // ui.end_row();
            //
            ui.label("Unlit:");
            ui.checkbox(&mut variables.playing, "");
            ui.end_row();
        });
    });
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
