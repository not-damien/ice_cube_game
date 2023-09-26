use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup, spawnPlayer)
        .add_systems(Update, (character_movement, melt))
        .init_resource::<Health>();
    }
}



#[derive(Component)]
pub struct Player {
    pub speed: f32,
}
#[derive(Resource)]
pub struct Health(pub f32);
impl Default for Health{
    fn default() -> Self {
        Health(100.)
    }
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
