#[derive(Clone, PartialEq, Eq)]
pub enum EntityType {
    Player,
    Zombie,
    FlyingBallLinear, // Projectile for linear ranged attacks
    FlyingBallArc, // Projectile for arc ranged attacks
}
