use bevy::prelude::*;

use crate::player::Health;

pub struct GameUi;
impl Plugin for GameUi{
    fn build(&self, app: &mut App){
        app.add_systems(Startup,spawn_game_ui)
        .add_systems(Update, update_game_ui);
    }
}
#[derive(Component)]
pub struct HeathText;

fn update_game_ui(mut texts: Query<&mut Text, With<HeathText>>, health: Res<Health>){
    let mut t = texts.get_single_mut().expect("Error in checking heath");
    t.sections[0].value = format!("Heath: {:?}", health.0);
}
fn spawn_game_ui(mut commands: Commands){
    commands
    .spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            //background_color: Color::BLUE.into(),
            ..default()
        },
        Name::new("UI Root"),
    ))
    .with_children(|commands| {
        commands.spawn((
            TextBundle {
                z_index:ZIndex::Global(5),
                text: Text::from_section(
                    "",
                    TextStyle {
                        font_size: 30.0,
                        ..default()
                    },
                ),
                ..default()
            },HeathText
        ));
    });
}