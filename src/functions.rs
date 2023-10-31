/// Test function for soon updates
use std::io::stdout;
use crossterm::{
    execute,
    cursor::{SavePosition, RestorePosition, MoveTo},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}
};

/// Print char at current position 
pub fn printch(x: u16, y: u16, symbol: &char) {

    let _ = execute!(
        stdout(),

        SavePosition,
        MoveTo(x, y),
        Print(symbol),
        RestorePosition,
    );
}

/// Print text at current position
pub fn printmsg(x: u16, y: u16, msg: &str) {

    let _ = execute!(
        stdout(),

        SavePosition,
        MoveTo(x, y),
        Print(msg),
        RestorePosition
        );
}

/// Print "Powered by CastleCore"
pub fn print_hello() { 

    set_color(Color::Black, Color::Red);
    Print(" Powered by ");

    set_color(Color::Red, Color::Black);
    Print(" CastleCore ");

    reset_color();
}

/// Print movable "Powered by CastleCore"
pub fn mv_print_hello(x: u16, y: u16) { 

    set_color(Color::Black, Color::White);
    printmsg(x, y, " Powered by ");

    set_color(Color::White, Color::Black);
    printmsg(x + 12, y, " CastleCore ");

    reset_color();
}

/// Set Foreground and Background color
pub fn set_color(fg_color: Color, bg_color: Color) {

    let _ = execute!(   // Need to rewrite " let _ = "
        stdout(),
        SetForegroundColor(fg_color),
        SetBackgroundColor(bg_color)); 
}

/// Reset colors
pub fn reset_color() {

    let _ = execute!(   // Need to rewrite " let _ = "
        stdout(),
        ResetColor);
}
