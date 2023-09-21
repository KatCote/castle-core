/// Test function for soon updates
use std::io::{stdout};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

pub fn print_hello() -> std::io::Result<()> {
    execute!(
        stdout(),

        SetForegroundColor(Color::Black),
        SetBackgroundColor(Color::Red),
        Print(" Powered by "),

        SetForegroundColor(Color::Red),
        SetBackgroundColor(Color::Black),
        Print(" CastleCore "),

        ResetColor
    )?;


    Ok(())
}

