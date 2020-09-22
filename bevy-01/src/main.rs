use bevy::prelude::*;

struct Player;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(player_movement.system())
        .run();
}

fn setup(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            ..Default::default()
        })
        .with(Player);
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, _player) in &mut query.iter() {
        let translation = transform.translation_mut();
        if keyboard_input.pressed(KeyCode::Right) {
            *translation.x_mut() += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            *translation.x_mut() -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            *translation.y_mut() += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            *translation.y_mut() -= 1.0;
        }
    }
}
