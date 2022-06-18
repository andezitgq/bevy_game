pub mod lib;

use std::f32::consts::PI as pi;
use std::ops::Mul;

use bevy::prelude::*;
use bevy::window::*;
use bevy::gltf::{Gltf, GltfNode, GltfMesh, GltfExtras};
use bevy::render::render_resource::{SamplerDescriptor, FilterMode};
use bevy_obj::*;
use bevy_rapier3d::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_discord_presence::{
    config::{RPCConfig, RPCPlugin},
};
use iyes_loopless::prelude::*;
use serde_json::Value;

use lib::orbit_camera::*;
use lib::ui::*;
use lib::menu::*; 
use lib::token::*;
use lib::components::*;
use lib::presence::*;

fn main() {
	App::new()
		.insert_resource(WindowDescriptor{
			title: "Kamplud'".to_string(),
			resizable: true,
			decorations: false,
			mode: WindowMode::BorderlessFullscreen,
			cursor_locked: false,
			cursor_visible: true,
			present_mode: PresentMode::Mailbox,
			..default()
		})
		.insert_resource(Screen(0.0, 0.0))
		.insert_resource(Pause(false))
		.insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        
		.add_plugins(DefaultPlugins)
		.add_plugin(RPCPlugin(RPCConfig{
			app_id: token(),
			show_time: true,
		}))
		.add_plugin(ObjPlugin)
		.add_plugin(EguiPlugin)
		.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        
        .add_loopless_state(GameState::MainMenu) 

		.add_enter_system(GameState::MainMenu, menu_bg)
		.add_enter_system(GameState::MainMenu, ds_menu)
		.add_enter_system(GameState::MainMenu, setup_font)
        .add_exit_system(GameState::MainMenu, despawn_with::<MainMenu>)
        
        .add_enter_system(GameState::InGame, setup)
        .add_enter_system(GameState::InGame, ds_level)
        .add_enter_system(GameState::InGame, spawn_camera)
        .add_exit_system(GameState::InGame, despawn_with::<InGame>)
        
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(main_menu)
                .into()
        )
        
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(scene_processing)
                .with_system(control_extras)
				.with_system(pause)
                .into()
        )
        
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_if_resource_equals::<Pause>(Pause(false))
                .with_system(control_character)
				.with_system(pan_orbit_camera)
				.with_system(get_coin)
				.with_system(setup_ui)
                .into()
        )
        
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_if_resource_equals::<Pause>(Pause(true))
                .with_system(pause_menu)
                .into()
        )
        
        .add_system(texture_filtering)
        .add_system(setup_ui_camera)
        .add_system(screen_size)

		.run();
}

fn pause(
	keys: Res<Input<KeyCode>>,
	mut windows: ResMut<Windows>,
	mut is_pause: ResMut<Pause>,
	mut rapier_config: ResMut<RapierConfiguration>,
){
	let window = windows.primary_mut();
	if keys.just_pressed(KeyCode::Escape) {
		is_pause.0 = !is_pause.0;
		if is_pause.0 {
			rapier_config.physics_pipeline_active = false;
			rapier_config.query_pipeline_active = false;
			window.set_cursor_lock_mode(false);
			window.set_cursor_visibility(true);
		} else {
			rapier_config.physics_pipeline_active = true;
			rapier_config.query_pipeline_active = true;
			window.set_cursor_lock_mode(true);
			window.set_cursor_visibility(false);
		}
	}
}

fn menu_bg(
	mut commands: Commands,
	assets: Res<AssetServer>,
){
	let scene = assets.load("scenes/menu/menu.glb#Scene0");
	commands.spawn()
	.insert(MainMenu)
	.with_children(|parent| {
        parent.spawn_scene(scene);
    });
	
	commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    })
    .insert(MainMenu);
    
    commands.spawn_bundle(PerspectiveCameraBundle {
		transform: Transform::from_xyz(-24.0, 12.0, -24.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	})
	.insert(MainMenu);
}

