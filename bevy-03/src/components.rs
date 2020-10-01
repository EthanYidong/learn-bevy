use bevy::prelude::*;
use derive_deref::*;

pub struct Player {
    pub speed: f32,
}

pub struct Weapon {
    pub fired: bool,
    pub offset: Vec3,
    pub cooldown: Timer,
    pub material_id: usize,
}

pub struct Laser {
    pub speed: f32,
}

//Marker components
pub struct Environment;
pub struct Enemy;
pub struct Bounded;
pub struct LoseHealthOnCollide;
pub struct CollisionDamage(pub i32);
pub struct Health(pub i32);

pub enum DeathBehavior {
    Despawn,
    None,
}

pub enum CollisionLayer {
    Player,
    Enemies,
    PlayerProjectiles,
    EnemyProjectiles,
}

#[derive(Deref)]
pub struct CollidesWith(Vec<CollisionLayer>);
