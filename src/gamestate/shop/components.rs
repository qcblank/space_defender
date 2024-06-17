use bevy::prelude::*;

#[derive(Component)]
pub struct ShopMenu {}

#[derive(Component)]
pub struct BuyButton {}

#[derive(Component)]
pub struct Item {
    price: u16,
}
