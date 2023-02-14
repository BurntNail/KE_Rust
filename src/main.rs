#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use eframe::{App, CreationContext, Frame, Storage};
use egui::Context;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::cmp::Ordering;

pub const MAX: i32 = 10;
pub const MIN: i32 = -10;

pub struct GuessingGameApp {
    current_number: i32,
    current_number_revealed: bool,
    last_guess: Option<i32>,
    current_guess: String,
    rand: ThreadRng,
}

impl GuessingGameApp {
    #[must_use]
    pub fn new(cc: &CreationContext) -> Self {
        let mut rand = thread_rng();

        let start = match cc
            .storage
            .and_then(|cc| cc.get_string("guess"))
            .map(|g| g.parse::<i32>())
        {
            Some(Ok(n)) => n,
            _ => rand.gen_range(MIN..=MAX),
        };

        Self {
            current_number: start,
            current_number_revealed: false,
            current_guess: String::default(),
            last_guess: None,
            rand,
        }
    }
    pub fn reset(&mut self) {
        self.last_guess = None;
        self.current_number = self.rand.gen_range(MIN..=MAX);
        self.current_number_revealed = false;
    }
    pub fn add_guess(&mut self, g: i32) {
        self.last_guess = Some(g);
        self.current_guess.clear();
    }
    #[must_use]
    pub fn ordering(&self) -> Option<Ordering> {
        self.last_guess.map(|lg| lg.cmp(&self.current_number))
    }
}

impl App for GuessingGameApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!(
                "Welcome to the Guessing Game!\nTry to guess the number between {MIN} and {MAX}"
            ));
            ui.separator();
            if let Some(ord) = self.ordering() {
                ui.label(if ord == Ordering::Equal {
                    format!("You got it right! It was {}", self.current_number)
                } else {
                    format!(
                        "Your last guess ({}) was {ord:?} than the actual number.",
                        self.last_guess.unwrap()
                    )
                });
            }
            ui.horizontal(|ui| {
                ui.label("Enter your guess here: ");
                ui.text_edit_singleline(&mut self.current_guess);
            });
            if ui.button("Submit Guess!").clicked() {
                match self.current_guess.parse() {
                    Ok(n) => self.add_guess(n),
                    Err(e) => {
                        eprintln!("Error parsing input: {e}");
                    }
                }
            }
            if ui.button("Reset!").clicked() {
                self.reset();
            }
        });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        if !self.current_number_revealed {
            storage.set_string("guess", self.current_number.to_string());
        } else if storage.get_string("guess").map_or(false, |s| !s.is_empty()) {
            storage.set_string("guess", String::default());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eframe::run_native(
        "Number Guessing Game",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(GuessingGameApp::new(cc))),
    )
    .expect("error running app");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "canvas",
            eframe::WebOptions::default(),
            Box::new(|cc| Box::new(GuessingGameApp::new(cc))),
        )
        .await
        .expect("error running app");
    });
}
