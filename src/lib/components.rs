use bevy::prelude::*;
use bevy::window::*;
use bevy::gltf::{Gltf, GltfNode, GltfMesh, GltfExtras};
use bevy::render::render_resource::{SamplerDescriptor, FilterMode};
use bevy_rapier3d::prelude::*;

#[derive(Default)]
pub struct GltfMeshes (pub Handle<Gltf>);

#[derive(Default)]
pub struct Win(pub bool);

#[derive(Default)]
pub struct LoadedMeshes(pub Vec<Handle<Mesh>>);

#[derive(Component)]
pub struct XP(pub u16);

#[derive(Component)]
pub struct PlayerName(pub String);

#[derive(Component)]
pub struct Health(pub u8);

#[derive(Component)]
pub struct IsGround(pub bool);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Dynamics;

#[derive(Component)]
pub struct PlayerChild;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Coin;

#[derive(Component)]
pub struct FinishTrigger;

#[derive(Component)]
pub struct DamageTrigger;

#[derive(Bundle)]
pub struct PhysicsBundle {
	pub rigidbody: RigidBody,
	pub collider: Collider,
	pub sensor: Sensor,
	pub friction: Friction,
	pub restitution: Restitution,
	pub is_ground: IsGround,
	
	pub velocity: Velocity,
	pub gravity: GravityScale,
	pub mass_properties: MassProperties,
	pub locked_axes: LockedAxes,
	pub dominance: Dominance,
	pub sleeping: Sleeping,
	pub damping: Damping,
	pub ccd: Ccd,
	pub act: ActiveCollisionTypes,
	pub events: ActiveEvents,
	
	pub force: ExternalForce,
	pub impulse: ExternalImpulse,
}

impl Default for PhysicsBundle {
	fn default() -> PhysicsBundle {
		return PhysicsBundle {
			rigidbody: RigidBody::Fixed,
			collider: Collider::cuboid(1.0, 1.0, 1.0),
			restitution: Restitution::default(),
			sensor: Sensor(true),
			friction: Friction::default(),
			is_ground: IsGround(false),
			
			gravity: GravityScale(1.0),
			velocity: Velocity {
				linvel: Vec3::new(0.0, 0.0, 0.0),
				angvel: Vec3::new(0.0, 0.0, 0.0),
			},
			mass_properties: MassProperties::default(),
			locked_axes: LockedAxes::default(),
			damping: Damping::default(),
			dominance: Dominance::group(0),
			sleeping: Sleeping::disabled(),
			ccd: Ccd::enabled(),
			act: ActiveCollisionTypes::default(),
			events: ActiveEvents::COLLISION_EVENTS,
			
			force: ExternalForce::default(),
			impulse: ExternalImpulse::default(),
		}
	}
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub xp: XP,
    pub name: PlayerName,
    pub health: Health,
    pub _p: Player,
	
	#[bundle]
	pub physics: PhysicsBundle,
    
    #[bundle]
    pub pbr: PbrBundle,
}

#[derive(Bundle)]
pub struct CoinBundle {
	pub _c: Coin,
	
	#[bundle]
	pub physics: PhysicsBundle,
	
	#[bundle]
	pub pbr: PbrBundle,
}
