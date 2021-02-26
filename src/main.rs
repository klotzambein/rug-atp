use glium::Surface;
use gui_framework::{AppInit, canvas::{CanvasError, CanvasObject, DrawingContext}};

pub struct CanvasGrid {

}

impl CanvasObject for CanvasGrid {
    fn draw<'a>(&self, ctx: &mut DrawingContext<'a>) -> Result<(), CanvasError> {
        ctx.programs
        
        Ok(())
    }
}

fn main() {
    // init_logger();

    let mut app = AppInit::new();

    // let state = init_state(&app);
    // let mut cmd = CommandExecutor::new(state.clone());

    app.set_canvas_click_handler({
        // let state = state.clone();
        Box::new(move |pos| {
            // if let CommandRequest::Point(req) = &mut *state.command_request() {
            //     if req.is_none() {
            //         *req = Some(pos);
            //     }
            // }
        })
    });

    // let root_canvas_obj = RootCanvasObject::new(state.clone());

    // - - - UI init - - -
    // let mut ui = UI::new(app.imgui.clone());

    app.run(move |app, target, last_frame| {
        // state.get_mut().model.room.as_mut().unwrap().step(0.001);
        // state.get_mut().model.redraw_room(&app.display).unwrap();

        target.clear_color_srgb(242. / 255., 206. / 255., 223. / 255., 1.);

        app.canvas.draw(target, &root_canvas_obj, &()).unwrap();

        // ui.draw(last_frame, target, &state, &mut cmd);
        // cmd.poll();

        // if let Some(mut imported) = state.imported() {
        //     imported.update(&app.display);
        // }
        // state.debug().update(&app.display);
    });
}