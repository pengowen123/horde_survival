use entity::modifiers::*;
use consts::time;

// Warrior
pub const WARRIOR_COOLDOWN_0: usize = time(7.0);
pub const WARRIOR_COOLDOWN_1: usize = time(8.0);
pub const WARRIOR_COOLDOWN_2: usize = time(10.0);
pub const WARRIOR_COOLDOWN_3: usize = time(12.0);

pub const WARRIOR_FORTIFY: Modifier = Modifier::new(0.5, time(3.0));

pub const WARRIOR_MAIM_DAMAGE: Modifier = Modifier::new(0.65, time(4.0));
pub const WARRIOR_MAIM_SLOW: Modifier = Modifier::new(0.65, time(4.0));
pub const WARRIOR_MAIM_RADIUS: f64 = 3.0;

pub const WARRIOR_RAGE_AS: Modifier = Modifier::new(1.3, time(4.0));
pub const WARRIOR_RAGE_DAMAGE: Modifier = Modifier::new(1.3, time(4.0));

pub const WARRIOR_EXECUTE_DAMAGE: f64 = 1000.0;
pub const WARRIOR_EXECUTE_RADIUS: f64 = 4.0;
