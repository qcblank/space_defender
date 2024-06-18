use bevy::prelude::*;

#[derive(Component)]
pub struct ShopMenu {}

#[derive(Component)]
pub struct PlayerScore {}

#[derive(Component)]
pub struct BuyButton {
    item: ShopItem,
}

impl BuyButton {
    pub fn with(item: ShopItem) -> Self {
        Self { item }
    }

    pub fn get_item(&self) -> &ShopItem {
        &self.item
    }
}

pub enum Upgrade {
    Firerate,
}

#[derive(Component)]
pub struct ShopItem {
    upgrade: Upgrade,
    price: u32,
}

impl ShopItem {
    pub fn with(upgrade: Upgrade, price: u32) -> Self {
        Self { upgrade, price }
    }

    pub fn get_price(&self) -> u32 {
        self.price
    }
}
