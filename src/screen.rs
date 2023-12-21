use std::io::stdout;
use console::Term;
use crossterm::execute;
use crossterm::{
    cursor::{Hide, Show},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle, /*SetSize,*/ size}
};

use crate::functions::printch;
use crate::render::render_full_map;

/// Init screen (base for next engine).
/// Input: true - enable loop, false - only init screen.
pub fn initscr(title: &str, ver: bool) {

    let set_title: &str = &(title.to_owned() + if ver == true {&crate::CC_VER} else {""});

    let _ = execute!(stdout(), EnterAlternateScreen, Hide); 
    let _ = execute!(stdout(), SetTitle(set_title));
}

/// Screen usability (will be rewrited soon) after initscr.
/// 'q' for quit.
pub fn usescr() {

    let Ok((_cols, _rows)) = size() else { return; };
    //execute!(stdout(), SetSize(cols, rows)).unwrap();

    let term = Term::stdout();
    let mut temp_x = 90;
    let mut temp_y = 60;

    loop {
        let temp_input_var = term.read_char();
        match temp_input_var {
            Ok('q') | Ok('Q') => break,
            Ok('w') => { temp_y = temp_y - 1; render_full_map(1, 1, 111, 48, temp_x, temp_y); printch(56, 23, &'◓'); }, 
            Ok('a') => { temp_x = temp_x - 1; render_full_map(1, 1, 111, 48, temp_x, temp_y); printch(56, 23, &'◐'); }, 
            Ok('s') => { temp_y = temp_y + 1; render_full_map(1, 1, 111, 48, temp_x, temp_y); printch(56, 23, &'◒'); }, 
            Ok('d') => { temp_x = temp_x + 1; render_full_map(1, 1, 111, 48, temp_x, temp_y); printch(56, 23, &'◑');  },
            Ok(_) => continue,
            Err(_) => todo!()
        }
    }

}

/// Exit from engine screen.
pub fn endscr() {

    let _ = execute!(stdout(), LeaveAlternateScreen, Show);
}
