use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct State {
	app: App,
}

impl State {
	pub fn new() -> Self {
		let mut app = App::new();

		app.add_plugins(DefaultPlugins);
		app.add_plugins(EguiPlugin);

		app.add_systems(Startup, Self::setup);
		app.add_systems(Update, Self::update_egui);

		Self {
			app,
		}
	}

	pub fn run(&mut self) {
		self.app.run();
	}

	fn setup() {

	}

	fn update_egui(mut ctxs: EguiContexts, mut closer: EventWriter<AppExit>) {
		let ctx = ctxs.ctx_mut();

		egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				egui::menu::menu_button(ui, "File", |ui| {
					if ui.button("Quit").clicked() {
						// request exit
						closer.send(AppExit);
					}
				});
			});
		});

		egui::SidePanel::left("Settings")
			.min_width(200.0)
			.show(ctx, |ui| {
				ui.label("BLAH");
			});

		egui::CentralPanel::default().show(ctx, |ui| {
			let prev_height = ui.available_height();
			ui.label("CENTER");
			let total_height = ui.available_height();
			ui.label(format!("prev {}, after {}", prev_height, total_height));
		});
	}
}