fn setup(
	mut windows: ResMut<Windows>,
	mut commands: Commands,	
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
    level: Option<Res<CurrentLevel>>,
){
	let window = windows.primary_mut();
	window.set_cursor_lock_mode(true);
	window.set_cursor_visibility(false);
    
    if let Some(level) = level {
		let gltf: Handle<Gltf> = assets.load(&level.0);
		commands.insert_resource(GltfMeshes(gltf));
	}
    
    //Lumo
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    }).insert(InGame);
	
	//Ludanto
	commands.spawn_bundle(PlayerBundle {
		xp: XP(0),
		health: Health(3),
		_p: Player,
		
		physics: PhysicsBundle {
			rigidbody: RigidBody::Dynamic,
			collider: Collider::ball(1.0f32),
			sensor: Sensor(false),
			
			restitution: Restitution::coefficient(0.7),
			dominance: Dominance::group(2),
			act: ActiveCollisionTypes::all(),
			events: ActiveEvents::COLLISION_EVENTS,
			ccd: Ccd::enabled(),
			..default()
		},
		
		pbr: PbrBundle {
			mesh: assets.load("models/player.obj"),
			material: materials.add(diffuse_mat("textures/player.jpg", &assets)),
			transform: Transform::from_xyz(-5.0, 0.0, 0.0),
			..default()
		},
	})
	.insert(InGame)
	.with_children(|parent| {
        parent.spawn()
        .insert(Sensor(true))
        .insert(Collider::ball(1.1f32))
		.insert(Restitution::default())
		.insert(Friction::default())
		.insert(ActiveCollisionTypes::all())
		.insert(ActiveEvents::COLLISION_EVENTS)
		.insert(Ccd::enabled())
        .insert(PlayerChild);
    });
}

fn scene_processing(
	mut commands: Commands,
	mut er_gltf: EventReader<AssetEvent<Gltf>>,
	cmeshes: Res<GltfMeshes>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    assets_gltfnode: Res<Assets<GltfNode>>,
){	
	for ev in er_gltf.iter() {
		if let AssetEvent::Created { handle } = ev {
			let scene = assets_gltf.get(handle).unwrap();
			let mut meshes: Vec<Handle<Mesh>> = Vec::new();
			
			if *handle == cmeshes.0 {
				commands.spawn_scene(scene.scenes[0].clone());
				for gltfnode in scene.nodes.iter() {
					let gltfnode = assets_gltfnode.get(gltfnode);
					if let Some(gltfnode) = gltfnode {
						let mut x: Vec<Handle<Mesh>> = mesh_event(&gltfnode, &assets_gltf, &assets_gltfmesh);
						meshes.append(&mut x);
					}
					
				}					
			}
			
			commands.insert_resource(LoadedMeshes(meshes));
		}
	}
}

fn mesh_event(
	gltfnode: 			&GltfNode,
	assets_gltf: 		&Res<Assets<Gltf>>,
    assets_gltfmesh: 	&Res<Assets<GltfMesh>>,
) -> Vec<Handle<Mesh>> {
	let mut ms: Vec<Handle<Mesh>> = Vec::new();
	
	if let Some(gltfmesh) = &gltfnode.mesh {
		let gltfmesh = assets_gltfmesh.get(gltfmesh);
		if let Some(gltfmesh) = gltfmesh {
			for primitive in gltfmesh.primitives.iter() {
				let mesh = primitive.mesh.clone();
				ms.push(mesh);
			}
		}
	}
	
	for children_node in gltfnode.children.iter() {
		ms.append(&mut mesh_event(children_node, assets_gltf, assets_gltfmesh))
	}
	
	return ms;
}

