use bevy::prelude::*;
use std::f32::consts::PI;
use crate::player::player::*;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&Player, &mut Transform)>
) {
    let Ok((player, mut transform)) = player_query.get_single_mut() else { return; };

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        let direction = direction.normalize_or_zero();

        transform.translation += direction * player.speed * time.delta_seconds();

        let target_angle = direction.x.atan2(direction.z) + PI;

        let target_rotation = Quat::from_rotation_y(target_angle);

        transform.rotation = transform.rotation.slerp(
            target_rotation,
            player.rotation_speed * time.delta_seconds()
        );
    }
}

pub fn player_jump(
    key_code: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform)>
) {
    let Ok((mut player, mut transform)) = player_query.get_single_mut() else {return;};
    let delta = time.delta_seconds();

    let gravity = -10.0;
    player.velocity_y += gravity * delta;

    if key_code.just_pressed(KeyCode::Space) && player.is_grounded {
        player.velocity_y = player.jump_force;
        player.is_grounded = false;
    }

    transform.translation.y += player.velocity_y * delta;

    if transform.translation.y <= 0.5 {
        transform.translation.y = 0.5;
        player.velocity_y = 0.0;
        player.is_grounded = true;
    }
}