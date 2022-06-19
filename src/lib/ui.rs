use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::NextState;

use crate::lib::components::*;
use crate::lib::presence::*;

pub fn setup_ui_camera(mut commands: Commands) {
	commands.spawn_bundle(UiCameraBundle::default()).insert(InGame);
}

pub fn setup_ui(
	mut egui_ctx: ResMut<EguiContext>,
	player: Query<(&XP, &Health, &Velocity), With<Player>>,
){  
	let (xp, health, velocity) = player.single();
    egui::TopBottomPanel::top("my_panel")
    .resizable(false)
    .show(egui_ctx.ctx_mut(), |ui| {
		ui.horizontal_centered(|ui| {
			ui.heading(format!("Saneco: {}    Poentaro: {}    Velocity: {}", health.0.to_string() , xp.0.to_string(), velocity.linvel));
		});
	});
}

pub fn win(
	mut commands: Commands,
	mut egui_ctx: ResMut<EguiContext>,
	mut rapier_config: ResMut<RapierConfiguration>,
	xp_query: Query<&XP, With<Player>>,
){
	let xp = xp_query.single();
	egui::Window::new("Vi ekvenkis!")
	.anchor(egui::Align2::CENTER_CENTER, [-0.0, 0.0])
	.resizable(false)
	.collapsible(false)
	.show(egui_ctx.ctx_mut(), |ui| {
		egui::ScrollArea::vertical()
		.max_height(100.0)
		.show(ui, |ui| {
			ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui|{
				ui.allocate_space(egui::Vec2::new(0.0, 10.0));
				ui.label(format!("Poentaro: {}", xp.0));
				ui.allocate_space(egui::Vec2::new(0.0, 10.0));
				if ui.add(egui::Button::new("En menuon").frame(true)).clicked() {
					commands.insert_resource(NextState(GameState::MainMenu));
					score_add(xp.0 as usize);
					rapier_config.physics_pipeline_active = true;
					rapier_config.query_pipeline_active = true;
					commands.remove_resource::<CurrentLevel>();
					commands.remove_resource::<LoadedMeshes>();
					commands.remove_resource::<GltfMeshes>();
				}
			});
		});
	});
}

pub fn game_over(
	mut commands: Commands,
	mut egui_ctx: ResMut<EguiContext>,
	mut rapier_config: ResMut<RapierConfiguration>,
	mut xp_query: Query<&mut XP, With<Player>>,
){
	let mut xp = xp_query.single_mut();
	egui::Window::new("Malvenko")
	.anchor(egui::Align2::CENTER_CENTER, [-0.0, 0.0])
	.resizable(false)
	.collapsible(false)
	.show(egui_ctx.ctx_mut(), |ui| {
		egui::ScrollArea::vertical()
		.max_height(100.0)
		.show(ui, |ui| {
			ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui|{
				ui.allocate_space(egui::Vec2::new(0.0, 50.0));
				if ui.add(egui::Button::new("En menuon").frame(true)).clicked() {
					xp.0 = 0;
					commands.insert_resource(NextState(GameState::MainMenu));
					rapier_config.physics_pipeline_active = true;
					rapier_config.query_pipeline_active = true;
					commands.remove_resource::<CurrentLevel>();
					commands.remove_resource::<LoadedMeshes>();
					commands.remove_resource::<GltfMeshes>();
				}
			});
		});
	});
}
