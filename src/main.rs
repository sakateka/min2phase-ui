#![allow(clippy::too_many_arguments)]

use eframe::egui;
use min2phase::*;

struct AppState {
    palette: [egui::Color32; 6],
    current_color: usize,
    facelets: [u8; 54],
    max_depth: u8,
    facelet_text: String,
    moves_text: String,
    prev_facelets: [u8; 54],
    log: String,
}

impl Default for AppState {
    fn default() -> Self {
        let mut s = Self {
            palette: [
                egui::Color32::WHITE,
                egui::Color32::from_rgb(220, 0, 0),
                egui::Color32::from_rgb(0, 160, 0),
                egui::Color32::from_rgb(230, 200, 0),
                egui::Color32::from_rgb(255, 120, 0),
                egui::Color32::from_rgb(0, 90, 200),
            ],
            current_color: 0,
            facelets: [0; 54],
            max_depth: 21,
            facelet_text: String::new(),
            moves_text: String::new(),
            prev_facelets: [0; 54],
            log: String::new(),
        };
        for i in 0..54 {
            s.facelets[i] = (i / 9) as u8;
        }
        s.prev_facelets = s.facelets;
        s.facelet_text = s.write_facelet_string();
        s
    }
}

impl AppState {
    fn write_facelet_string(&self) -> String {
        let mut buf = String::with_capacity(54);
        for i in 0..54 {
            buf.push(match self.facelets[i] {
                0 => 'U',
                1 => 'R',
                2 => 'F',
                3 => 'D',
                4 => 'L',
                _ => 'B',
            });
        }
        buf
    }

    fn apply_facelet_string(&mut self, s: &str) {
        if s.len() < 54 {
            return;
        }
        for (i, ch) in s.chars().take(54).enumerate() {
            self.facelets[i] = match ch {
                'U' => 0,
                'R' => 1,
                'F' => 2,
                'D' => 3,
                'L' => 4,
                _ => 5,
            };
        }
    }

    fn draw_palette(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for (i, col) in self.palette.iter().enumerate() {
                let mut button = egui::Button::new(" ")
                    .fill(*col)
                    .min_size(egui::vec2(24.0, 24.0));
                if i == self.current_color {
                    button = button.stroke(egui::Stroke {
                        width: 2.0,
                        color: egui::Color32::BLACK,
                    });
                }
                if ui.add(button).clicked() {
                    self.current_color = i;
                }
            }
        });
    }

    fn draw_face(&mut self, ui: &mut egui::Ui, face_index: usize) {
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(4.0, 4.0);
            let base = face_index * 9;
            for r in 0..3 {
                ui.horizontal(|ui| {
                    for c in 0..3 {
                        let idx = base + r * 3 + c;
                        let color = self.palette[self.facelets[idx] as usize];
                        let label = if r == 1 && c == 1 {
                            match face_index {
                                0 => "U",
                                1 => "R",
                                2 => "F",
                                3 => "D",
                                4 => "L",
                                _ => "B",
                            }
                        } else {
                            ""
                        };
                        if ui
                            .add(
                                egui::Button::new(label)
                                    .fill(color)
                                    .min_size(egui::vec2(28.0, 28.0)),
                            )
                            .clicked()
                        {
                            self.facelets[idx] = self.current_color as u8;
                        }
                    }
                });
            }
        });
    }

    fn draw_empty_face(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(4.0, 4.0);
            for _ in 0..3 {
                ui.horizontal(|ui| {
                    for _ in 0..3 {
                        ui.allocate_space(egui::vec2(28.0, 28.0));
                    }
                });
            }
        });
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Facelet (54 chars):");
            ui.add(egui::TextEdit::singleline(&mut self.facelet_text).desired_width(f32::INFINITY));
            ui.label("Rotations / Moves:");
            ui.add(egui::TextEdit::singleline(&mut self.moves_text).desired_width(f32::INFINITY));
            ui.horizontal(|ui| {
                if ui.button("Solve Cube").clicked() {
                    let face = if self.facelet_text.trim().len() >= 54 {
                        self.facelet_text.trim().to_string()
                    } else {
                        self.write_facelet_string()
                    };
                    self.log = format!("Cube: {}\n{}", face, self.log);
                    let sol = solve(&face, self.max_depth);
                    let moves = sol.split_whitespace().count();
                    self.moves_text = sol.to_string();
                    self.log = format!("{} ({}f)\n{}", sol, moves, self.log);
                }
                if ui.button("Random Cube").clicked() {
                    self.prev_facelets = self.facelets;
                    let s = random_cube();
                    self.apply_facelet_string(&s);
                    self.facelet_text = s.clone();
                    self.moves_text.clear();
                    self.log = format!("{}\n{}", s, self.log);
                }
                if ui.button("Scramble").clicked() {
                    self.prev_facelets = self.facelets;
                    let scr = random_moves(25);
                    let cur = self.write_facelet_string();
                    if let Some(new_face) = apply_moves(&cur, &scr) {
                        self.apply_facelet_string(&new_face);
                        self.facelet_text = new_face.clone();
                        self.moves_text = scr.clone();
                        self.log = format!("Scramble: {}\n{}", scr, self.log);
                    }
                }
                if ui.button("Apply Moves").clicked() {
                    self.prev_facelets = self.facelets;
                    let base = if self.facelet_text.trim().len() >= 54 {
                        self.facelet_text.trim().to_string()
                    } else {
                        self.write_facelet_string()
                    };
                    let mv = self.moves_text.trim().to_string();
                    if !mv.is_empty()
                        && let Some(new_face) = apply_moves(&base, &mv)
                    {
                        self.apply_facelet_string(&new_face);
                        self.facelet_text = new_face.clone();
                        self.log = format!("Apply: {}\n{}", mv, self.log);
                    }
                }
                if ui.button("Reset").clicked() {
                    self.facelets = self.prev_facelets;
                    self.facelet_text = self.write_facelet_string();
                    self.moves_text.clear();
                }
            });

            ui.separator();
            ui.heading("min2phase UI (egui)");
            ui.separator();
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Select color then click tiles to paint.");
                    self.draw_palette(ui);
                });
                ui.add_space(16.0);
                ui.vertical(|ui| {
                    ui.label("Move Limit");
                    ui.add(egui::DragValue::new(&mut self.max_depth).range(1..=24));
                });
            });
            ui.add_space(16.0);

            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.vertical(|ui| {
                    self.draw_empty_face(ui);
                });
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    self.draw_face(ui, 0);
                });
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    self.draw_empty_face(ui);
                });
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    self.draw_empty_face(ui);
                });
            });
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.vertical(|ui| {
                    self.draw_face(ui, 4);
                });
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    self.draw_face(ui, 2);
                });
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    self.draw_face(ui, 1);
                });
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    self.draw_face(ui, 5);
                });
            });
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.add_space(16.0);
                ui.vertical(|ui| {
                    self.draw_empty_face(ui);
                });
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    self.draw_face(ui, 3);
                });
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    self.draw_empty_face(ui);
                });
                ui.add_space(4.0);
                ui.vertical(|ui| {
                    self.draw_empty_face(ui);
                });
            });

            ui.add_space(12.0);
            ui.separator();
            ui.label("Output");
            egui::ScrollArea::vertical()
                .max_height(160.0)
                .show(ui, |ui| {
                    ui.code(&self.log);
                });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.inner_size = Some(egui::vec2(900.0, 900.0));
    eframe::run_native(
        "min2phase UI",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_pixels_per_point(1.2);
            Ok::<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>>(Box::new(
                AppState::default(),
            ))
        }),
    )
}
