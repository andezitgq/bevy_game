use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::lib::menu::Screen;
use crate::{XP, Player, Health};

pub fn setup_ui_camera(mut commands: Commands) {
	commands.spawn_bundle(UiCameraBundle::default());
}

pub fn setup_ui(
	mut egui_ctx: ResMut<EguiContext>,
	player: Query<(&XP, &Health), With<Player>>,
	screen: Res<Screen>,
){  
	let (xp, health) = player.single();
    egui::TopBottomPanel::top("my_panel")
    .resizable(false)
    .show(egui_ctx.ctx_mut(), |ui| {
		ui.horizontal_centered(|ui| {
			//ui.allocate_space(egui::Vec2::new(0.0, screen.1 / 2.0 - 25.0));
			ui.heading(format!("Saneco: {}   Poentaro: {}",health.0.to_string() , xp.0.to_string()));
		});
	});
}
