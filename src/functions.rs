/// Test function for soon updates
use std::io::{self, stdout};
use crossterm::{
    execute,
    cursor::Hide,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle},};
use crate::CC_VER;

/// Print "Powered by CastleCore"
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

/// Init screen (base for next engine).
/// Input: true - enable loop, false - only init screen.
pub fn initscr(enable_loop: bool) -> std::io::Result<()> {

    let title: &str = &("CastleCore ".to_owned() + &CC_VER);

    execute!(io::stdout(), EnterAlternateScreen, Hide)?; 
    execute!(io::stdout(), SetTitle(title))?;

    let _ = print_hello();

    if enable_loop == true { loop {} }

    Ok(())
}

/// Exit from engine screen.
pub fn endscr() -> std::io::Result<()> {

    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
