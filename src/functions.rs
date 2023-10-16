/// Test function for soon updates
use std::io::stdout;
use crossterm::{
    execute,
    cursor::{SavePosition, RestorePosition, MoveTo},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}
};

/// Print char at current position 
pub fn printch(x: u16, y: u16, symbol: &char) -> std::io::Result<()> {

    execute!(
        stdout(),

        SavePosition,
        MoveTo(x, y),
        Print(symbol),
        RestorePosition,
    )?;

    Ok(())
}

/// Print text at current position
pub fn printmsg(x: u16, y: u16, msg: &str) -> std::io::Result<()> {

    execute!(
        stdout(),

        SavePosition,
        MoveTo(x, y),
        Print(msg),
        RestorePosition
        )?;

        Ok(())
}

/// Print "Powered by CastleCore"
pub fn print_hello() -> std::io::Result<()> { 

    set_color(Color::Black, Color::Red)?;
    Print(" Powered by ");

    set_color(Color::Red, Color::Black)?;
    Print(" CastleCore ");

    reset_color()?;

    Ok(())
}

/// Print movable "Powered by CastleCore"
pub fn mv_print_hello(x: u16, y: u16) -> std::io::Result<()> { 

    set_color(Color::Black, Color::White)?;
    printmsg(x, y, " Powered by ")?;

    set_color(Color::White, Color::Black)?;
    printmsg(x + 12, y, " CastleCore ")?;

    reset_color()?;

    Ok(())
}

/// Set Foreground and Background color
pub fn set_color(fg_color: Color, bg_color: Color) -> std::io::Result<()> {

    execute!(
        stdout(),
        SetForegroundColor(fg_color),
        SetBackgroundColor(bg_color))?; 

    Ok(())
}

/// Reset colors
pub fn reset_color() -> std::io::Result<()> {

    execute!(
        stdout(),
        ResetColor)?;

    Ok(())
}
