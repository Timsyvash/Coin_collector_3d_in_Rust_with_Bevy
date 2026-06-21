use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

use crate::game::game::*;
use crate::player::player_move::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub rotation_speed: f32,
    pub jump_force: f32,
    pub velocity_y: f32,
    pub is_grounded: bool,
}

#[allow(dead_code)]
#[derive(Resource, Default)]
pub struct PlayerPhysicsState {
    pub vertical_velocity: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerPhysicsState>()
            .add_systems(Startup, init_player)
            .add_systems(Update, (player_movement, player_jump).run_if(in_state(GameState::Playing)));
    }
}

fn init_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let flashlight = (
        SpotLightBundle {
            spot_light: SpotLight {
                color: Color::rgba(1.0, 0.96, 0.37, 1.0),
                intensity: 4000.0,
                outer_angle: 0.6,
                inner_angle: 0.5,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.5, -0.2),
            ..default()
        },
        Name::new("Flashlight"),
    );

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player {
            speed: 7.5,
            rotation_speed: 10.0,
            jump_force: 10.0,
            velocity_y: 0.0,
            is_grounded: true,
        },
        ThirdPersonCameraTarget,
        RigidBody::KinematicPositionBased,
        KinematicCharacterController {
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(0.5),
                min_width: CharacterLength::Absolute(0.2),
                include_dynamic_bodies: true,
            }),
            snap_to_ground: Some(CharacterLength::Absolute(0.2)),
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn(SceneBundle {
            scene: asset_server.load("Player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
        parent.spawn(flashlight);
    });
}
