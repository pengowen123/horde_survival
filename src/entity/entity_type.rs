/// The type of an entity
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityType {
    Player,
    Zombie,
    FlyingBallLinear, // Projectile for linear ranged attacks
    FlyingBallArc, // Projectile for arc ranged attacks
}
