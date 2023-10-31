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
pub fn write_full_window(screen: Screen) {

    // Interface layer

    let Ok((_cols, _rows)) = size() else { return; };

    if _cols < 10 || _rows < 10 {
        println!("[{} x {}]\nToo small", _cols, _rows);
        return;
    }

    match screen {
        Screen::Empty => (),
        Screen::RenderLayer(ri) => render_layer(1, 1, _cols-1, _rows-1, ri)
    };

    // Border layer

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
    }

    // Info layer

    if _cols >= 26 { mv_print_hello(_cols/2 - 12, _rows); }

}

/// The border of the window that is split vertically (without loot thread now)
// TODO: make split_ratio changable after init window
pub fn write_vertical_split_window(left_screen: Screen, right_screen: Screen, split_ratio: f32) {
    
    // Interface layer

    let Ok((_cols, _rows)) = size() else { return; };

    let bar_pos = ((_cols as f32 - 2.0) * split_ratio) as u16;

    if split_ratio <= 0.0 { write_full_window(right_screen); return; }
    else if split_ratio >= 1.0 {  write_full_window(left_screen); return; } 

    let r1_x = bar_pos - 1;
    let r2_x = bar_pos + 1;
    let r1_y = _rows - 1;
    let r2_y: u16 = 1;

    // +----+-----+
    // |    |r2   |
    // |    |     |
    // |  r1|     |
    // +----+-----+

    if _cols < 10 || _rows < 10 {
        println!("[{} x {}]\nToo small", _cols, _rows);
        return;
    }

    match left_screen {
        Screen::Empty => (),
        Screen::RenderLayer(left_screen_rl) => render_layer(
            1,
            1,
            r1_x,
            r1_y,
            left_screen_rl
        )
    };

    match right_screen {
        Screen::Empty => (),
        Screen::RenderLayer(right_screen_rl) => render_layer(
            r2_x,
            r2_y,
            _cols-1,
            _rows-1,
            right_screen_rl
        )
    };

    // Border layer

    for col in 0.._cols {
        for row in 0.._rows {

            if col == 0 && row == 0
                { printch(col, row, &LU_CORNER); }
            else if col == 0 && row == (_rows-1)
                { printch(col, row, &LD_CORNER); }
            else if col == (_cols-1) && row == 0
                { printch(col, row, &RU_CORNER); }
            else if col == (_cols-1) && row == (_rows-1)
                { printch(col, row, &RD_CORNER); }
            else if col == bar_pos && row == 0
                { printch(col, row, &UD_T); }
            else if col == bar_pos && row == (_rows-1)
                { printch(col, row, &DU_T); }
            else if col == bar_pos
                { printch(col, row, &LR_LINE); }
            else if col != 0 && col != (_cols-1) && (row == 0 || row == (_rows-1))
                { printch(col, row, &UD_LINE); }
            else if (col == 0 || col == (_cols-1)) && row != 0 && row != (_rows-1)
                { printch(col, row, &LR_LINE); }
            //else
                //{ printch(i, j, &' '); }

        }
    }

    // Info layer

    if _cols >= 26 { mv_print_hello(_cols/2 - 12, _rows); }

}