fn control_extras(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assets_mesh: ResMut<Assets<Mesh>>,
    q_parent: Query<(Entity, &Transform, &GltfExtras), Added<GltfExtras>>,
    q_child: Query<(&Parent, Entity, &Handle<Mesh>), Added<Handle<Mesh>>>,
    loaded_meshes: Option<Res<LoadedMeshes>>,
){	
	if let Some(loaded_meshes) = loaded_meshes {
		for (parent, ent, mesh) in q_child.iter() {
			for loaded_mesh in loaded_meshes.0.iter() {
				if loaded_mesh == mesh {
					if let Some(mesh) = assets_mesh.get(mesh) {
						if let Some(collider) = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh) {
							for (exent, _t, gltf_extras) in q_parent.iter() {
								if exent == parent.0 {
									let v: Value = serde_json::from_str(&gltf_extras.value).expect("Couldn't parse GltfExtra value as JSON");
									if v["collider"].as_str() == Some("true") {
										commands.entity(parent.0)
										.insert(Sensor(false))
										.insert(collider.clone())
										.insert(Ccd::enabled())
										.insert(ActiveCollisionTypes::default())
										.insert(ActiveEvents::COLLISION_EVENTS)
										.insert(InGame);
									}
									
									if v["type"].as_str() == Some("finish") {
										commands.entity(parent.0)
										.insert(Sensor(true))
										.insert(FinishTrigger);
										
										commands.entity(ent)
										.remove::<Handle<Mesh>>();
									}
									
									if v["type"].as_str() == Some("ground") {
										commands.entity(parent.0)
										.insert(Ground);
									}
									
									if v["type"].as_str() == Some("damage") {
										commands.entity(parent.0)
										.insert(Sensor(true))
										.insert(DamageTrigger);
										
										commands.entity(ent)
										.remove::<Handle<Mesh>>();
									}
								}
							}
						}
					}
				}
			}
		}
	}
	
	for (_exent, t, gltf_extras) in q_parent.iter() {
		let v: Value = serde_json::from_str(&gltf_extras.value).expect("Couldn't parse GltfExtra value as JSON");
		if v["type"].as_str() == Some("coin") {
			commands.spawn_bundle(CoinBundle {
				_c: Coin,
				
				physics: PhysicsBundle {
					collider: Collider::cuboid(0.6, 0.2, 0.6),
					..default()
				},
				
				pbr: PbrBundle {
					mesh: assets_mesh.add(Mesh::from(shape::Torus {
						radius: 0.5,
						ring_radius: 0.1,
						subdivisions_segments: 16,
						subdivisions_sides: 16,
					})),
					material: materials.add(golden_mat()),
					transform: Transform {
						translation: t.translation,
						rotation: Quat::from_axis_angle(Vec3::X, radian(0.0)),
						scale: Vec3::new(1.0, 1.0, 1.0),
					},
					..default()
				},
			}).insert(InGame);
		}
	}	
}

fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(0.0, 16.0, -16.0);
    let radius = translation.length();

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(translation)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(PanOrbitCamera {
        radius,
        ..Default::default()
    }).insert(InGame);
}

