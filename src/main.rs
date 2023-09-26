use bevy::{prelude::*, render::camera::ScalingMode};
use player::Player;
use player::PlayerPlugin;
mod player;
use ui::GameUi;
mod ui;



fn main() {
    println!("hello World");
    App::new()
    .add_plugins((PlayerPlugin, GameUi))
    .add_plugins(
        DefaultPlugins
        .set(ImagePlugin::default_nearest())//makes pixel art look better
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ice Boy Adventure".into(),
                ..default()
            }),
            ..default()
        })
        .build())
    .add_systems(Startup, (setup,spawnCounter))
    .add_systems(Update, (move_camera))
    .run();
}


fn setup(mut commands:Commands){
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 128.0,
    };

    commands.spawn(camera);


}

fn move_camera(
    mut q: Query<(&mut Transform, &Camera), Without<Player>>,
    player: Query<&Transform, With<Player>>
){
    let player_transform = player.get_single().expect("more than one player");
        for (mut transform, _) in &mut q{
            transform.translation.y = player_transform.translation.y;
            transform.translation.x = player_transform.translation.x;
        }

}
fn spawnCounter(mut commands:Commands, asset_server: Res<AssetServer>){
    let texture = asset_server.load("counter1.png");
    let tex= asset_server.load("kitchenbg1.png");
    
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            ..default()
        },
        texture:tex,
        ..default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            ..default()
        },
        texture,
        ..default()
    });
}