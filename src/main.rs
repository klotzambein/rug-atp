use std::{cell::RefCell, rc::Rc};

use dear_gui::AppInit;
use glium::Surface;

pub mod entity;
pub mod generation;
pub mod grid;
pub mod tile;
pub mod ui;
pub mod world;

use grid::CanvasGrid;
use ui::UI;
use world::{Pos, World};

fn main() {
    let mut app = AppInit::new();

    let ui = Rc::new(RefCell::new(UI::new(app.imgui.clone())));

    let mut grid = CanvasGrid::new(&app.display, 10, 10);
    let world = Rc::new(RefCell::new(World::new(
        320,
        320,
        500,
        &mut rand::thread_rng(),
    )));

    app.set_canvas_click_handler({
        let world = world.clone();
        let ui = ui.clone();
        Box::new(move |pos| {
            let world = world.borrow_mut();
            println!("User clicked: {:?}", pos);

            // TODO: Convert coordinstes to tile space
            let tile_x = ((pos.x + 5.) / 10.) as isize;
            let tile_y = ((pos.y + 5.) / 10.) as isize;

            if ((tile_x as usize) < world.width)
                && ((tile_y as usize) < world.height)
                && (tile_x >= 0)
                && (tile_y >= 0)
            {
                let pos = Pos(tile_x as i16, tile_y as i16);
                let current_tile_idx = world.idx(pos);
                println!("Corresponding tile: {:?}", current_tile_idx);
                // TODO: Get tile and entity
                let mut ui = ui.borrow_mut();
                ui.selected_tile = Some(pos);
                ui.selected_entity = world.tiles_entity[current_tile_idx];
                // TODO: Select correct entity
                // let current_entity =  world.entity(e_id);
            } else {
                ui.borrow_mut().selected_tile = None;
            }
        })
    });

    for i in 0..100_000 {
        if i % 1000 == 0 {
            dbg!(i);
        }
        world.borrow_mut().step();
    }

    let mut i = 0;
    app.run(move |app, target, last_frame| {
        target.clear_color_srgb(242. / 255., 206. / 255., 223. / 255., 1.);

        app.canvas.draw(target, &grid, &()).unwrap();
        ui.borrow_mut()
            .draw(last_frame, target, &world.borrow_mut());

        if i == 0 {
            world.borrow_mut().step();
        }
        i = (i + 1) % 10;
        world.borrow_mut().update_grid(&app.display, &mut grid);

    });
}
