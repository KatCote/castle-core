use crossterm::terminal::size;
use crate::functions::*;
use crate::render::*;
use crate::*;

pub enum Screen {
    Empty,
    RenderLayer(render::RenderInterface)
}

/// Border full window (without loot thread now)
pub fn write_full_window(screen: Screen) -> std::io::Result<()> {

    // Interface layer

    let (_cols, _rows) = size()?;

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

    let _ = mv_print_hello(_cols/2 - 12, _rows);

    Ok(())
}
