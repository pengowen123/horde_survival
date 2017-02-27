use consts::graphics::gui::SHOP_MATRIX_COLUMNS;
use items::shop::ShopItem;

// A matrix of items in the shop
// If an item is SHOP_EMPTY, the slot is empty
pub type ShopMatrix = Vec<[(bool, ShopItem); SHOP_MATRIX_COLUMNS]>;

pub struct WidgetStates {
    pub weapon_matrix: ShopMatrix,
    pub armor_matrix: ShopMatrix,
    pub other_matrix: ShopMatrix,
}

impl Default for WidgetStates {
    fn default() -> Self {
        use std::mem;

        let mut matrices = get_shop_matrix();

        // mem::replace to move out of the array
        WidgetStates {
            weapon_matrix: mem::replace(&mut matrices[0], Vec::new()),
            armor_matrix: mem::replace(&mut matrices[0], Vec::new()),
            other_matrix: mem::replace(&mut matrices[0], Vec::new()),
        }
    }
}

fn get_shop_matrix() -> [ShopMatrix; 3] {
    use consts::items::weapon::*;

    let mut weapons =
        vec![[(false, SHOP_TEST_SWORD), (false, SHOP_TEST_BOW), (false, SHOP_TEST_GUN)],
             [(false, SHOP_TEST_SWORD), (false, SHOP_TEST_BOW), (false, SHOP_TEST_GUN)],
             [(false, SHOP_TEST_SWORD), (false, SHOP_TEST_BOW), (false, SHOP_TEST_GUN)],
             [(false, SHOP_TEST_SWORD), (false, SHOP_TEST_BOW), (false, SHOP_TEST_GUN)],
             [(false, SHOP_TEST_SWORD), (false, SHOP_TEST_BOW), (false, SHOP_TEST_GUN)],
             [(false, SHOP_TEST_SWORD), (false, SHOP_EMPTY), (false, SHOP_EMPTY)]];

    let mut armor =
        vec![[(false, SHOP_TEST_SWORD), (false, SHOP_TEST_BOW), (false, SHOP_TEST_GUN)],
             [(false, SHOP_TEST_SWORD), (false, SHOP_TEST_BOW), (false, SHOP_TEST_GUN)],
             [(false, SHOP_TEST_SWORD), (false, SHOP_TEST_BOW), (false, SHOP_EMPTY)]];

    let mut other =
        vec![[(false, SHOP_TEST_SWORD), (false, SHOP_TEST_BOW), (false, SHOP_TEST_GUN)],
             [(false, SHOP_TEST_SWORD), (false, SHOP_TEST_BOW), (false, SHOP_TEST_GUN)]];

    // Select the first item of each matrix
    weapons[0][0].0 = true;
    armor[0][0].0 = true;
    other[0][0].0 = true;

    [weapons, armor, other]
}
