use std::{cell::RefCell, rc::Rc, time::Instant};

use dear_gui::event_handling::Imgui;
use glium::Surface;
use imgui::{im_str, Condition, PlotLines, Slider, Ui, Window};

use crate::{
    entity::EntityId,
    statistics::Statistics,
    world::{Pos, World},
};
// We have idx_tile: usize -> world.tiles_type[idx_tile]: TileTexture -> get name through debug: String/str
pub struct UI {
    pub imgui: Rc<RefCell<Imgui>>,
    pub selected_entity: Option<EntityId>,
    pub selected_tile: Option<Pos>,
    stats: Rc<RefCell<Statistics>>,
}

impl UI {
    pub fn new(imgui: Rc<RefCell<Imgui>>, stats: Rc<RefCell<Statistics>>) -> UI {
        UI {
            imgui,
            selected_entity: None,
            selected_tile: None,
            stats,
        }
    }

    pub fn draw(
        &mut self,
        last_frame: Instant,
        target: &mut impl Surface,
        world: &mut World,
        tps: &mut f32,
    ) {
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
        // ui.show_demo_window(&mut true);
        self.window_inspector(&ui, world);
        self.window_market(&ui, world);
        self.window_stepper(&ui, world, tps);
        self.window_stats(&ui, world);

        imgui.platform.prepare_render(&ui, &window);
        let draw_data = ui.render();
        imgui
            .renderer
            .render(target, draw_data)
            .expect("Rendering failed");
    }

    fn window_stepper(&self, ui: &Ui, world: &mut World, tps: &mut f32) {
        Window::new(im_str!("Stepper"))
            .size([200., 150.], Condition::Once)
            .position([350., 100.], Condition::Once)
            .build(ui, || {
                ui.text(&format!(
                    "Day: {}, Time: {}",
                    world.tick / 200,
                    world.time_of_day()
                ));
                ui.checkbox(im_str!("Run"), &mut world.is_running);
                if ui.button(im_str!("Step"), [100., 30.]) {
                    world.step_once(&mut *self.stats.borrow_mut());
                }
                Slider::new(im_str!("TPS"), 0.5..=1000.)
                    .power(5.)
                    .build(ui, tps);
            });
    }

    fn window_inspector(&mut self, ui: &Ui, world: &World) {
        Window::new(im_str!("Inspector"))
            .size([250., 500.], Condition::Once)
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
                if ui.button(im_str!("Select alive"), [100., 30.]) {
                    self.selected_entity = world.random_alive();
                }
            });
    }

    fn window_market(&mut self, ui: &Ui, world: &World) {
        Window::new(im_str!("Market"))
            .size([200., 200.], Condition::Once)
            .build(ui, || {
                let prices = world
                    .market
                    .prices()
                    .map(|p| p.map(|p| p as f32).unwrap_or(f32::NAN));
                ui.text(&format!("Prices: {:#?}", prices));
                ui.text(&format!("M-Prices: {:#?}", world.market.market_price));
                ui.text(&format!("M-Demand: {:#?}", world.market.market_demand));
                for (r, p) in self.stats.borrow().prices.iter() {
                    let values = p.as_ref(); //&p[p.len().max(1000) - 1000..];
                    PlotLines::new(ui, &im_str!("Price {:?}", r), values)
                        .graph_size([0., 50.])
                        .scale_min(0.)
                        .build();
                }
                for (r, v) in self.stats.borrow().volume.iter() {
                    let values = v.as_ref(); //&v[v.len().max(1000) - 1000..];
                    PlotLines::new(ui, &im_str!("Volume {:?}", r), values)
                        .graph_size([0., 50.])
                        .scale_min(0.)
                        .build();
                }
            });
    }

    fn window_stats(&mut self, ui: &Ui, world: &World) {
        Window::new(im_str!("Statistics"))
            .size([500., 200.], Condition::Once)
            .build(ui, || {
                PlotLines::new(
                    ui,
                    &im_str!(
                        "Alive agents:\n{}\nDead agents:\n{}",
                        world.alive_count,
                        world.start_count - world.alive_count
                    ),
                    self.stats.borrow().agent_count.as_ref(),
                )
                .graph_size([0., 50.])
                .build();
            });
    }
}
