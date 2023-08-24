use eframe;
use egui;
use egui::{InputState, Key, ScrollArea, TextBuffer};
use std::collections::HashSet;

use crate::game::Game;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GUIApp {
	user_input: String,
	auto_scroll: bool,
	refocus_input: bool,
	
	#[serde(skip)]
	game: Game,
	#[serde(skip)]
	keys_down: HashSet<egui::Key>,
	#[serde(skip)]
	keys_pressed: HashSet<egui::Key>,
	#[serde(skip)]
	keys_released: HashSet<egui::Key>,
}

impl Default for GUIApp {
	fn default() -> Self {
		Self {
			user_input: "".to_owned(),
			game: Game::default(),
			auto_scroll: true,
			refocus_input: true, // Should we focus again on the input text box?
			
			keys_down: HashSet::default(),
			keys_pressed: HashSet::default(),
			keys_released: HashSet::default(),
		}
	}
}

impl GUIApp {
	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customize the look and feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
		
		// Load previous app state (if any).
		// Note that you must enable the `persistence` feature for this to work.
		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
		}
		
		Default::default()
	}
}

impl eframe::App for GUIApp {
	/// Called by the frame work to save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.input(|input_state|{
			let new_keys_down = input_state.keys_down.clone();
			self.keys_pressed = new_keys_down.difference(&self.keys_down).map(|k| { k.clone() }).collect();
			self.keys_released = self.keys_down.difference(&new_keys_down).map(|k| { k.clone() }).collect();
			self.keys_down = new_keys_down;
		});
		/*
		let Self {
			user_input: readline,
			row_sizes,
			history,
			auto_scroll,
			value
		} = self;
		*/
		
		// Examples of how to create different panels and windows.
		// Pick whichever suits you.
		// Tip: a good default choice is to just keep the `CentralPanel`.
		// For inspiration and more examples, go to https://emilk.github.io/egui
		
		#[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			// The top panel is often a good place for a menu bar:
			egui::menu::bar(ui, |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("Quit").clicked() {
						_frame.close();
					}
				});
			});
		});
		
		egui::SidePanel::left("side_panel").show(ctx, |ui| {
			ui.heading("Side Panel");
			
			//ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
			//if ui.button("Increment").clicked() {
			//	self.value += 1.0;
			//}
			
			ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
				ui.horizontal(|ui| {
					ui.spacing_mut().item_spacing.x = 0.0;
					ui.label("powered by ");
					ui.hyperlink_to("egui", "https://github.com/emilk/egui");
					ui.label(" and ");
					ui.hyperlink_to(
						"eframe",
						"https://github.com/emilk/egui/tree/master/crates/eframe",
					);
					ui.label(".");
				});
			});
		});
		
		egui::CentralPanel::default().show(ctx, |ui| {
			// The central panel the region left after adding TopPanel's and SidePanel's
			//ui.heading("eframe template");
			//ui.hyperlink("https://github.com/emilk/eframe_template");
			//ui.add(egui::github_link_file!( "https://github.com/emilk/eframe_template/blob/master/", "Source code." ));

			/*
			use egui_extras::{TableBuilder, Column};

			TableBuilder::new(ui)
				.column(Column::remainder().at_least(100.0))
				.body(|mut body| {
					let row_heights: Vec<f32> = vec![60.0, 18.0, 31.0, 240.0];
					body.heterogeneous_rows(row_heights.into_iter(), |row_index, mut row| {
						let thick = row_index % 6 == 0;
						row.col(|ui| {
							ui.centered_and_justified(|ui| {
								ui.label(row_index.to_string());
							});
						});
					});
				});
				
			*/
			let text_style = egui::TextStyle::Body;
			let outcome_history = &self.game.get_history();
			let user_input_history = &self.game.get_user_inputs();
			let row_height = ui.text_style_height(&text_style);
			let num_rows = outcome_history.len() + user_input_history.len();
			ScrollArea::vertical().auto_shrink([false; 2]).stick_to_bottom(self.auto_scroll).show_rows(
				ui,
				row_height,
				num_rows,
				|ui, row_range| {
					for row in row_range {
						//let text = format!("This is row {}/{}", row + 1, num_rows);
						if row % 2 == 0 {
							ui.label(&outcome_history[row/2]);
						} else {
							ui.label(&user_input_history[row/2]);
						}
					}
				},
			);
			ui.horizontal(|ui| {
				ui.label("> ");
				if self.refocus_input {
					ui.text_edit_singleline(&mut self.user_input).request_focus();
					self.refocus_input = false;
				} else {
					ui.text_edit_singleline(&mut self.user_input);
				}
				if ui.button("[SEND]").clicked() || self.keys_released.contains(&Key::Enter) {
					self.game.send_command(self.user_input.take());
					self.refocus_input = true;
				};
			});
			
			egui::warn_if_debug_build(ui);
		});
		
		if false {
			egui::Window::new("Window").show(ctx, |ui| {
				ui.label("Windows can be moved by dragging them.");
				ui.label("They are automatically sized based on contents.");
				ui.label("You can turn on resizing and scrolling if you like.");
				ui.label("You would normally choose either panels OR windows.");
			});
		}
	}
}