use chrono::{DateTime, Utc};
use eframe::egui;
use eframe::glow::Context;
use serde::{Deserialize, Serialize};
use std::fs;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Horeb",
        native_options,
        Box::new(|cc| Box::new(HorebApp::new(cc))),
    );
}

#[derive(Serialize, Deserialize)]
struct Note {
    title: String,
    content: String,
    last_modified: DateTime<Utc>,
}

impl Note {
    pub fn new() -> Self {
        Self {
            title: "New note".into(),
            content: "".into(),
            last_modified: Utc::now(),
        }
    }
}

#[derive(Default)]
struct HorebApp {
    selected_index: Option<usize>,
    notes: Vec<Note>,
}

impl HorebApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();

        let path = homedir::get_my_home()
            .unwrap()
            .unwrap()
            .join("horeb_notes.json");
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(obj) = serde_json::from_str::<Vec<Note>>(&content) {
                    app.notes = obj;
                }
            }
        }

        app
    }
}

impl eframe::App for HorebApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("Side Panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("Available notes:");
                    if ui.button("+").clicked() {
                        self.notes.push(Note::new());
                        self.selected_index = Some(self.notes.len() - 1);
                    }

                    if ui.button("-").clicked() {
                        if let Some(selected) = self.selected_index {
                            self.notes.remove(selected);
                            self.selected_index = None;
                        }
                    }
                });
            });
            ui.separator();

            let mut requested_date_update = None;

            for (idx, note) in self.notes.iter().enumerate() {
                if ui.button(&note.title).clicked() {
                    requested_date_update = self.selected_index;
                    self.selected_index = Some(idx);
                }
            }

            if let Some(requested) = requested_date_update {
                self.notes[requested].last_modified = Utc::now();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if let Some(selected) = self.selected_index {
                    let index = self.selected_index.unwrap();
                    let note = &mut self.notes[index];

                    ui.heading(note.last_modified.to_string());
                } else {
                    ui.heading("No note selected");
                }
            });

            ui.separator();

            if let Some(selected) = self.selected_index {
                let index = self.selected_index.unwrap();
                let note = &mut self.notes[index];

                ui.horizontal(|ui| {
                    ui.label("Title: ");
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut note.title),
                    );
                });

                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut note.content),
                );
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&Context>) {
        let path = homedir::get_my_home()
            .unwrap()
            .unwrap()
            .join("horeb_notes.json");
        let content = serde_json::to_string(&mut self.notes).expect("Failed to serialize notes");

        fs::write(path, content).expect("Failed to write serialized notes");
    }
}
