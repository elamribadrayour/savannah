mod utils;

extern crate nannou;

use nannou::color;
use nannou::app::App;
use nannou::geom::Rect;
use nannou::draw::Draw;
use nannou::frame::Frame;
use nannou::event::Update;

use crate::utils::Population;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    population: Population,
}

fn model(app: &App) -> Model {
    app.new_window()
        .fullscreen()
        .title("Savannah")
        .view(view)
        .build()
        .unwrap();

    Model {
        population: Population::new(&app, &mut rand::thread_rng(), 50),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.population.update(update.since_last.as_secs_f32());
}

fn view_population(window: &Rect, draw: &Draw, model: &Model) {
    model
        .population
        .iter()
        .for_each(|animal| {
            draw.xy(animal.get_xy(&window))
                .texture(&animal.get_icon())
                .wh(animal.get_size())
                .z_degrees(animal.get_angle());
        });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(color::AQUAMARINE);
    view_population(&app.window_rect(), &draw, &model);
    draw.to_frame(app, &frame).unwrap();
}
