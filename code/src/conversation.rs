use bevy::{
	prelude::*,
	text::Text2dBounds,
};

#[derive(Component)]
pub struct Hero;
#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct DialogueBox;
#[derive(Component)]
pub struct UserInput;

enum ConversationState {
    Introduction,
    Conversation,
    GoodEnding,
	BadEnding
}


pub fn setup_conversation(
	mut commands: Commands,
	mut clear_color: ResMut<ClearColor>, 
	asset_server: Res<AssetServer>,
){
    clear_color.0 = Color::DARK_GREEN;
    let user_text_style = TextStyle {
		font: asset_server.load("Fonts/SourceSansPro-Regular.ttf"),
        font_size: 60.0,
        color: Color::WHITE
    };
    let enemy_text_style = TextStyle {
		font: asset_server.load("Fonts/SourceSansPro-Regular.ttf"),
        font_size: 60.0,
        color: Color::BLACK
    };

    commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("hero.png"),
		transform: Transform::from_xyz(-500., -225., 2.),
		sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(200., 200.)),
            ..default()
        },
		..default()
	}).insert(Hero);

	commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("enemy.png"),
		transform: Transform::from_xyz(500., 200., 2.),
		sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(200., 200.)),
            ..default()
        },
		..default()
	}).insert(Enemy);

	let box_size = Vec2::new(700.0, 200.0);
    let box_position = Vec2::new(-45.0, -250.0);
    let box_position_two = Vec2::new(45.0, 175.0);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::DARK_GRAY,
            custom_size: Some(Vec2::new(box_size.x, box_size.y)),
            ..default()
        },
        transform: Transform::from_translation(box_position.extend(0.0)),
        ..default()
    }).insert(DialogueBox);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(box_size.x, box_size.y)),
            ..default()
        },
        transform: Transform::from_translation(box_position_two.extend(0.0)),
        ..default()
    }).insert(DialogueBox);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Excuse me neighbor, can I borrow some sugar?", enemy_text_style),
        text_2d_bounds: Text2dBounds {
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position_two.x - box_size.x / 2.0,
            box_position_two.y + box_size.y / 2.0,
            1.0,
        ),
        ..default()
    }).insert(DialogueBox);
    
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Press enter to display input", user_text_style),
        text_2d_bounds: Text2dBounds {
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position.x - box_size.x / 2.0,
            box_position.y + box_size.y / 2.0,
            1.0,
        ),
        ..default()
    }).insert(DialogueBox)
      .insert(UserInput);
	//info!("Setting Up: GameState: Conversation");
}

pub fn clear_conversation(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut hero: Query<Entity, With<Hero>>,
	mut enemy: Query<Entity, With<Enemy>>,
    dialogue: Query<Entity, With<DialogueBox>>,

) {
    clear_color.0 = Color::BLACK;
    for entity in dialogue.iter() {
        commands.entity(entity).despawn();
    }
    let hero_eid = hero.single_mut();
	let enemy_eid = enemy.single_mut();
    commands.entity(hero_eid).despawn();
	commands.entity(enemy_eid).despawn();
}

pub fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
	mut dialogue: Query<&mut Text, With<UserInput>>
) {
	let mut dialogue_text = dialogue.single_mut();

	for ev in char_evr.iter() {

		if keys.just_pressed(KeyCode::Return) {
			string.clear();	
            dialogue_text.sections[0].value = "".to_string();
		} else
		if keys.just_pressed(KeyCode::Back) {
			string.pop();
			dialogue_text.sections[0].value = string.to_string();
		} else {
			string.push(ev.char); 
			dialogue_text.sections[0].value = string.to_string();
		}
	}
}
