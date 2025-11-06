//! This module provides the hearth theme constant.

use awgen_ui::prelude::*;
use bevy::prelude::*;

/// Creates a new instance of the hearth theme.
pub fn heath_theme(asset_server: &Res<AssetServer>) -> UiTheme {
    UiTheme {
        outer_window: ContainerTheme {
            background_color: Color::srgb_u8(213, 169, 110),
            border_color: Color::srgb_u8(91, 74, 49),
            border_thickness: 4.0,
            border_radius: 8.0,
            padding: 4.0,
        },
        inner_window: ContainerTheme {
            background_color: Color::srgb_u8(217, 173, 114),
            border_color: Color::srgb_u8(193, 147, 91),
            border_thickness: 4.0,
            border_radius: 8.0,
            padding: 4.0,
        },
        button: InteractiveTheme {
            background_color: ColorTheme {
                default: Color::srgb_u8(217, 173, 114),
                hovered: Color::srgb_u8(217, 173, 114).lighter(0.1),
                pressed: Color::srgb_u8(217, 173, 114).darker(0.1),
                disable: Color::srgb_u8(217, 173, 114).with_saturation(0.0),
            },
            border_color: ColorTheme {
                default: Color::srgb_u8(193, 147, 91),
                hovered: Color::srgb_u8(193, 147, 91).lighter(0.1),
                pressed: Color::srgb_u8(193, 147, 91).darker(0.1),
                disable: Color::srgb_u8(193, 147, 91).with_saturation(0.0),
            },
            border_thickness: 2.0,
            border_radius: 4.0,
            padding: 2.0,
        },
        asset: InteractiveTheme {
            background_color: ColorTheme {
                default: Color::srgb_u8(184, 140, 85),
                hovered: Color::srgb_u8(184, 140, 85).lighter(0.1),
                pressed: Color::srgb_u8(184, 140, 85).darker(0.1),
                disable: Color::srgb_u8(184, 140, 85).with_saturation(0.0),
            },
            border_color: ColorTheme {
                default: Color::srgb_u8(113, 79, 46),
                hovered: Color::srgb_u8(113, 79, 46).lighter(0.1),
                pressed: Color::srgb_u8(113, 79, 46).darker(0.1),
                disable: Color::srgb_u8(113, 79, 46).with_saturation(0.0),
            },
            border_thickness: 2.0,
            border_radius: 4.0,
            padding: 2.0,
        },
        icon_color: ColorTheme {
            default: Color::srgb_u8(240, 240, 240),
            hovered: Color::srgb_u8(240, 240, 240).lighter(0.1),
            pressed: Color::srgb_u8(240, 240, 240).darker(0.1),
            disable: Color::srgb_u8(240, 240, 240).with_saturation(0.0),
        },
        icon_size: 32.0,
        text: FontTheme {
            font: asset_server.load("fonts/pixel_arial.ttf"),
            font_size: 16.0,
            color: ColorTheme {
                default: Color::srgb_u8(97, 74, 49),
                hovered: Color::srgb_u8(97, 74, 49).lighter(0.1),
                pressed: Color::srgb_u8(97, 74, 49).darker(0.1),
                disable: Color::srgb_u8(97, 74, 49).with_saturation(0.0),
            },
        },
    }
}

#[allow(dead_code)]
fn main() {}
