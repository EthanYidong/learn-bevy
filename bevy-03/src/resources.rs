use bevy::prelude::*;
use derive_deref::*;

#[derive(Deref)]
pub struct MaterialHandles(pub Vec<Handle<ColorMaterial>>);

pub struct CollisionEvent(pub Entity, pub Entity);

#[derive(Default, Deref, DerefMut)]
pub struct CollisionEventReader(pub EventReader<CollisionEvent>);

#[derive(Deref, DerefMut)]
pub struct EnemySpawnTimer(pub Timer);

pub struct BoundingBox {
    pub width: f32,
    pub height: f32,
}
