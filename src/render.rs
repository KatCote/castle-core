use crate::functions::printch;

/// Enum for use in match in functions as param
pub enum RenderInterface {
    DEFAULT,
    MapAll,
    MapBase,
    MapColor,
    MapTrigger,
    MapWall,
    InvPage1, // Inventory page 1
    InvPage2, // Inventory page 2
    PlayerParams,
    InfoWorld,
    InfoItem
}

pub fn render_layer(x1: u16, y1: u16, x2: u16, y2: u16, layer: RenderInterface) {

    for i in x1..x2 {
        for j in y1..y2 {
            match layer {
                RenderInterface::DEFAULT => {
                    let _ = printch(i, j, &'.');
                },
                RenderInterface::MapAll => {
                    let _ = printch(i, j, &'0');
                },
                RenderInterface::MapBase => {
                    let _ = printch(i, j, &'1');
                },
                RenderInterface::MapColor => {
                    let _ = printch(i, j, &'2');
                },
                RenderInterface::MapTrigger => {
                    let _ = printch(i, j, &'3');
                },
                RenderInterface::MapWall => {
                    let _ = printch(i, j, &'4');
                },
                RenderInterface::InvPage1 => {
                    let _ = printch(i, j, &'5');
                },
                RenderInterface::InvPage2 => {
                    let _ = printch(i, j, &'6');
                },
                RenderInterface::PlayerParams => {
                    let _ = printch(i, j, &'7');
                },
                RenderInterface::InfoWorld => {
                    let _ = printch(i, j, &'8');
                },
                RenderInterface::InfoItem => {
                    let _ = printch(i, j, &'9');
                }
            }
        }
    }
}