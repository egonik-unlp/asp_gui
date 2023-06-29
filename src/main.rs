#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use asp_lib::{handle_many_spectra, handle_single_spectrum};
use eframe::{egui, NativeOptions};
use egui::{FontId, RichText};
fn main() -> Result<(), eframe::Error> {
    let mut resultado1 = "Esperando".to_owned();
    let mut resultado2 = "Esperando".to_owned();
    let mut exp_dir = "exportados".to_owned();
    let mut file = "".to_owned();
    let mut dir = "".to_owned();
    let mut exp_dir_dir = "".to_owned();
    let options = NativeOptions::default();
    eframe::run_simple_native("Agilent ASP Parser", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Agilent ASP Parser");
            ui.label(RichText::new("Programa para convertir archivos ASP generados por equipos de la linea Agilent Cary en archivos csv.\n Ingrese la ubicacion del archivo o directorio a convertir.\nLos archivos convertidos se almacenan en un subdirectorio dentro de la carpeta en la cual se ubican los archivos originales").font(egui::FontId {size: 14f32, family: egui::FontFamily::Proportional }) );
            ui.add_space(20f32);
            ui.label(
                RichText::new("Convertir archivos asp individuales").font(egui::FontId {
                    size: 18f32,
                    family: egui::FontFamily::Proportional,
                }),
            );

            ui.horizontal(|ui| {
                let name_label = ui.label("Archivo origen: ");
                ui.text_edit_singleline(&mut file)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("Prefijo para carpeta de destino: ");
                ui.text_edit_singleline(&mut exp_dir)
                    .labelled_by(name_label.id);
            });
            ui.add_space(20f32);

            if ui.button("Convertir archivo individual").clicked() {
                resultado1 = match handle_single_spectrum(&file, &exp_dir) {
                    Ok(x) => format!("El archivo {} se convirtió exitosamente", x),
                    Err(e) => format!("Hubo un error, descripción => {}", e),
                };
            }
            ui.add_space(16f32);
            ui.label(&resultado1);
            ui.add_space(20f32);
            ui.separator();
            ui.label(
                RichText::new("Convertir directorios de archivos asp").font(FontId {
                    size: 18f32,
                    family: egui::FontFamily::Proportional,
                }),
            );
            ui.add_space(20f32);

            ui.horizontal(|ui| {
                let name_label = ui.label("Carpeta origen: ");
                ui.text_edit_singleline(&mut dir).labelled_by(name_label.id);
            });

            ui.horizontal(|ui| {
                let name_label = ui.label("Prefijo para carpeta de destino: ");
                ui.text_edit_singleline(&mut exp_dir_dir)
                    .labelled_by(name_label.id);
            });
            ui.add_space(20f32);

            if ui.button("Convertir directorio").clicked() {
                resultado2 = match handle_many_spectra(&dir, &exp_dir_dir) {
                    Ok(x) => format!("El archivo {} se convirtió exitosamente", x),
                    Err(e) => format!("Hubo un error, descripción => {}", e),
                };
            }
            ui.add_space(16f32);

            ui.label(&resultado2);
        });
    })
}
