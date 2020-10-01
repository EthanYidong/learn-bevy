mod components;
mod resources;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use components::*;
use resources::*;

macro_rules! load_assets {
    ($asset_server:expr, $materials:expr, $($path:expr),+) => {
        {
            let mut assets = Vec::new();
            $(
                assets.push(
                    $materials.add(
                        $asset_server.load($path).unwrap().into()
                    )
                );
            )+
            assets
        }
    };
}

fn main() {
    App::build()
        .add_stage_after(stage::POST_UPDATE, "detection")
        .add_stage_after("detection", "handle_events")
        .add_stage_after("handle_events", "cleanup")
        .add_resource(WindowDescriptor {
            title: "Space Shooter".to_string(),
            width: 1024,
            height: 1024,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_event::<CollisionEvent>()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(player_control.system())
        .add_system(laser_move.system())
        .add_system(weapons.system())
        .add_system(enemy_spawn.system())
        .add_system(environment_move.system())
        .add_system_to_stage("detection", collision_detection.system())
        .add_system_to_stage("handle_events", handle_collisions.system())
        .add_system_to_stage("cleanup", bounding_box.system())
        .add_system_to_stage("cleanup", despawn_dead.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_texture_handle = asset_server
        .load("assets/textures/playerShip1_blue.png")
        .unwrap();
    commands
        .insert_resource(MaterialHandles(load_assets![
            asset_server,
            materials,
            "assets/textures/laserBlue01.png",
            "assets/textures/laserRed01.png",
            "assets/textures/enemyRed1.png"
        ]))
        .insert_resource(BoundingBox {
            width: 1536.0,
            height: 1536.0,
        })
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(5.0, true)))
        .insert_resource(CollisionEventReader::default())
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(player_texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, -256.0, 0.0)),
            ..Default::default()
        })
        .with(Player { speed: 400.0 })
        .with(Weapon {
            fired: false,
            offset: Vec3::new(0.0, 60.0, 0.0),
            cooldown: Timer::from_seconds(0.4, false),
            material_id: 0,
        })
        .with(LoseHealthOnCollide)
        .with(CollisionDamage(1))
        .with(Health(1))
        .with(DeathBehavior::Despawn);
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
        transform.translate(Vec3::new(
            movement * player.speed * time.delta_seconds,
            0.0,
            0.0,
        ));
        if let Some(mut w) = weapon {
            w.fired = weapon_fired || w.fired;
        }
    }
}

fn weapons(
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
                    material: materials[weapon.material_id],
                    transform: Transform::from_translation(weapon.offset + transform.translation()),
                    ..Default::default()
                })
                .with(Laser { speed: 1000.0 })
                .with(Bounded)
                .with(LoseHealthOnCollide)
                .with(CollisionDamage(1))
                .with(Health(1))
                .with(DeathBehavior::Despawn);
            weapon.fired = false;
            weapon.cooldown.reset();
        }
    }
}

fn laser_move(time: Res<Time>, mut query: Query<(&Laser, &mut Transform)>) {
    for (laser, mut transform) in &mut query.iter() {
        transform.translate(Vec3::new(0.0, laser.speed * time.delta_seconds, 0.0))
    }
}

fn bounding_box(
    mut commands: Commands,
    bounds: Res<BoundingBox>,
    mut query: Query<(Entity, &Bounded, &Transform)>,
) {
    for (entity, _, transform) in &mut query.iter() {
        let translation = transform.translation();
        let is_oob = translation.x() < -bounds.width / 2.0
            || translation.x() > bounds.width / 2.0
            || translation.y() < -bounds.height / 2.0
            || translation.y() > bounds.height / 2.0;
        if is_oob {
            commands.despawn(entity);
        }
    }
}

fn enemy_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    materials: Res<MaterialHandles>,
) {
    spawn_timer.tick(time.delta_seconds);
    if spawn_timer.finished {
        for i in -3..=3 {
            commands
                .spawn(SpriteComponents {
                    material: materials[2],
                    transform: Transform::from_translation(Vec3::new(i as f32 * 128.0, 600.0, 0.0)),
                    ..Default::default()
                })
                .with(Environment)
                .with(Bounded)
                .with(Enemy)
                .with(LoseHealthOnCollide)
                .with(CollisionDamage(1))
                .with(Health(1))
                .with(DeathBehavior::Despawn);
        }
    }
}

fn environment_move(time: Res<Time>, mut query: Query<(&Environment, &mut Transform)>) {
    for (_, mut transform) in &mut query.iter() {
        transform.translate(Vec3::new(0.0, -256.0 * time.delta_seconds, 0.0));
    }
}

fn collision_detection(
    mut collision_events: ResMut<Events<CollisionEvent>>,
    mut query_1: Query<(Entity, &Transform, &Sprite,)>,
) {
    let colliders: Vec<(Entity, Vec3, Vec2)> = query_1
        .iter()
        .into_iter()
        .map(|(entity, transform, sprite)| (entity, transform.translation(), sprite.size))
        .collect();

    for (i, (entity_1, translation_1, size_1)) in colliders.iter().copied().enumerate() {
        for (entity_2, translation_2, size_2) in colliders.iter().copied().skip(i + 1) {
            if let Some(_) = collide(translation_1, size_1, translation_2, size_2) {
                collision_events.send(CollisionEvent(entity_1, entity_2));
                collision_events.send(CollisionEvent(entity_2, entity_1));
            }
        }
    }
}

fn handle_collisions(
    events: Res<Events<CollisionEvent>>,
    mut event_reader: ResMut<CollisionEventReader>,
    health_query: Query<(&mut Health, &LoseHealthOnCollide)>,
    damage_query: Query<(&CollisionDamage,)>,
) {
    for event in event_reader.iter(&events) {
        if let Ok(dmg) = damage_query.get::<CollisionDamage>(event.0) {
            if let Ok(mut health) = health_query.get_mut::<Health>(event.1) {
                health.0 -= dmg.0;
            }
        }
    }
}

fn despawn_dead(mut commands: Commands, mut query: Query<(Entity, &Health, &DeathBehavior)>) {
    for (entity, health, death_behavior) in &mut query.iter() {
        if health.0 < 0 {
            match death_behavior {
                DeathBehavior::Despawn => {
                    commands.despawn(entity);
                }
                DeathBehavior::None => (),
            }
        }
    }
}