pub fn write_default_game_window(
    screen_1: Screen,
    screen_2: Screen,
    screen_3: Screen,
    screen_4: Screen,
    screen_5: Screen,
    split_ratio_1: f32,
    split_ratio_2: f32,
    split_ratio_3: f32,
    split_ratio_4: f32) {
    
    // Define layer

    let Ok((_cols, _rows)) = size() else { return; };


    let bar_sr1_x: u16 = ((_cols as f32 - 2.0) * split_ratio_1) as u16;
    
    let bar_sr2_y: u16 = ((_rows as f32 - 2.0) * split_ratio_2) as u16;

    let bar_sr3_x: u16 = ((_cols as f32 - 2.0)/2.0 * split_ratio_3) as u16;

    let bar_sr4_x: u16 = (((_cols as f32 - 2.0)/2.0) + ((_cols as f32 - 2.0)/2.0 * split_ratio_4)) as u16;


    let s11_x: u16 = if split_ratio_1 == 0.0 {0} else {1};
    let s11_y: u16 = if split_ratio_1 == 0.0 {0} else {1};

    let s12_x: u16 = if split_ratio_1 == 0.0 {0} else if split_ratio_1 == 1.0 {bar_sr1_x + 1} else {bar_sr1_x};
    let s12_y: u16 = bar_sr2_y;


    let s21_x: u16 = if split_ratio_1 == 0.0 {1} else if split_ratio_1 == 1.0 {bar_sr1_x + 1} else {bar_sr1_x + 1};
    let s21_y: u16 = 1;

    let s22_x: u16 = if split_ratio_1 == 1.0 {_cols} else {_cols - 1};
    let s22_y: u16 = bar_sr2_y;


    let s31_x: u16 = 1;
    let s31_y: u16 = bar_sr2_y + 1;

    let s32_x: u16 = bar_sr3_x;
    let s32_y: u16 = _rows - 1;


    let s41_x: u16 = bar_sr3_x + 1;
    let s41_y: u16 = bar_sr2_y + 1;

    let s42_x: u16 = bar_sr4_x;
    let s42_y: u16 = _rows - 1;

    
    let s51_x: u16 = bar_sr4_x + 1;
    let s51_y: u16 = bar_sr2_y + 1;

    let s52_x: u16 = _cols - 1;
    let s52_y: u16 = _rows - 1;

    //                                      (SR1)
    //                                        |
    // +--------------------------------------+------------------+
    // |s11                                   |s21               |
    // |                                      |                  |
    // |                                      |                  |
    // |                                      |                  |
    // |                                      |                  |
    // |              Screen 1                |     Screen 2     |
    // |                                      |                  |
    // |                                      |                  |
    // |                                      |                  |
    // |                                      |                  |
    // |                                      |                  |
    // |                            .      s12|               s22|
    // +----------+-----------------.-------+-+------------------+--(SR2)  
    // |s31       |s41              .       |s51                 |
    // |          |                 .       |                    |
    // | Screen 3 |         Screen 4.       |      Screen 5      |
    // |          |                 .       |                    |
    // |       s32|                 .    s42|                 s52|
    // +----------+-----------------.-------+--------------------+  
    //            |                 .       |
    //          (SR3)                     (SR4)

    // Interface layer

    if _cols < 20 || _rows < 10 {
        println!("[{} x {}]\nToo small", _cols, _rows);
        return;
    }

    if split_ratio_1 != 0.0 { match screen_1 {
        Screen::Empty => (),
        Screen::RenderLayer(screen_1_rl) => render_layer(
            s11_x,
            s11_y,
            s12_x,
            s12_y,
            screen_1_rl
        )
    }; }

    if split_ratio_1 != 1.0 { match screen_2 {
        Screen::Empty => (),
        Screen::RenderLayer(screen_2_rl) => render_layer(
            s21_x,
            s21_y,
            s22_x,
            s22_y,
            screen_2_rl
        )
    }; }

    match screen_3 {
        Screen::Empty => (),
        Screen::RenderLayer(screen_3_rl) => render_layer(
            s31_x,
            s31_y,
            s32_x,
            s32_y,
            screen_3_rl
        )
    };

    match screen_4 {
        Screen::Empty => (),
        Screen::RenderLayer(screen_4_rl) => render_layer(
            s41_x,
            s41_y,
            s42_x,
            s42_y,
            screen_4_rl
        )
    };

    match screen_5 {
        Screen::Empty => (),
        Screen::RenderLayer(screen_5_rl) => render_layer(
            s51_x,
            s51_y,
            s52_x,
            s52_y,
            screen_5_rl
        )
    };

    // Border layer
    
    /*for col in 0.._cols {
        for row in 0.._rows {

            if col == 0 && row == 0
                { printch(col, row, &LU_CORNER); }
            else if col == 0 && row == (_rows-1)
                { printch(col, row, &LD_CORNER); }
            else if col == (_cols-1) && row == 0
                { printch(col, row, &RU_CORNER); }
            else if col == (_cols-1) && row == (_rows-1)
                { printch(col, row, &RD_CORNER); }
            else if col != 0 && col != (_cols-1) && (row == 0 || row == (_rows-1))
                { printch(col, row, &UD_LINE); }
            else if (col == 0 || col == (_cols-1)) && row != 0 && row != (_rows-1)
                { printch(col, row, &LR_LINE); }

            if split_ratio_1 != 0.0 && split_ratio_1 != 1.0 && row != 0 && row <= (bar_sr2_y-1)
                { printch(bar_sr1_x, row, &LR_LINE); }
            else if split_ratio_1 != 0.0 && split_ratio_1 != 1.0 && row == 0
                { printch(bar_sr1_x, row, &UD_T); }
            else if split_ratio_1 != 0.0 && split_ratio_1 != 1.0 && row <= bar_sr2_y
                { printch(bar_sr1_x, row, &DU_T); }

            if bar_sr2_y != 0 && bar_sr2_y != _rows && col != 0 && col != (_cols-1)
                { printch(col, bar_sr2_y, &UD_LINE); }
            else if bar_sr2_y != 0 && bar_sr2_y != _rows && col == 0
                { printch(col, bar_sr2_y, &LR_T); }
            else if bar_sr2_y != 0 && bar_sr2_y != _rows && col == (_cols-1)
                { printch(col, bar_sr2_y, &RL_T); }

            if bar_sr3_x != 0 && bar_sr3_x != _cols && row != (_rows-1) && row >= (bar_sr2_y+1)
                { printch(bar_sr3_x, row, &LR_LINE); }
            else if bar_sr3_x != 0 && bar_sr3_x != _cols && row == (bar_sr2_y)
                { printch(bar_sr3_x, row, &UD_T); }
            else if bar_sr3_x != 0 && bar_sr3_x != _cols && row == (_rows-1)
                { printch(bar_sr3_x, row, &DU_T); }

            if bar_sr4_x != 0 && bar_sr4_x != _cols && row != (_rows-1) && row >= (bar_sr2_y+1)
                { printch(bar_sr4_x, row, &LR_LINE); }
            else if bar_sr4_x != 0 && bar_sr4_x != _cols && row == (bar_sr2_y)
                { printch(bar_sr4_x, row, &UD_T); }
            else if bar_sr4_x != 0 && bar_sr4_x != _cols && row == (_rows-1)
                { printch(bar_sr4_x, row, &DU_T); }

            if bar_sr1_x == bar_sr4_x && bar_sr1_x != 0 && bar_sr1_x != _cols
                { printch(bar_sr1_x, bar_sr2_y, &CROSS); }

        }
    }*/
    
    // Info layer

    //if _cols >= 26 { mv_print_hello(_cols/2 - 12, _rows); }

}
