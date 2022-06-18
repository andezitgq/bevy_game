use std::fs;
use bevy::prelude::*;
use bevy::window::*;
use bevy::app::AppExit;
use bevy_rapier3d::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_discord_presence::{
    config::{RPCConfig, RPCPlugin},
    state::ActivityState,
};
use iyes_loopless::prelude::*;
use crate::lib::components::*;

pub fn despawn_with<T: Component>(
	mut commands: Commands, 
	q: Query<Entity, With<T>>
){
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn screen_size(
	mut resize_event: EventReader<WindowResized>,
	mut screen: ResMut<Screen>,
){
	for e in resize_event.iter() {
		screen.0 = e.width;
		screen.1 = e.height;
	}
}

pub fn setup_font(
	mut egui_ctx: ResMut<EguiContext>,
){
	let mut fonts = egui::FontDefinitions::default();
		
	fonts.font_data.insert("my_font".to_owned(),
	egui::FontData::from_static(include_bytes!("../../assets/fonts/ubuntu.ttf"))); 

	fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "my_font".to_owned());
	fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("my_font".to_owned());

	egui_ctx.ctx_mut().set_fonts(fonts);
}

pub fn main_menu(
	mut commands: Commands,
	mut egui_ctx: ResMut<EguiContext>,
	mut exit: EventWriter<AppExit>,
	screen: Res<Screen>,
	lvl_dialog: Option<Res<LevelDialog>>,
){
	if let Some(lvl_dialog) = lvl_dialog{
		if lvl_dialog.0 == true {
			egui::Window::new("Elektu nivelon")
			.anchor(egui::Align2::CENTER_CENTER, [-0.0, 0.0])
			.resizable(false)
			.collapsible(false)
			.show(egui_ctx.ctx_mut(), |ui| {
				egui::ScrollArea::vertical()
				.max_height(100.0)
				.show(ui, |ui| {
					ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui|{
						let paths = fs::read_dir("./assets/scenes").unwrap();
						for path in paths {
							let path = path.unwrap().path().display().to_string();
							if path.ends_with(".glb") {
								if ui.add(egui::Button::new(level_name(&path))).clicked() {	
									let path = path[9..].to_string();
									commands.insert_resource(CurrentLevel(path));
									commands.insert_resource(NextState(GameState::InGame));
								};
							}
						}
					});
				});
			});
		}
	}
	egui::SidePanel::left("side_panel").default_width(200.0).resizable(false).show(egui_ctx.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
			ui.allocate_space(egui::Vec2::new(0.0, screen.1 / 2.0 - 25.0));
			ui.heading("Kamplud'");
			if ui.add(egui::Button::new("Ludi").frame(false)).clicked() {
				commands.insert_resource(LevelDialog(true));
			}
			
			if ui.add(egui::Button::new("Eliri").frame(false)).clicked() {
				exit.send(AppExit);
			}
        });
        
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add(egui::Hyperlink::from_label_and_url(
                "farita de linuksulo",
                "https://github.com/andezitgq/",
            ));
        });
    });
}

pub fn pause_menu(
	mut commands: Commands,
	mut egui_ctx: ResMut<EguiContext>,
	mut rapier_config: ResMut<RapierConfiguration>,
	mut exit: EventWriter<AppExit>,
	screen: Res<Screen>,
){
	egui::SidePanel::left("side_panel").default_width(200.0).resizable(false).show(egui_ctx.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
			ui.allocate_space(egui::Vec2::new(0.0, screen.1 / 2.0 - 25.0));
			ui.heading("Kamplud'");
			if ui.add(egui::Button::new("Ludi").frame(false)).clicked() {
				commands.insert_resource(LevelDialog(true));
			}
			
			if ui.add(egui::Button::new("Eliri al menuo").frame(false)).clicked() {
				commands.insert_resource(NextState(GameState::MainMenu));
				commands.insert_resource(Pause(false));
				rapier_config.physics_pipeline_active = true;
				rapier_config.query_pipeline_active = true;
				commands.remove_resource::<CurrentLevel>();
				commands.remove_resource::<LoadedMeshes>();
				commands.remove_resource::<GltfMeshes>();
			}
			
			if ui.add(egui::Button::new("Eliri al OS").frame(false)).clicked() {
				exit.send(AppExit);
			}
        });
        
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add(egui::Hyperlink::from_label_and_url(
                "farita de linuksulo",
                "https://github.com/andezitgq/",
            ));
        });
    });
}

fn level_name(path: &String) -> String {
	let mut i = path.len();
	
	for ch in path.chars().rev() {
		if ch == '/' {
			break;
		}
		i -= 1;
	}
	
	let mut c = path[i..(path.len() - 4)].chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
