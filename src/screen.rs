use std::io::stdout;
use console::Term;
use crossterm::execute;
use crossterm::{
    cursor::{Hide, Show},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle, /*SetSize,*/ size}
};

/// Init screen (base for next engine).
/// Input: true - enable loop, false - only init screen.
pub fn initscr() {

    let title: &str = &("CastleCore ".to_owned() + &crate::CC_VER);

    let _ = execute!(stdout(), EnterAlternateScreen, Hide); 
    let _ = execute!(stdout(), SetTitle(title));
}

/// Screen usability (will be rewrited soon) after initscr.
/// 'q' for quit.
pub fn usescr() {

    let Ok((_cols, _rows)) = size() else { return; };
    //execute!(stdout(), SetSize(cols, rows)).unwrap();

    let term = Term::stdout();

    loop {
        let temp_input_var = term.read_char();
        match temp_input_var {
            Ok('q') | Ok('Q') => break,
            Ok(_) => continue,
            Err(_) => todo!()
        }
    }

}

/// Exit from engine screen.
pub fn endscr() {

    let _ = execute!(stdout(), LeaveAlternateScreen, Show);
}
