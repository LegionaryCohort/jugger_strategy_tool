use bevy::{
    ecs::component::Component,
    math::{Vec2, Vec3Swizzles},
    transform::components::Transform,
};

#[derive(Component, Copy, Clone, Debug)]
pub struct UnitSize(pub f32);

#[derive(Copy, Clone, Debug)]
pub struct ZLevel(pub f32);

#[derive(Copy, Clone, Debug)]
pub struct Position(pub Vec2);
impl Position {
    pub fn into_transform(&self, z_level: ZLevel) -> Transform {
        Transform::from_xyz(self.0.x, self.0.y, z_level.0)
    }
}
impl From<&Transform> for Position {
    fn from(transform: &Transform) -> Self {
        Self(transform.translation.xy())
    }
}

pub const SIZE_SCALING_FACTOR: f32 = 100.; // pixels per meter
pub fn radius_from_meters(radius: f32) -> f32 {
    radius * SIZE_SCALING_FACTOR
}
pub trait FromMeters {
    fn from_meters(x: f32, y: f32) -> Self;
}
impl FromMeters for Vec2 {
    fn from_meters(x: f32, y: f32) -> Self {
        Vec2::new(x, y) * SIZE_SCALING_FACTOR
    }
}
impl FromMeters for Position {
    fn from_meters(x: f32, y: f32) -> Self {
        Self(Vec2::from_meters(x, y))
    }
}
