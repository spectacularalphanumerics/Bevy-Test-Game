use bevy::prelude::*;

const GRAVITY: f32 = -20.8;
const JUMP_FORCE: f32 = 100.0;
const MOVE_SPEED: f32 = 100.0;
const GROUND_LEVEL: f32 = -100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup) // Use `add_systems` with `Startup`
        .add_systems(Update, (player_movement, apply_gravity, jump, ground_collision)) // Use `add_systems` with `Update`
        .run();
}

#[derive(Component)]
struct Player {
    velocity: Vec3,
    is_jumping: bool,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default()); 

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                image: asset_server.load("character.png"),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, GROUND_LEVEL, 0.0),
            ..Default::default()
        },
        Player {
            velocity: Vec3::ZERO,
            is_jumping: false,
        },
    ));
}
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Use `ButtonInput` instead of `Input`
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let delta_time = time.delta().as_secs_f32();
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= MOVE_SPEED * delta_time;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += MOVE_SPEED * delta_time;
        }
    }
}

fn apply_gravity(mut query: Query<(&mut Player, &mut Transform)>, time: Res<Time>) {
    for (mut player, mut transform) in query.iter_mut() {
        let delta_time = time.delta().as_secs_f32();
        if transform.translation.y > GROUND_LEVEL || player.is_jumping {
            player.velocity.y += GRAVITY * delta_time;
            transform.translation += player.velocity * delta_time;
        }
    }
}

fn jump(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Use `ButtonInput` instead of `Input`
    mut query: Query<(&mut Player, &Transform)>,
) {
    for (mut player, transform) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) && transform.translation.y <= GROUND_LEVEL {
            player.velocity.y = JUMP_FORCE;
            player.is_jumping = true;
        }
    }
}

fn ground_collision(mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in query.iter_mut() {
        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            player.velocity.y = 0.0;
            player.is_jumping = false;
        }
    }
}