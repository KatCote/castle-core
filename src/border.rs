use crossterm::terminal::size;
use crate::{*, functions::printch};

/// Border full window (without loot thread now)
pub fn write_full_window() -> std::io::Result<()> {

    //loop{
    let (_cols, _rows) = size()?;

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

    let _ = functions::print_hello(_cols/2 - 12, _rows);

    //let _ = render::render_layer(1, 1, _cols-1, _rows-1, render::RenderInterface::DEFAULT); }

    Ok(())
}