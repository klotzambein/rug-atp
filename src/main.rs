use glium::texture::Texture2d;
use glium::{Surface, VertexBuffer};
use gui_framework::{
    canvas::{CanvasError, CanvasObject, DrawingContext},
    graphics::primitives::{Sprite, Vf2},
    texture::load_png_texture,
    AppInit,
};

pub struct CanvasGrid {
    vertex_buffer: VertexBuffer<Sprite>,
    texture: Texture2d,
}

impl CanvasObject for CanvasGrid {
    fn draw<'a>(&self, ctx: &mut DrawingContext<'a>) -> Result<(), CanvasError> {
        ctx.programs.draw_sprites(
            ctx.target,
            self.vertex_buffer.slice(..).unwrap(),
            &self.texture,
            ctx.model_transform,
            ctx.view_transform,
        )?;

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

    let grid = CanvasGrid {
        vertex_buffer: VertexBuffer::new(
            &app.display,
            &(0..64).map(|i| Sprite {
                vertex: Vf2::new((i % 8) as f32 * 10.0, (i / 8) as f32 * 10.0),
                size: Vf2::new(10., 10.),
                texture_index: i,
            }).collect::<Vec<_>>(),
        )
        .unwrap(),
        texture: load_png_texture(&app.display),
    };

    app.run(move |app, target, last_frame| {
        // state.get_mut().model.room.as_mut().unwrap().step(0.001);
        // state.get_mut().model.redraw_room(&app.display).unwrap();

        target.clear_color_srgb(242. / 255., 206. / 255., 223. / 255., 1.);

        app.canvas.draw(target, &grid, &()).unwrap();

        // ui.draw(last_frame, target, &state, &mut cmd);
        // cmd.poll();

        // if let Some(mut imported) = state.imported() {
        //     imported.update(&app.display);
        // }
        // state.debug().update(&app.display);
    });
}
