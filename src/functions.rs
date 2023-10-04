/// Test function for soon updates
use std::io::{ stdout, Write };
use crossterm::{
    execute,
    cursor::{Hide, Show, SavePosition, RestorePosition, MoveTo},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle, /*SetSize,*/ size},};
use crate::*;
use console::*;

/// Print char on current position 
pub fn printch(x: u16, y: u16, msg: &char) {

    execute!(
        stdout(),

        SavePosition,
        MoveTo(x, y),
        Print(msg),
        RestorePosition,
    );
}

pub fn printmsg(x: u16, y: u16, msg: &str)
{
    execute!(
        stdout(),

        SavePosition,
        MoveTo(x, y),
        Print(msg),
        RestorePosition
        );
}

/// Border full window (without loot thread now)
pub fn border_full_window() -> std::io::Result<()> {

    //loop{
    let (_cols, _rows) = size()?;

    for i in 0.._cols {
        for j in 0.._rows {

            if i == 0 && j == 0
                { printch(i, j, &LU_CORNER); }
            else if i == 0 && j == (_rows-1)
                { printch(i, j, &LD_CORNER); }
            else if i == (_cols-1) && j == 0
                { printch(i, j, &RU_CORNER); }
            else if i == (_cols-1) && j == (_rows-1)
                { printch(i, j, &RD_CORNER); }
            else if i != 0 && i != (_cols-1) && (j == 0 || j == (_rows-1))
                { printch(i, j, &UD_LINE); }
            else if (i == 0 || i == (_cols-1)) && j != 0 && j != (_rows-1)
                { printch(i, j, &LR_LINE); }
            //else
                //{ printch(i, j, &' '); }

        }
    }//}

    Ok(())
}

/// Print "Powered by CastleCore"
pub fn print_hello(x: u16, y: u16) -> std::io::Result<()> { 

    execute!(
        stdout(),
        SetForegroundColor(Color::Black),
        SetBackgroundColor(Color::White))?;

    printmsg(x, y, " Powered by ");

    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black))?;

    printmsg(x + 12, y, " CastleCore ");

    execute!(stdout(), ResetColor)?;

    Ok(())
}

/// Init screen (base for next engine).
/// Input: true - enable loop, false - only init screen.
pub fn initscr() -> std::io::Result<()> {

    let title: &str = &("CastleCore ".to_owned() + &CC_VER);

    execute!(stdout(), EnterAlternateScreen, Hide)?; 
    execute!(stdout(), SetTitle(title))?;

    //let _ = print_hello(1, 1);

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
