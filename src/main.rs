use std::{cell::RefCell, rc::Rc, time::Instant};

use config::Config;
use dear_gui::AppInit;
use glium::Surface;

pub mod config;
pub mod entity;
pub mod generation;
pub mod grid;
pub mod market;
pub mod statistics;
pub mod tile;
pub mod ui;
pub mod world;

use grid::CanvasGrid;
use statistics::Statistics;
use ui::UI;
use world::{Pos, World};

const WORLD_CHUNK_LEN: usize = 30;

fn main() {
    let mut app = AppInit::new();

    let stats = Rc::new(RefCell::new(Statistics::default()));
    let config = Rc::new(Config::default());

    let ui = Rc::new(RefCell::new(UI::new(app.imgui.clone(), stats.clone())));

    let mut grid = CanvasGrid::new(&app.display, WORLD_CHUNK_LEN, WORLD_CHUNK_LEN);
    let world = Rc::new(RefCell::new(World::new(
        WORLD_CHUNK_LEN * 32,
        WORLD_CHUNK_LEN * 32,
        &mut rand::thread_rng(),
        config,
    )));

    app.set_canvas_click_handler({
        let world = world.clone();
        let ui = ui.clone();
        Box::new(move |pos| {
            let world = world.borrow_mut();
            println!("User clicked: {:?}", pos);

            // Convert coordinates to tile space
            let tile_x = ((pos.x + 5.) / 10.).floor() as i16;
            let tile_y = ((pos.y + 5.) / 10.).floor() as i16;

            if ((tile_x as usize) < world.width)
                && ((tile_y as usize) < world.height)
                && (tile_x >= 0)
                && (tile_y >= 0)
            {
                let pos = Pos::new(tile_x, tile_y);
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

    let mut tps = 2.;
    let mut seconds = 0.0;
    app.run(move |app, target, last_frame| {
        if world.borrow_mut().is_running {
            seconds += (Instant::now() - last_frame).as_secs_f32();
        }

        target.clear_color_srgb(242. / 255., 206. / 255., 223. / 255., 1.);

        app.canvas.draw(target, &grid, &()).unwrap();
        ui.borrow_mut()
            .draw(last_frame, target, &mut world.borrow_mut(), &mut tps);

        let start_sim = Instant::now();
        while seconds > 0. {
            seconds -= 1. / tps;
            world.borrow_mut().step(&mut *stats.borrow_mut());
            if (Instant::now() - start_sim).as_secs_f32() > 0.1 {
                tps = 100f32.max(tps * 0.9);
                seconds = 0.0;
                break;
            }
        }

        world.borrow_mut().update_grid(&app.display, &mut grid);
    });
}
