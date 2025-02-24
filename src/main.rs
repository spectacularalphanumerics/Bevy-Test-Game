#![allow(missing_docs)]
use bevy::prelude::*;
use bevy::ecs::system::ParamSet;

const GRAVITY: f32 = -20.8;
const JUMP_FORCE: f32 = 10000.0;
const MOVE_SPEED: f32 = 100.0;
const GROUND_LEVEL: f32 = -100.0;
#[derive(Component)]
struct Platform;

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

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(200.0, 20.0)),
                color: Color::rgb(0.0, 10.0, 0.0),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -150.0, 0.0),
            ..Default::default()
        },
        Platform,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(200.0, 20.0)),
                color: Color::rgb(0.0, 10.0, 0.0),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -150.0, 0.0),
            ..Default::default()
        },
        Platform,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(200.0, 20.0)),
                color: Color::rgb(0.0, 1.0, 0.0),
                ..Default::default()
            },
            transform: Transform::from_xyz(200.0, -50.0, 0.0),
            ..Default::default()
        },
        Platform,
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

fn apply_gravity(
    mut queries: ParamSet<(
        Query<(&mut Player, &mut Transform)>,
        Query<(&Transform, &Sprite), With<Platform>>,
    )>,
    time: Res<Time>,
) {

   let platforms: Vec<(Transform, Sprite)> = {
    let platform_query = queries.p1();
    platform_query.iter().map(|(transform, sprite)| (transform.clone(), sprite.clone())).collect()
   };
   for (mut player, mut transform) in queries.p0().iter_mut() {
    let delta_time = time.delta().as_secs_f32();

    player.velocity.y += GRAVITY * delta_time;
    transform.translation += player.velocity * delta_time;

    for (platform_transform, platform_sprite) in &platforms {
        if check_collision(&transform, platform_transform, platform_sprite.custom_size.unwrap()) {
            if player.velocity.y < 0.0 {
                transform.translation.y = platform_transform.translation.y + platform_sprite.custom_size.unwrap().y / 2.0 + 25.0;
                player.velocity.y = 0.0;
                player.is_jumping = false;
            }
        }
    }
   }
}

fn ground_collision(mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in query.iter_mut() {
        if transform.translation.y <= GROUND_LEVEL {
            // transform.translation.y = GROUND_LEVEL;
            // player.velocity.y = 0.0;
            // player.is_jumping = false;
        }
    }
}

fn jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &Transform)>,
    platform_query: Query<(&Transform, &Sprite), With<Platform>>,
) {
    for (mut player, player_transform) in query.iter_mut() {
        let mut can_jump = false;

        // Check if the player is on a platform
        for (platform_transform, platform_sprite) in platform_query.iter() {
            if check_collision(player_transform, platform_transform, platform_sprite.custom_size.unwrap()) {
                can_jump = true;
                break;
            }
        }

        // Allow jumping if the player is on a platform and the space key is pressed
        if keyboard_input.just_pressed(KeyCode::Space) && can_jump {
            println!("Jumping! Velocity: {}", JUMP_FORCE);
            player.velocity.y = JUMP_FORCE;
            player.is_jumping = true;
        }
    }
}


fn check_collision(player_transform: &Transform, platform_transform: &Transform, platform_size: Vec2) -> bool {
    let player_size = Vec2::new(50.0, 50.0);

    let player_min = player_transform.translation.truncate() - player_size / 2.0;
    let player_max = player_transform.translation.truncate() + player_size / 2.0;

    let platform_min = platform_transform.translation.truncate() - platform_size / 2.0;
    let platform_max= platform_transform.translation.truncate() + platform_size / 2.0;

    player_min.x < platform_max.x &&
    player_max.x > platform_min.x &&
    player_min.y < platform_max.y &&
    player_max.y > platform_min.y
}