use consts::balance::GLOBAL_ATTACK_TIME;
use utils::time;

pub fn get_attack_time(attack_speed: f64) -> usize {
    let x = 1.0 / attack_speed;

    time(x * GLOBAL_ATTACK_TIME)
}
