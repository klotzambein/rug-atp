use std::{cell::RefCell, rc::Rc};

use dear_gui::AppInit;
use entity::EntityId;
use glium::Surface;

pub mod entity;
pub mod grid;
pub mod tile;
pub mod ui;
pub mod world;
pub mod generation;

use grid::CanvasGrid;
use ui::UI;
use world::World;

fn main() {
    let mut app = AppInit::new();

    let ui = Rc::new(RefCell::new(UI::new(app.imgui.clone())));

    let mut grid = CanvasGrid::new(&app.display, 10, 10);
    let world = Rc::new(RefCell::new(World::new(320, 320, 1_000)));

    app.set_canvas_click_handler({
        let _world = world.clone();
        let ui = ui.clone();
        Box::new(move |pos| {
            println!("User clicked: {:?}", pos);
            // TODO: Convert coordinstes to tile space
            // TODO: Get tile and entity
            // TODO: Select correct entity
            ui.borrow_mut().selected_entity = Some(EntityId::new(12));
        })
    });

    for i in 0..100_000 {
        if i % 10000 == 0 {
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

        // ui.draw(last_frame, target, &state, &mut cmd);
    });
}
