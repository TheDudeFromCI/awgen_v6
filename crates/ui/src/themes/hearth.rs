//! This module implements various UI themes.

use bevy::prelude::*;

use crate::theme::{ButtonTheme, ColorTheme, ContainerTheme, FontTheme, TreeViewTheme, UiTheme};

/// Creates a new instance of the `hearth` UI theme.
#[cfg(feature = "editor")]
pub fn hearth_theme(asset_server: &Res<AssetServer>) -> UiTheme {
    use crate::theme::{GlobalTheme, GridPreviewTheme};
    use crate::{DOWN_ARROW_ICON, QUIVER_FONT, RIGHT_ARROW_ICON, SPACER_ICON};

    let font = asset_server.load(QUIVER_FONT);
    let right_arrow_icon = asset_server.load(RIGHT_ARROW_ICON);
    let down_arrow_icon = asset_server.load(DOWN_ARROW_ICON);
    let spacer_icon = asset_server.load(SPACER_ICON);

    UiTheme::from(GlobalTheme {
        outer_window: ContainerTheme {
            background_color: Color::srgb_u8(213, 169, 110).into(),
            border_color: Color::srgb_u8(91, 74, 49).into(),
            border_thickness: 4.0,
            border_radius: 8.0,
            padding: UiRect::all(px(4.0)),
            text: FontTheme {
                font: font.clone(),
                font_size: 32.0,
                color: ColorTheme::Interactive {
                    default: Color::srgb_u8(97, 74, 49),
                    hovered: Color::srgb_u8(97, 74, 49).lighter(0.1),
                    pressed: Color::srgb_u8(97, 74, 49).darker(0.1),
                    disable: Color::srgb_u8(97, 74, 49).with_saturation(0.0),
                    checked: Color::srgb_u8(97, 74, 49).darker(0.1),
                },
            },
            icon_size: 32.0,
            icon_color: Color::srgb_u8(255, 255, 255).into(),
        },
        inner_window: ContainerTheme {
            background_color: Color::srgb_u8(217, 173, 114).into(),
            border_color: Color::srgb_u8(193, 147, 91).into(),
            border_thickness: 4.0,
            border_radius: 8.0,
            padding: UiRect::all(px(4.0)),
            text: FontTheme {
                font: font.clone(),
                font_size: 24.0,
                color: ColorTheme::Interactive {
                    default: Color::srgb_u8(97, 74, 49),
                    hovered: Color::srgb_u8(97, 74, 49).lighter(0.1),
                    pressed: Color::srgb_u8(97, 74, 49).darker(0.1),
                    disable: Color::srgb_u8(97, 74, 49).with_saturation(0.0),
                    checked: Color::srgb_u8(97, 74, 49).darker(0.1),
                },
            },
            icon_size: 24.0,
            icon_color: Color::srgb_u8(255, 255, 255).into(),
        },
        button: ButtonTheme {
            container: ContainerTheme {
                background_color: ColorTheme::Interactive {
                    default: Color::srgb_u8(217, 173, 114),
                    hovered: Color::srgb_u8(217, 173, 114).lighter(0.1),
                    pressed: Color::srgb_u8(217, 173, 114).darker(0.1),
                    disable: Color::srgb_u8(217, 173, 114).with_saturation(0.0),
                    checked: Color::srgb_u8(217, 173, 114).darker(0.1),
                },
                border_color: ColorTheme::Interactive {
                    default: Color::srgb_u8(193, 147, 91),
                    hovered: Color::srgb_u8(193, 147, 91).lighter(0.1),
                    pressed: Color::srgb_u8(193, 147, 91).darker(0.1),
                    disable: Color::srgb_u8(193, 147, 91).with_saturation(0.0),
                    checked: Color::srgb_u8(193, 147, 91).darker(0.1),
                },
                border_thickness: 2.0,
                border_radius: 4.0,
                padding: UiRect::all(px(2.0)),
                text: FontTheme {
                    font: font.clone(),
                    font_size: 16.0,
                    color: ColorTheme::Interactive {
                        default: Color::srgb_u8(97, 74, 49),
                        hovered: Color::srgb_u8(97, 74, 49).lighter(0.1),
                        pressed: Color::srgb_u8(97, 74, 49).darker(0.1),
                        disable: Color::srgb_u8(97, 74, 49).with_saturation(0.0),
                        checked: Color::srgb_u8(97, 74, 49).darker(0.1),
                    },
                },
                icon_size: 16.0,
                icon_color: ColorTheme::Interactive {
                    default: Color::srgb_u8(240, 240, 240),
                    hovered: Color::srgb_u8(240, 240, 240).lighter(0.1),
                    pressed: Color::srgb_u8(240, 240, 240).darker(0.1),
                    disable: Color::srgb_u8(240, 240, 240).with_saturation(0.0),
                    checked: Color::srgb_u8(240, 240, 240).darker(0.1),
                },
            },
        },
        tree_view: TreeViewTheme {
            container: ContainerTheme {
                background_color: Color::srgb_u8(217, 173, 114).into(),
                border_color: Color::srgb_u8(193, 147, 91).into(),
                border_thickness: 4.0,
                border_radius: 8.0,
                padding: UiRect::ZERO,
                text: FontTheme {
                    font: font.clone(),
                    font_size: 24.0,
                    color: ColorTheme::Interactive {
                        default: Color::srgb_u8(97, 74, 49),
                        hovered: Color::srgb_u8(97, 74, 49).lighter(0.1),
                        pressed: Color::srgb_u8(97, 74, 49).darker(0.1),
                        disable: Color::srgb_u8(97, 74, 49).with_saturation(0.0),
                        checked: Color::srgb_u8(97, 74, 49).darker(0.1),
                    },
                },
                icon_size: 16.0,
                icon_color: Color::srgb_u8(255, 255, 255).into(),
            },
            label: ContainerTheme {
                background_color: ColorTheme::Interactive {
                    default: Color::srgb_u8(217, 173, 114),
                    hovered: Color::srgb_u8(217, 173, 114).lighter(0.1),
                    pressed: Color::srgb_u8(217, 173, 114).darker(0.1),
                    disable: Color::srgb_u8(217, 173, 114).with_saturation(0.0),
                    checked: Color::srgb_u8(217, 173, 114).darker(0.1),
                },
                border_color: Color::srgb_u8(255, 255, 255).into(),
                border_thickness: 0.0,
                border_radius: 8.0,
                padding: UiRect::horizontal(px(4.0)),
                text: FontTheme {
                    font: font.clone(),
                    font_size: 16.0,
                    color: ColorTheme::Interactive {
                        default: Color::srgb_u8(97, 74, 49),
                        hovered: Color::srgb_u8(97, 74, 49).lighter(0.1),
                        pressed: Color::srgb_u8(97, 74, 49).darker(0.1),
                        disable: Color::srgb_u8(97, 74, 49).with_saturation(0.0),
                        checked: Color::srgb_u8(97, 74, 49).darker(0.1),
                    },
                },
                icon_size: 16.0,
                icon_color: ColorTheme::Interactive {
                    default: Color::srgb_u8(240, 240, 240),
                    hovered: Color::srgb_u8(240, 240, 240).lighter(0.1),
                    pressed: Color::srgb_u8(240, 240, 240).darker(0.1),
                    disable: Color::srgb_u8(240, 240, 240).with_saturation(0.0),
                    checked: Color::srgb_u8(240, 240, 240).darker(0.1),
                },
            },
            right_arrow_icon,
            down_arrow_icon,
            spacer_icon,
        },
        grid_preview: GridPreviewTheme {
            container: ContainerTheme {
                background_color: Color::srgb_u8(217, 173, 114).into(),
                border_color: Color::srgb_u8(193, 147, 91).into(),
                border_thickness: 4.0,
                border_radius: 8.0,
                padding: UiRect::all(px(4.0)),
                text: FontTheme {
                    font: font.clone(),
                    font_size: 24.0,
                    color: ColorTheme::Interactive {
                        default: Color::srgb_u8(97, 74, 49),
                        hovered: Color::srgb_u8(97, 74, 49).lighter(0.1),
                        pressed: Color::srgb_u8(97, 74, 49).darker(0.1),
                        disable: Color::srgb_u8(97, 74, 49).with_saturation(0.0),
                        checked: Color::srgb_u8(97, 74, 49).darker(0.1),
                    },
                },
                icon_size: 24.0,
                icon_color: Color::srgb_u8(255, 255, 255).into(),
            },
            cell_size: Vec2::new(128.0, 128.0),
            cell_spacing: Vec2::new(10.0, 10.0),
            cell: ContainerTheme {
                background_color: ColorTheme::Interactive {
                    default: Color::srgb_u8(184, 140, 85),
                    hovered: Color::srgb_u8(184, 140, 85).lighter(0.1),
                    pressed: Color::srgb_u8(184, 140, 85).darker(0.1),
                    disable: Color::srgb_u8(184, 140, 85).with_saturation(0.0),
                    checked: Color::srgb_u8(184, 140, 85).darker(0.1),
                },
                border_color: ColorTheme::Interactive {
                    default: Color::srgb_u8(113, 79, 46),
                    hovered: Color::srgb_u8(113, 79, 46).lighter(0.1),
                    pressed: Color::srgb_u8(113, 79, 46).darker(0.1),
                    disable: Color::srgb_u8(113, 79, 46).with_saturation(0.0),
                    checked: Color::srgb_u8(113, 79, 46).darker(0.1),
                },
                border_thickness: 2.0,
                border_radius: 4.0,
                padding: UiRect::all(px(8.0)),
                text: FontTheme {
                    font: font.clone(),
                    font_size: 16.0,
                    color: ColorTheme::Interactive {
                        default: Color::srgb_u8(97, 74, 49),
                        hovered: Color::srgb_u8(97, 74, 49).lighter(0.1),
                        pressed: Color::srgb_u8(97, 74, 49).darker(0.1),
                        disable: Color::srgb_u8(97, 74, 49).with_saturation(0.0),
                        checked: Color::srgb_u8(97, 74, 49).darker(0.1),
                    },
                },
                icon_size: 16.0,
                icon_color: ColorTheme::Interactive {
                    default: Color::srgb_u8(240, 240, 240),
                    hovered: Color::srgb_u8(240, 240, 240).lighter(0.1),
                    pressed: Color::srgb_u8(240, 240, 240).darker(0.1),
                    disable: Color::srgb_u8(240, 240, 240).with_saturation(0.0),
                    checked: Color::srgb_u8(240, 240, 240).darker(0.1),
                },
            },
        },
    })
}
