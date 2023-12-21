use castlecore::*;
use crate::core::*;
use crate::screen::*;
use crate::render::*;
use crate::window::*;

#[test]
fn spr_1_full() {
    initpath();
    initscr("spr_1_full_test ", false);
    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapFull),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Color)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Trigger)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Wall)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Explore)),
        1.0,
        0.5,
        0.5,
        0.5
    );
    endscr();
}

#[test]
fn spr_1_wout() {
    initscr("spr_1_wout_test ", false);
    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapFull),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Color)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Trigger)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Wall)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Explore)),
        0.0,
        0.5,
        0.5,
        0.5
    );
    endscr();
}

#[test]
fn spr_2_full() {
    initscr("spr_2_f_wout ", false);
    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapFull), Screen::RenderLayer(RenderInterface::MapFull), Screen::RenderLayer(RenderInterface::MapFull), Screen::RenderLayer(RenderInterface::MapFull), Screen::RenderLayer(RenderInterface::MapFull),
        0.5,
        1.0,
        0.5,
        0.5
    );
    endscr();
}

#[test]
fn spr_2_wout() {
    initscr("spr_2_wout_test ", false);
    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapFull),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Color)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Trigger)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Wall)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Explore)),
        0.5,
        0.0,
        0.5,
        0.5
    );
    endscr();
}

#[test]
fn spr_3_full() {
    initscr("spr_3_full_test ", false);
    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapFull),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Color)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Trigger)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Wall)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Explore)),
        0.5,
        0.5,
        1.0,
        0.5
    );
    endscr();
}

#[test]
fn spr_3_wout() {
    initscr("spr_3_wout_test ", false);
    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapFull),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Color)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Trigger)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Wall)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Explore)),
        0.5,
        0.5,
        0.0,
        0.5
    );
    endscr();
}

#[test]
fn spr_4_full() {
    initscr("spr_4_full_test ", false);
    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapFull),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Color)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Trigger)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Wall)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Explore)),
        0.5,
        0.5,
        0.5,
        1.0
    );
    endscr();
}

#[test]
fn spr_4_wout() {
    initscr("spr_4_wout_test ", false);
    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapFull),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Color)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Trigger)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Wall)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Explore)),
        0.5,
        0.5,
        0.5,
        0.0
    );
    endscr();
}

#[test]
fn spr_dbl_3f_4f() {
    initscr("spr_dbl_3f_4f_test ", false);
    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapFull),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Color)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Trigger)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Wall)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Explore)),
        0.5,
        0.5,
        1.0,
        1.0
    );
    endscr();
}

#[test]
fn spr_dbl_3w_4w() {
    initscr("spr_dbl_3w_4w_test ", false);
    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapFull),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Color)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Trigger)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Wall)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Explore)),
        0.5,
        0.5,
        0.0,
        0.0
    );
    endscr();
}

#[test]
fn spr_dbl_3f_4w() {
    initscr("spr_dbl_3f_4w_test ", false);
    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapFull),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Color)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Trigger)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Wall)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Explore)),
        0.5,
        0.5,
        1.0,
        0.0
    );
    endscr();
}

#[test]
fn spr_dbl_3w_4f() {
    initscr("spr_dbl_3w_4f_test ", false);
    write_default_game_window(
        Screen::RenderLayer(RenderInterface::MapFull),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Color)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Trigger)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Wall)),
        Screen::RenderLayer(RenderInterface::MapLayer(MapLayer::Explore)),
        0.5,
        0.5,
        0.0,
        1.0
    );
    endscr();
}