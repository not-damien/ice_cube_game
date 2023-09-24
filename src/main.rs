use bevy::{prelude::*, render::camera::ScalingMode};

fn main() {
    println!("hello World");
    App::new()
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
    .add_systems(Startup, (setup,spawnCounter, spawnPlayer))
    .add_systems(Update, (character_movement, melt,move_camera))
    .init_resource::<Health>()
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

fn melt(
    mut commands:Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut heath: ResMut<Health>,
    player: Query<&Transform, With<Player>>
){
    if !input.just_pressed(KeyCode::M){
        return;
    }

    let player_transform = player.get_single().expect("more than one player");
    //let player_transform= player.single();
    if heath.0 >= 10.0{
        heath.0 -= 10.0;
        println!("heath went downnnn");
        let texture = asset_server.load("icecube0.png");
        commands.spawn(
            SpriteBundle{
                texture,
                transform: *player_transform,
                ..default()
            }
        );
    }else{
        println!("you melted")
    }
}



#[derive(Resource)]
pub struct Health(pub f32);
impl Default for Health{
    fn default() -> Self {
        Health(100.)
    }
}
#[derive(Component)]
pub struct Player {
    pub speed: f32,
}


fn spawnPlayer(mut commands:Commands, asset_server: Res<AssetServer>){
    let texture = asset_server.load("icecube100.png");

    commands.spawn( (
        SpriteBundle {
            sprite: Sprite {
                //custom_size: Some(Vec2::new(62.0, 62.0)),
                
                ..default()
                },
            texture,
            transform: Transform::from_xyz(-100., -6.0, 1.),
            ..default()
        }
        ,
        Player { speed: 100.0 }));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let speed = 200.0;
        if input.pressed(KeyCode::W) {
            transform.translation.y += player.speed * time.delta_seconds();
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
        //println!("x = {}, y = {}",transform.translation.x, transform.translation.y);
    }
}