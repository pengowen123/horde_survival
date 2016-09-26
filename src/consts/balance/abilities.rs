use entity::modifiers::*;
use consts::time;

// Warrior
pub const WARRIOR_COOLDOWNS: [usize; 4] = [time(4.0), time(4.0), time(4.0), time(4.0)];

pub const WARRIOR_PRE_0: usize = time(1.0);
pub const WARRIOR_PRE_1: usize = time(1.0);
pub const WARRIOR_PRE_2: usize = time(1.0);
pub const WARRIOR_PRE_3: usize = time(1.0);
pub const WARRIOR_POST_0: usize = time(1.0);
pub const WARRIOR_POST_1: usize = time(1.0);
pub const WARRIOR_POST_2: usize = time(1.0);
pub const WARRIOR_POST_3: usize = time(1.0);

pub const WARRIOR_FORTIFY: Modifier = Modifier::multiplicative(0.5, time(3.0));

pub const WARRIOR_MAIM_DAMAGE: Modifier = Modifier::multiplicative(0.65, time(4.0));
pub const WARRIOR_MAIM_SLOW: Modifier = Modifier::multiplicative(0.65, time(4.0));
pub const WARRIOR_MAIM_RADIUS: f64 = 3.0;

pub const WARRIOR_RAGE_AS: Modifier = Modifier::multiplier(1.0, time(4.0));
pub const WARRIOR_RAGE_DAMAGE: Modifier = Modifier::multiplier(1.0, time(4.0));

pub const WARRIOR_EXECUTE_DAMAGE: f64 = 1000.0;
pub const WARRIOR_EXECUTE_RADIUS: f64 = 4.0;
