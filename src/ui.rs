use std::{cell::RefCell, rc::Rc, time::Instant};

use dear_gui::event_handling::Imgui;
use glium::Surface;
use imgui::{im_str, Condition, Ui, Window};

use crate::{
    entity::EntityId,
    world::{Pos, World},
};
// We have idx_tile: usize -> world.tiles_type[idx_tile]: TileTexture -> get name through debug: String/str
pub struct UI {
    pub imgui: Rc<RefCell<Imgui>>,
    pub selected_entity: Option<EntityId>,
    pub selected_tile: Option<Pos>,
}

impl UI {
    pub fn new(imgui: Rc<RefCell<Imgui>>) -> UI {
        UI {
            imgui,
            selected_entity: None,
            selected_tile: None,
        }
    }

    pub fn draw(&mut self, last_frame: Instant, target: &mut impl Surface, world: &World) {
        let imgui = self.imgui.clone();
        let mut imgui = imgui.borrow_mut();
        let imgui = &mut *imgui;

        let io = imgui.context.io_mut();
        let gl_win = imgui.display.gl_window();
        let window = gl_win.window();
        imgui
            .platform
            .prepare_frame(io, &window)
            .expect("Failed to start frame");
        io.update_delta_time(last_frame);

        let ui = imgui.context.frame();

        // Here goes the code that describes the GUI
        ui.show_demo_window(&mut true);
        self.window_inspector(&ui, world);

        imgui.platform.prepare_render(&ui, &window);
        let draw_data = ui.render();
        imgui
            .renderer
            .render(target, draw_data)
            .expect("Rendering failed");
    }

    fn window_inspector(&self, ui: &Ui, world: &World) {
        Window::new(im_str!("Inspector"))
            .size([200., 200.], Condition::Once)
            .build(ui, || {
                if let Some(e_id) = self.selected_entity {
                    let e = world.entity(e_id);
                    ui.text(&format!("Position: {:?}", e.pos));
                    match &e.ty {
                        // EntityType::Agent(a) => {
                        //     ui.text(&format!("Job: {:?}", a.job));
                        // }
                        e => ui.text(&format!("Selected: {:#?}", e)),
                    }
                } else if let Some(p) = self.selected_tile {
                    let current_tile_idx = world.idx(p);

                    ui.text(&format!("Position: {:?}", p));
                    ui.text(&format!(
                        "Tile Type: {:?}",
                        world.tiles_type[current_tile_idx]
                    ));
                } else {
                    ui.text("Nothing selected");
                }
            });
    }
}

// pub struct UILayers;
// impl UIComponent<CommandExecutor> for UILayers {
//     type Model = Vec<(String, bool)>;
//     fn draw(&mut self, ui: &Ui, model: &Self::Model, cmd: &mut CommandExecutor) {
//         Window::new(im_str!("Layers")).build(ui, || {
//             for (i, (l, vis)) in model.iter().enumerate() {
//                 let id = ui.push_id(i as i32);
//                 let mut vis = *vis;
//                 ui.text(&l);
//                 ui.same_line(120.);
//                 if ui.checkbox_flags(im_str!(""), &mut vis, true) {
//                     cmd.execute(Command::NamedWithArgs {
//                         name: "set_layer_visibility".to_owned(),
//                         arguments: vec![Value::Bool(vis), Value::String(l.clone())],
//                     });
//                 }
//                 ui.spacing();
//                 id.pop(ui)
//             }
//         });
//     }
// }

// pub struct UIMenuBar;
// impl UIComponent<CommandExecutor> for UIMenuBar {
//     type Model = ();
//     fn draw(&mut self, ui: &Ui, _: &Self::Model, cmd: &mut CommandExecutor) {
//         ui.main_menu_bar(|| {
//             ui.menu(im_str!("Actions"), true, || {
//                 if MenuItem::new(im_str!("detect_room")).build(ui) {
//                     cmd.execute(Command::Named {
//                         name: "detect_room".to_owned(),
//                     })
//                 }
//             });
//         });
//     }
// }

// pub struct UIStatus;
// impl UIComponent<CommandExecutor> for UIStatus {
//     type Model = CommandRequest;

//     fn draw(&mut self, ui: &Ui, model: &CommandRequest, _: &mut CommandExecutor) {
//         match model {
//             CommandRequest::Point(_) => {
//                 ui.text("Please select a point.");
//             }
//             CommandRequest::None => {
//                 ui.text("All good.");
//             }
//         }
//     }
// }

// pub struct UITester(pub &'static str);
// impl UIComponent<CommandExecutor> for UITester {
//     type Model = ();
//     fn draw(&mut self, ui: &Ui, _: &Self::Model, _: &mut CommandExecutor) {
//         ui.text(self.0)
//     }
// }
