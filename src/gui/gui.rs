#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{
    collections::{BTreeMap, HashMap},
    vec,
};

use eframe::egui;

pub fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("TTG"),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<App>::default())
        }),
    )
}

struct App {
    name: String,
    age: u32,
    text_input_vec: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: "Awais".to_owned(),
            age: 21,
            text_input_vec: vec!["p".to_string(), "^".to_string(), "q".to_string()],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(&ctx, catppuccin_egui::MOCHA);

        //? Operation Name and Operation Symbol
        // let OperationMap = BTreeMap::from([
        //     ("AND", "^"),
        //     ("OR", "v"),
        //     ("NOT", "¬"),
        //     ("NAND", "⊼"),
        //     ("NOR", "⊽"),
        //     ("XOR", "⊗"),
        //     ("XNOR", ""),
        //     ("BICONDITIONAL", "↔"),
        // ]);

        //? Order of Values in vec
        //? 1. Name Uppercase
        //? 2. Name Lowercase
        //? 3. Programming Symbol
        //? 4. Mathematical Symbol
        let operation_map: Vec<Vec<&str>> = vec![
            vec!["P", "", "P", ""],
            vec!["Q", "", "Q", ""],
            vec!["AND", "and", "&", "^"],
            vec!["OR", "or", "v", "||"],
            vec!["NOT", "not", "¬", "!"],
            vec!["XOR", "xor", "^", "⊗"],
            vec!["XNOR", "xnor", "~", "⊙"],
            vec!["IMPLICATION", "implication", "=>", "->"],
            vec!["BICONDITIONAL", "biconditional", "<->", "<=>"],
        ];

        //? The values shown in the text input
        let mut input_display = self.text_input_vec.join(" ");
        // let combined_text = self.text_input_vec.join(" ");

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Truth Table Generator");

            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.label(&input_display).labelled_by(name_label.id);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button("Increment").clicked() {
                self.age += 1;
            }

            if ui.button("Clear").clicked() {
                self.text_input_vec.clear();
                self.text_input_vec.push(String::new());
            }

            ui.horizontal(|ui| {
                for i in operation_map {
                    let name = i[0];
                    let symbol = i[2];
                    ui.vertical(|ui| {
                        if ui.button(symbol).clicked() {
                            self.text_input_vec.push(symbol.to_string());
                        }
                        ui.label(name);
                    });
                }
            });

            let answer_display = if input_display.len() == 0 {
                ""
            } else if &input_display == "p ^ q ^" {
                "T T T T"
            } else {
                "Unknown statement"
            };
            ui.label(answer_display);
        });
    }
}
