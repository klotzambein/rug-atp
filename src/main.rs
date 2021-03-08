use dear_gui::AppInit;
use glium::Surface;

pub mod agent;
pub mod grid;
// pub mod resources;
pub mod tile;
pub mod world;

use grid::CanvasGrid;
use world::World;

fn main() {
    let mut app = AppInit::new();

    app.set_canvas_click_handler({
        // let state = state.clone();
        Box::new(move |pos| {
            println!("User clicked: {:?}", pos);
        })
    });

    // let mut ui = UI::new(app.imgui.clone());

    let mut grid = CanvasGrid::new(&app.display, 10, 10);
    let mut world = World::new(320, 320, 500);

    app.run(move |app, target, _last_frame| {
        target.clear_color_srgb(242. / 255., 206. / 255., 223. / 255., 1.);

        app.canvas.draw(target, &grid, &()).unwrap();

        world.step();
        world.update_grid(&app.display, &mut grid);

        // ui.draw(last_frame, target, &state, &mut cmd);
    });
}
