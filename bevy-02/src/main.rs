use bevy::prelude::*;

struct Player {
    speed: f32,
}

struct Weapon {
    fired: bool,
    offset: Vec3,
    cooldown: Timer,
    material_id: usize,
}

struct Laser {
    speed: f32,
}

struct MaterialHandles(Vec<Handle<ColorMaterial>>);

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Space Shooter".to_string(),
            width: 1024,
            height: 1024,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(player_control.system())
        .add_system(laser_move.system())
        .add_system_to_stage(stage::POST_UPDATE, weapons.system())
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let player_texture_handle = asset_server.load("assets/textures/playerShip1_blue.png").unwrap();
    let laser_texture_handle = asset_server.load("assets/textures/laserBlue01.png").unwrap();

    commands
        .insert_resource(MaterialHandles(vec![materials.add(laser_texture_handle.into())]))
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(player_texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, -256.0, 0.0)),
            ..Default::default()
        })
        .with(Player { 
            speed: 400.0
        })
        .with(Weapon {
            fired: false,
            offset: Vec3::new(0.0, 60.0, 0.0),
            cooldown: Timer::from_seconds(0.4, false),
            material_id: 0,
        });
}

fn player_control(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, Option<&mut Weapon>)>,
) {
    let mut movement = 0.0;

    let weapon_fired = keyboard_input.just_pressed(KeyCode::Space);

    if keyboard_input.pressed(KeyCode::A) {
        movement -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement += 1.0;
    }

    for (player, mut transform, weapon) in &mut query.iter() {
        transform.translate(Vec3::new(movement * player.speed * time.delta_seconds, 0.0, 0.0));
        if let Some(mut w) = weapon {
            w.fired = weapon_fired;
        }
    }
}

fn weapons (
    mut commands: Commands,
    time: Res<Time>,
    materials: Res<MaterialHandles>,
    mut query: Query<(&mut Weapon, &Transform)>,
) {
    for (mut weapon, transform) in &mut query.iter() {
        weapon.cooldown.tick(time.delta_seconds);
        if weapon.cooldown.finished && weapon.fired {
            commands
                .spawn(SpriteComponents {
                    material: materials.0[weapon.material_id],
                    transform: Transform::from_translation(weapon.offset + transform.translation()),
                    ..Default::default()
                })
                .with(Laser {
                    speed: 1000.0
                });
            weapon.fired = false;
            weapon.cooldown.reset();
        }
    }
}

fn laser_move(
    time: Res<Time>,
    mut query: Query<(&Laser, &mut Transform)>
) {
    for (laser, mut transform) in &mut query.iter() {
        transform.translate(Vec3::new(0.0, laser.speed * time.delta_seconds, 0.0) )
    }
}
