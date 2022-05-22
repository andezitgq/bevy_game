use bevy::prelude::*;
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

pub fn despawn_with<T: Component>(
	mut commands: Commands, 
	q: Query<Entity, With<T>>
){
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn main_menu(
	mut egui_ctx: ResMut<EguiContext>,
	mut commands: Commands,
	mut exit: EventWriter<AppExit>,
	keys: Res<Input<KeyCode>>,
){
	 egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
			ui.heading("Kampludejo");
			if ui.button("Ludi").clicked() {
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
    
    if keys.pressed(KeyCode::Space) {
		commands.insert_resource(NextState(GameState::InGame));
	}
}
