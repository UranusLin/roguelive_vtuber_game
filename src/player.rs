use bevy::prelude::*;

struct Player;

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>
) {
    for (_player, mut transform) in query.iter_mut() {
        const MOVEMENT_SPEED: f32 = 4.0;
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += MOVEMENT_SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= MOVEMENT_SPEED;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= MOVEMENT_SPEED;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += MOVEMENT_SPEED;
        }
    }
}

struct Enemy;

fn enemy_movement(
    mut query: Query<(&Enemy, &mut Transform)>,
    player_query: Query<(&Player, &Transform)>
) {
    if let Some((_player, player_transform)) = player_query.iter().next() {
        for (_enemy, mut enemy_transform) in query.iter_mut() {
            const ENEMY_SPEED: f32 = 2.0;
            let direction = (player_transform.translation - enemy_transform.translation).normalize();
            enemy_transform.translation += direction * ENEMY_SPEED;
        }
    }
}