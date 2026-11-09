use bevy::{
    math::{Vec2, Vec3Swizzles},
    transform::components::Transform,
};

#[derive(Copy, Clone, Debug)]
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
