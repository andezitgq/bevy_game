use std::fs;
use bevy::prelude::*;
use bevy::window::*;
use bevy::app::AppExit;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
}

#[derive(Component)]
pub struct MainMenu;
#[derive(Component)]
pub struct InGame;

#[derive(Default)]
pub struct Screen(pub f32, pub f32);

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

pub fn main_menu(
	mut egui_ctx: ResMut<EguiContext>,
	mut commands: Commands,
	mut exit: EventWriter<AppExit>,
	screen: Res<Screen>,
){
	let mut fonts = egui::FontDefinitions::default();
		
	fonts.font_data.insert("my_font".to_owned(),
	egui::FontData::from_static(include_bytes!("../../assets/fonts/ubuntu.ttf"))); 

	fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "my_font".to_owned());
	fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().push("my_font".to_owned());

	egui_ctx.ctx_mut().set_fonts(fonts);
	
	
	egui::Window::new("Elektu nivelon")
	.anchor(egui::Align2::CENTER_CENTER, [-0.0, 0.0])
	.resizable(false)
	.collapsible(false)
	.show(egui_ctx.ctx_mut(), |ui| {
		egui::ScrollArea::vertical()
		.max_height(100.0)
		.show(ui, |ui| {
			ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui|{
				ui.add(egui::Button::new("Nivelo 1"));
				ui.add(egui::Button::new("Nivelo 2"));
				ui.add(egui::Button::new("Nivelo 3"));
				ui.add(egui::Button::new("Nivelo 4"));
				ui.add(egui::Button::new("Nivelo 5"));
				ui.add(egui::Button::new("Nivelo 6"));
				ui.add(egui::Button::new("Nivelo 7"));
				ui.add(egui::Button::new("Nivelo 8"));
			});
		});
	});
	
	egui::SidePanel::left("side_panel").default_width(200.0).resizable(false).show(egui_ctx.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
			ui.allocate_space(egui::Vec2::new(0.0, screen.1 / 2.0 - 25.0));
			ui.heading("Kampludejo");
			if ui.add(egui::Button::new("Ludi").frame(false)).clicked() {
				commands.insert_resource(NextState(GameState::InGame));
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
