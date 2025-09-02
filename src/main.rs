#![allow(clippy::too_many_arguments)]

use eframe::egui;
use min2phase::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Face {
    U,
    R,
    F,
    D,
    L,
    B,
}

impl Face {
    fn from_index(i: u8) -> Self {
        match i {
            0 => Face::U,
            1 => Face::R,
            2 => Face::F,
            3 => Face::D,
            4 => Face::L,
            _ => Face::B,
        }
    }

    fn to_index(self) -> u8 {
        match self {
            Face::U => 0,
            Face::R => 1,
            Face::F => 2,
            Face::D => 3,
            Face::L => 4,
            Face::B => 5,
        }
    }

    fn to_char(self) -> char {
        match self {
            Face::U => 'U',
            Face::R => 'R',
            Face::F => 'F',
            Face::D => 'D',
            Face::L => 'L',
            Face::B => 'B',
        }
    }

    fn from_char(ch: char) -> Option<Self> {
        match ch {
            'U' => Some(Face::U),
            'R' => Some(Face::R),
            'F' => Some(Face::F),
            'D' => Some(Face::D),
            'L' => Some(Face::L),
            'B' => Some(Face::B),
            _ => None,
        }
    }
}

struct Facelet(pub [Face; 54]);

impl Facelet {
    fn new_solved() -> Self {
        let mut arr = [Face::U; 54];
        for i in 0..54 {
            arr[i] = Face::from_index((i / 9) as u8);
        }
        Facelet(arr)
    }

    fn to_facelet_string(&self) -> String {
        let mut buf = String::with_capacity(54);
        for i in 0..54 {
            buf.push(self.0[i].to_char());
        }
        buf
    }

    fn apply_from_str(&mut self, s: &str) {
        if s.len() < 54 {
            return;
        }
        for (i, ch) in s.chars().take(54).enumerate() {
            if let Some(face) = Face::from_char(ch) {
                self.0[i] = face;
            }
        }
    }
}

struct AppState {
    palette: [egui::Color32; 6],
    current_color: usize,
    facelets: Facelet,
    max_depth: u8,
    facelet_text: String,
    moves_text: String,
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
            facelets: Facelet::new_solved(),
            max_depth: 21,
            facelet_text: String::new(),
            moves_text: String::new(),

            log: String::new(),
        };
        s.facelet_text = s.facelets.to_facelet_string();
        s
    }
}

impl AppState {
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
                        let color = self.palette[self.facelets.0[idx].to_index() as usize];
                        let label_str = if r == 1 && c == 1 {
                            Face::from_index(face_index as u8).to_char().to_string()
                        } else {
                            String::new()
                        };
                        let label = egui::RichText::new(label_str)
                            .strong()
                            .size(18.0)
                            .color(egui::Color32::BLACK);
                        if ui
                            .add(
                                egui::Button::new(label)
                                    .fill(color)
                                    .min_size(egui::vec2(28.0, 28.0)),
                            )
                            .clicked()
                        {
                            self.facelets.0[idx] = Face::from_index(self.current_color as u8);
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
            ui.separator();
            ui.label("Facelet (54 chars):");
            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
                ui.add(
                    egui::TextEdit::singleline(&mut self.facelet_text).desired_width(f32::INFINITY),
                );
            });
            ui.label("Rotations / Moves:");
            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
                ui.add(
                    egui::TextEdit::singleline(&mut self.moves_text).desired_width(f32::INFINITY),
                );
            });
            ui.separator();
            ui.horizontal(|ui| {
                let solve_label = egui::RichText::new("Solve Cube")
                    .strong()
                    .color(egui::Color32::BLACK)
                    .size(18.0);
                if ui
                    .add(
                        egui::Button::new(solve_label)
                            .fill(egui::Color32::from_rgb(0x4C, 0xAF, 0x50))
                            .min_size(egui::vec2(120.0, 34.0)),
                    )
                    .clicked()
                {
                    let face = if self.facelet_text.trim().len() >= 54 {
                        self.facelet_text.trim().to_string()
                    } else {
                        self.facelets.to_facelet_string()
                    };
                    self.log = format!("Cube: {}\n{}", face, self.log);
                    let sol = solve(&face, self.max_depth);
                    let moves = sol.split_whitespace().count();
                    self.moves_text = sol.to_string();
                    self.log = format!("{} ({}f)\n{}", sol, moves, self.log);
                }
                let random_label = egui::RichText::new("Random Cube")
                    .strong()
                    .color(egui::Color32::BLACK)
                    .size(18.0);
                if ui
                    .add(
                        egui::Button::new(random_label)
                            .fill(egui::Color32::from_rgb(0xFF, 0x98, 0x00))
                            .min_size(egui::vec2(120.0, 34.0)),
                    )
                    .clicked()
                {
                    let s = random_cube();
                    self.facelets.apply_from_str(&s);
                    self.facelet_text = s.clone();
                    self.moves_text.clear();
                    self.log = format!("{}\n{}", s, self.log);
                }
                let scramble_label = egui::RichText::new("Scramble")
                    .strong()
                    .color(egui::Color32::BLACK)
                    .size(18.0);
                if ui
                    .add(
                        egui::Button::new(scramble_label)
                            .fill(egui::Color32::from_rgb(0x21, 0x96, 0xF3))
                            .min_size(egui::vec2(120.0, 34.0)),
                    )
                    .clicked()
                {
                    let scr = random_moves(25);
                    let cur = self.facelets.to_facelet_string();
                    if let Some(new_face) = apply_moves(&cur, &scr) {
                        self.facelets.apply_from_str(&new_face);
                        self.facelet_text = new_face.clone();
                        self.moves_text = scr.clone();
                        self.log = format!("Scramble: {}\n{}", scr, self.log);
                    }
                }
                let apply_label = egui::RichText::new("Apply Moves")
                    .strong()
                    .color(egui::Color32::BLACK)
                    .size(18.0);
                if ui
                    .add(
                        egui::Button::new(apply_label)
                            .fill(egui::Color32::from_rgb(0xFF, 0xFF, 0xFF))
                            .min_size(egui::vec2(120.0, 34.0)),
                    )
                    .clicked()
                {
                    let base = if self.facelet_text.trim().len() >= 54 {
                        self.facelet_text.trim().to_string()
                    } else {
                        self.facelets.to_facelet_string()
                    };
                    let mv = self.moves_text.trim().to_string();
                    if !mv.is_empty()
                        && let Some(new_face) = apply_moves(&base, &mv)
                    {
                        self.facelets.apply_from_str(&new_face);
                        self.facelet_text = new_face.clone();
                        self.log = format!("Apply: {}\n{}", mv, self.log);
                    }
                }
                let reset_label = egui::RichText::new("Reset")
                    .strong()
                    .color(egui::Color32::BLACK)
                    .size(18.0);
                if ui
                    .add(
                        egui::Button::new(reset_label)
                            .fill(egui::Color32::from_rgb(0xF4, 0x43, 0x36))
                            .min_size(egui::vec2(120.0, 34.0)),
                    )
                    .clicked()
                {
                    self.facelets = Facelet::new_solved();
                    self.facelet_text = self.facelets.to_facelet_string();
                    self.log = format!("Reset\n{}", self.log);
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
                    ui.label(egui::RichText::new(&self.log).monospace());
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
