use std::{cell::RefCell, rc::Rc, time::Instant};

use dear_gui::event_handling::Imgui;
use glium::Surface;
use imgui::{im_str, Ui, Window};

use crate::{
    entity::{EntityId, EntityType},
    world::World,
};

pub struct UI {
    pub imgui: Rc<RefCell<Imgui>>,
    pub selected_entity: Option<EntityId>,
}

impl UI {
    pub fn new(imgui: Rc<RefCell<Imgui>>) -> UI {
        UI {
            imgui,
            selected_entity: Some(EntityId::new(19)),
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
        Window::new(im_str!("Inspector")).build(ui, || {
            if let Some(e) = self.selected_entity.map(|e| world.entity(e)) {
                match &e.ty {
                    EntityType::Agent(a) => {
                        ui.text(&format!("Pos: {:?}", e.pos));
                        ui.text(&format!("Job id: {}", a.job_id));
                    }
                    _ => unimplemented!(),
                }
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