fn control_character(
	keys: Res<Input<KeyCode>>,
	mut camera_query: Query<(&mut PanOrbitCamera, &Transform), Without<Player>>,
	mut player_query: Query<(&mut Health, &mut Transform, &mut Velocity, &mut ExternalImpulse, &mut IsGround), With<Player>>,
	pl_child_query: Query<Entity, With<PlayerChild>>,
	finish_query: Query<Entity, With<FinishTrigger>>,
	ground_query: Query<Entity, With<Ground>>,
	damage_query: Query<Entity, With<DamageTrigger>>,
	mut collision_events: EventReader<CollisionEvent>,
){	
	let (mut _health, mut transform, mut _player_velocity, mut _player_impulse, mut is_ground) = player_query.single_mut();
	let (mut poc, camera_transform) = camera_query.single_mut();
	let player_child = pl_child_query.single();
	
	for collision_event in collision_events.iter() {
		if let CollisionEvent::Started(ent1, ent2, _flags) = collision_event {
			for ground_ent in ground_query.iter() {
				if (ground_ent.eq(ent1) && player_child.eq(ent2)) || (ground_ent.eq(ent2) && player_child.eq(ent1)) {
					is_ground.0 = true;
				}
			}
			
			for finish_ent in finish_query.iter() {
				if (finish_ent.eq(ent1) && player_child.eq(ent2)) || (finish_ent.eq(ent2) && player_child.eq(ent1)) {
					println!("Vi ekvenkis!");
				}
			}
			
			for damage_ent in damage_query.iter() {
				if (damage_ent.eq(ent1) && player_child.eq(ent2)) || (damage_ent.eq(ent2) && player_child.eq(ent1)) {
					println!("Ok");
					transform.translation = Vec3::new(-5.0, 0.0, 0.0);
					_player_velocity.linvel = Vec3::ZERO;
				}
			}
		}
		
		if let CollisionEvent::Stopped(ent1, ent2, _flags) = collision_event {
			for ground_ent in ground_query.iter() {
				if (ground_ent.eq(ent1) && player_child.eq(ent2)) || (ground_ent.eq(ent2) && player_child.eq(ent1)) {
					is_ground.0 = false;
				}
			}
			
			for damage_ent in damage_query.iter() {
				if (damage_ent.eq(ent1) && player_child.eq(ent2)) || (damage_ent.eq(ent2) && player_child.eq(ent1)) {
					//
				}
			}
		}
	}
	
	let ct0 = Vec3::new(camera_transform.translation.x,			//
						camera_transform.translation.y - 16.0,	//Kameraa Transformo je NULA Alteco
						camera_transform.translation.z); 		//
	let direct_vector = Vec3::new((transform.translation.x - ct0.x) / 16.0,
								  0.0,
								  (transform.translation.z - ct0.z) / 16.0);
	let perp_vector = Quat::from_axis_angle(Vec3::Y, radian(90.0)).mul(direct_vector);
	
	if keys.pressed(KeyCode::W) { _player_impulse.impulse = direct_vector * 2.0;}
	if keys.pressed(KeyCode::S) { _player_impulse.impulse = -direct_vector * 2.0;}
	if keys.pressed(KeyCode::A) { _player_impulse.impulse = perp_vector * 2.0;}
	if keys.pressed(KeyCode::D) { _player_impulse.impulse = -perp_vector * 2.0;}
	
	if keys.pressed(KeyCode::W) && keys.pressed(KeyCode::A) { _player_impulse.impulse = (direct_vector + perp_vector) * 2.0;}
	if keys.pressed(KeyCode::S) && keys.pressed(KeyCode::A) { _player_impulse.impulse = (perp_vector - direct_vector) * 2.0;}
	if keys.pressed(KeyCode::W) && keys.pressed(KeyCode::D) { _player_impulse.impulse = (direct_vector - perp_vector) * 2.0;}
	if keys.pressed(KeyCode::S) && keys.pressed(KeyCode::D) { _player_impulse.impulse =(-direct_vector - perp_vector) * 2.0;}
	   
	if keys.just_pressed(KeyCode::Space) && is_ground.0 == true { _player_impulse.impulse = Vec3::new(0.0, 50.0, 0.0);}
	
	_player_velocity.linvel = Vec3::new(_player_velocity.linvel.x.clamp(-20.0, 20.0),
										_player_velocity.linvel.y,
										_player_velocity.linvel.z.clamp(-20.0, 20.0));
		
	poc.focus = transform.translation;
}

fn get_coin(
	mut commands: Commands,
	mut q_player: Query<(Entity, &mut XP, &Player)>,
	mut q_coin: Query<(Entity, &Coin)>,
	mut collision_events: EventReader<CollisionEvent>,
){
	let (player_ent, mut xp, _player) = q_player.single_mut();
	
	for collision_event in collision_events.iter() {
		if let CollisionEvent::Started(ent1, ent2, _flags) = collision_event {
			for (coin_ent, _coin) in q_coin.iter_mut() {
				if (&player_ent == ent1 && &coin_ent == ent2) || (&player_ent == ent2 && &coin_ent == ent1) {
					xp.0 += 1;
					commands.entity(coin_ent).despawn();
				}
			}
		}
	}
}

fn texture_filtering(
	mut tex_events: EventReader<AssetEvent<Image>>,
	mut images: ResMut<Assets<Image>>
) {
	let desc = SamplerDescriptor {
		mag_filter: FilterMode::Linear,
		min_filter: FilterMode::Linear,
		mipmap_filter: FilterMode::Linear,
		
		..default()
	};
		
	for event in tex_events.iter() {
		if let AssetEvent::Created{handle} = event {
			if let Some(txt) = images.get_mut(handle) {
				txt.sampler_descriptor = desc.clone();
			}
		}
	}
}

fn diffuse_mat(path: &str, assets: &Res<AssetServer>) -> StandardMaterial {	
	StandardMaterial {
		base_color_texture: Some(assets.load(path)),
		alpha_mode: AlphaMode::Blend,
        unlit: true,
		..default()
	}
}

fn golden_mat() -> StandardMaterial {
	return StandardMaterial {
		base_color: Color::GOLD,
		perceptual_roughness: 0.2,
		metallic: 1.0,
		reflectance: 1.0,
		unlit: false,
		double_sided: false,
		..default()
	};
}

fn _print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn radian(deg: f32) -> f32 {
	return deg * pi / 180.0; 
}
