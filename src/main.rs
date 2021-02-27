use glium::Surface;
use grid::CanvasGrid;
use gui_framework::AppInit;

pub mod grid;
pub mod tile;

fn main() {
    let mut app = AppInit::new();

    app.set_canvas_click_handler({
        // let state = state.clone();
        Box::new(move |pos| {
            println!("User clicked: {:?}", pos);
        })
    });

    // let mut ui = UI::new(app.imgui.clone());

    let grid: grid::CanvasGrid = CanvasGrid::new(&app.display, 100, 100);

    app.run(move |app, target, _last_frame| {
        target.clear_color_srgb(242. / 255., 206. / 255., 223. / 255., 1.);

        app.canvas.draw(target, &grid, &()).unwrap();

        // ui.draw(last_frame, target, &state, &mut cmd);
    });
}
