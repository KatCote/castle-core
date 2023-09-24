/// Test function for soon updates
use std::io::stdout;
use crossterm::{
    execute,
    cursor::{Hide, Show},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle, SetSize, size},};
use crate::CC_VER;
use console::*;
use std::fs::File;
use std::io::prelude::*;

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

    let (cols, rows) = size()?;
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

pub fn create_map_file() -> std::io::Result<()> {
    std::fs::create_dir("../../resourses")?;
    let mut file = File::create("../../resourses/WORLD-BASE-1-1.map")?;
    file.write_all(b"1234567890abcdefghijklmopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*(){}:></")?;
    Ok(())
}

/// Exit from engine screen.
pub fn endscr() -> std::io::Result<()> {

    execute!(stdout(), LeaveAlternateScreen, Show)?;

    Ok(())
}
