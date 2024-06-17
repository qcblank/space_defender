use bevy::prelude::*;

pub const SHOP_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.);
    style.height = Val::Percent(100.);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Percent(8.0);
    style.column_gap = Val::Percent(8.0);
    style
};
