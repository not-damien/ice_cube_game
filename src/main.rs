use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (setup, spawnPlayer))
    .add_systems(Update, character_movement)
    .run();
}


fn setup(mut commands:Commands){
    commands.spawn(Camera2dBundle::default());
}

fn spawnPlayer(mut commands:Commands, asset_server: Res<AssetServer>){
    let texture = asset_server.load("icecube100.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(62.0, 62.0)),
            ..default()
        },
        texture,
        ..default()
    });
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut characters {
        let speed = 200.0;
        if input.pressed(KeyCode::W) {
            transform.translation.y += speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += speed * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= speed * time.delta_seconds();
        }
    }
}