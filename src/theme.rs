use freya::prelude::*;

const SELECTED_ITEM_BG: Color = Color::from_rgb(85, 55, 20);

pub fn brand_theme() -> Theme {
    let mut theme = dark_theme();
    theme.name = "psql-freya";
    theme.colors = ColorsSheet {
        primary: Color::from_rgb(255, 140, 0),
        secondary: Color::from_rgb(255, 196, 100),
        tertiary: Color::from_rgb(180, 90, 0),
        background: Color::from_rgb(22, 22, 22),
        surface_primary: Color::from_rgb(35, 35, 35),
        surface_secondary: Color::from_rgb(40, 40, 40),
        surface_tertiary: Color::from_rgb(50, 50, 50),
        border: Color::from_rgb(55, 55, 55),
        text_primary: Color::WHITE,
        text_secondary: Color::from_rgb(180, 180, 180),
        text_placeholder: Color::from_rgb(120, 120, 120),
        error: Color::from_rgb(255, 80, 80),
        ..DARK_COLORS
    };
    theme
}

pub fn divider() -> Rect {
    let color = use_theme().read().colors.border;
    rect()
        .height(Size::px(1.))
        .background(color)
        .margin(Gaps::new(8., 0., 8., 0.))
}

pub fn select(item: SideBarItem, is_selected: bool) -> SideBarItem {
    if is_selected {
        item.background(SELECTED_ITEM_BG)
            .hover_background(SELECTED_ITEM_BG)
    } else {
        item
    }
}
