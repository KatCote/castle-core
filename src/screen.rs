use std::io::stdout;
use console::Term;
use crossterm::execute;
use crossterm::{
    cursor::{Hide, Show},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle, /*SetSize,*/ size}
};

/// Init screen (base for next engine).
/// Input: true - enable loop, false - only init screen.
pub fn initscr() -> std::io::Result<()> {

    let title: &str = &("CastleCore ".to_owned() + &crate::CC_VER);

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
