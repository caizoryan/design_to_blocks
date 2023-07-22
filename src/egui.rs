use bevy::prelude::*;
use bevy_egui::{
    egui::{self, InnerResponse, Widget},
    EguiContexts,
};

use crate::{Bounds, ChunkStates, ColorChannels, SelectedIndex};

pub fn update_egui(
    mut contexts: EguiContexts,
    mut variables: ResMut<ChunkStates>,
    selected: Res<SelectedIndex>,
) {
    let index = match selected.0 {
        Some(index) => index,
        _ => return,
    };

    let ctx = contexts.ctx_mut();
    egui::Window::new("Cube material preview").show(ctx, |ui| {
        let res = egui::Grid::new("preview").show(ui, |ui| {
            ui.label("Base color:");
            color_picker_widget(ui, &mut variables.0[index].base_color);
            ui.end_row();

            ui.label("Scale");
            egui::Slider::new(&mut variables.0[index].scale, 0.3..=20.0).ui(ui);
            ui.end_row();

            ui.label("Perceptual roughness:");
            egui::Slider::new(&mut variables.0[index].perceptual_roughness, 0.01..=1.0).ui(ui);
            ui.end_row();

            ui.selectable_value(&mut variables.0[index].inter_color, ColorChannels::R, "R");
            ui.selectable_value(&mut variables.0[index].inter_color, ColorChannels::G, "G");
            ui.selectable_value(&mut variables.0[index].inter_color, ColorChannels::B, "B");
            ui.end_row();

            ui.selectable_value(&mut variables.0[index].playing, true, "Playing");
            ui.selectable_value(&mut variables.0[index].playing, false, "Paused");

            ui.end_row();
            ui.columns(2, |ui| {
                ui[0].label("Min:");
                ui[0].add(location_edit_widget(&mut variables.0[index].bounds.0));
                ui[0].end_row();

                ui[1].label("Max:");
                ui[1].add(location_edit_widget(&mut variables.0[index].bounds.1));
                ui[1].end_row();
            });
        });
        res
    });
}

fn location_edit_widget(location: &mut Vec3) -> impl Widget + '_ {
    move |ui: &mut egui::Ui| {
        ui.vertical(|ui| {
            ui.label("X:");
            ui.add(egui::DragValue::new(&mut location.x));
            ui.end_row();

            ui.label("Y:");
            ui.add(egui::DragValue::new(&mut location.y));
            ui.end_row();

            ui.label("Z:");
            ui.add(egui::DragValue::new(&mut location.z));
            ui.end_row();
        })
        .response
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
