use crossterm::terminal::size;
use crate::functions::*;
use crate::render::*;
use crate::*;

/// Screen render variations
pub enum Screen {
    Empty,
    RenderLayer(render::RenderInterface)
}

/// Border full window (actually without loot thread)
pub fn write_full_window(screen: Screen) -> std::io::Result<()> {

    // Interface layer

    let (_cols, _rows) = size()?;

    if _cols < 10 || _rows < 10 {
        println!("[{} x {}]\nToo small", _cols, _rows);
        return Ok(());
    }

    match screen {
        Screen::Empty => (),
        Screen::RenderLayer(ri) => render_layer(1, 1, _cols-1, _rows-1, ri)
    };

    // Border layer

    for i in 0.._cols {
        for j in 0.._rows {

            if i == 0 && j == 0
                { let _ = printch(i, j, &LU_CORNER); }
            else if i == 0 && j == (_rows-1)
                { let _ = printch(i, j, &LD_CORNER); }
            else if i == (_cols-1) && j == 0
                { let _ = printch(i, j, &RU_CORNER); }
            else if i == (_cols-1) && j == (_rows-1)
                { let _ = printch(i, j, &RD_CORNER); }
            else if i != 0 && i != (_cols-1) && (j == 0 || j == (_rows-1))
                { let _ = printch(i, j, &UD_LINE); }
            else if (i == 0 || i == (_cols-1)) && j != 0 && j != (_rows-1)
                { let _ = printch(i, j, &LR_LINE); }
            //else
                //{ printch(i, j, &' '); }

        }
    }

    // Info layer

    if _cols >= 26 { let _ = mv_print_hello(_cols/2 - 12, _rows); }

    Ok(())
}

/// The border of the window that is split vertically (without loot thread now)
// TODO: make split_ratio changable after init window
pub fn write_vertical_split_window(left_screen: Screen, right_screen: Screen, split_ratio: f32)  -> std::io::Result<()> {
    
    // Interface layer

    let (_cols, _rows) = size()?;

    let bar_pos = ((_cols as f32 - 2.0) * split_ratio) as u16;

    if split_ratio <= 0.0 { return write_full_window(right_screen); }
    else if split_ratio >= 1.0 { return write_full_window(left_screen); } 

    let r1_x = bar_pos - 1; // TODO --+
    let r2_x = bar_pos + 1; //        |
    let r1_y = _rows - 1;   //        |
    let r2_y: u16 = 1;           // <------+

    // +----+-----+
    // |    |r2   |
    // |    |     |
    // |  r1|     |
    // +----+-----+

    if _cols < 10 || _rows < 10 {
        println!("[{} x {}]\nToo small", _cols, _rows);
        return Ok(());
    }

    match left_screen {
        Screen::Empty => (),
        Screen::RenderLayer(left_screen_ri) => render_layer(1, 1, r1_x, r1_y, left_screen_ri)
    };

    match right_screen {
        Screen::Empty => (),
        Screen::RenderLayer(right_screen_ri) => render_layer(r2_x, r2_y, _cols-1, _rows-1, right_screen_ri)
    };

    // Border layer

    for col in 0.._cols {
        for row in 0.._rows {

            if col == 0 && row == 0
                { let _ = printch(col, row, &LU_CORNER); }
            else if col == 0 && row == (_rows-1)
                { let _ = printch(col, row, &LD_CORNER); }
            else if col == (_cols-1) && row == 0
                { let _ = printch(col, row, &RU_CORNER); }
            else if col == (_cols-1) && row == (_rows-1)
                { let _ = printch(col, row, &RD_CORNER); }
            else if col == bar_pos && row == 0
                { let _ = printch(col, row, &UD_T); }
            else if col == bar_pos && row == (_rows-1)
                { let _ = printch(col, row, &DU_T); }
            else if col == bar_pos
                { let _ = printch(col, row, &LR_LINE); }
            else if col != 0 && col != (_cols-1) && (row == 0 || row == (_rows-1))
                { let _ = printch(col, row, &UD_LINE); }
            else if (col == 0 || col == (_cols-1)) && row != 0 && row != (_rows-1)
                { let _ = printch(col, row, &LR_LINE); }
            //else
                //{ printch(i, j, &' '); }

        }
    }

    // Info layer

    if _cols >= 26 { let _ = mv_print_hello(_cols/2 - 12, _rows); }

    Ok(())
}
