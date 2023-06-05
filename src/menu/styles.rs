use bevy::prelude::*;

use super::resources::MenuResources;

pub const MAIN_MENU_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
    gap: Size::new(Val::Px(9.), Val::Px(9.)),
    ..Style::DEFAULT
};

pub const GAME_OVER_MENU_STYLE: Style = MAIN_MENU_STYLE;

pub const TITLE_STYLE: Style = Style {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(300.), Val::Px(120.)),
    ..Style::DEFAULT
};

pub const PLAY_BUTTON_STYLE: Style = Style {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(100.), Val::Px(100.)),
    ..Style::DEFAULT
};

pub fn text_style(menu_resource: &Res<MenuResources>, font_size: f32, color: Color) -> TextStyle {
    TextStyle {
        font: menu_resource.font.clone(),
        font_size,
        color,
    }
}
