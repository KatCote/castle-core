/// Test function for soon updates
use std::io::stdout;
use crossterm::{
    execute,
    cursor::{Hide, Show, SavePosition, RestorePosition, MoveTo},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle, /*SetSize,*/ size},};
use crate::CC_VER;
use console::*;

/// Print char on current position 
pub fn mvprintch(x: u16, y: u16, msg: &char) {

    execute!(
        stdout(),

        SavePosition,

        MoveTo(x, y),
        Print(msg),

        RestorePosition,
        );
}

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
pub fn initscr() -> std::io::Result<()> {

    let title: &str = &("CastleCore ".to_owned() + &CC_VER);

    execute!(stdout(), EnterAlternateScreen, Hide)?; 
    execute!(stdout(), SetTitle(title))?;

    let _ = print_hello();

    Ok(())
}

/// Screen usability (will be rewrited soon) after initscr.
/// 'q' for quit.
pub fn usescr() -> std::io::Result<()> {

    let (_cols, _rows) = size()?;
    //execute!(stdout(), SetSize(cols, rows)).unwrap();

    let term = Term::stdout();

    Ok(loop {
        let temp_input_var = term.read_char();
        match temp_input_var {
            Ok('q') | Ok('Q') => break,
            Ok(_) => continue,
            Err(_) => todo!()
        }
    })

}

/// Exit from engine screen.
pub fn endscr() -> std::io::Result<()> {

    execute!(stdout(), LeaveAlternateScreen, Show)?;

    Ok(())
}
