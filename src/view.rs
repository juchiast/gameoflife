use ::*;
use relm::{Relm, RemoteRelm, Widget};
use gtk::prelude::*;
use gtk::{Window, WindowType, DrawingArea, Button};
use gtk::{FileChooserDialog};
use tokio_core::reactor::Interval;
use std::time::Duration;

use map::*;

#[derive(Clone)]
pub struct Model {
    map: Map,
    size: Pos,
    center: Pos,
    scale: i32,
}
impl Model {
    fn new() -> Self {
        Model {
            map: Map::acorn(),
            size: pos(250, 250),
            center: pos(0, 0),
            scale: 2,
        }
    }
}

#[derive(SimpleMsg)]
pub enum Msg {
    Motion(((f64, f64), u32)),
    Save,
    Open,
    Tick,
    Quit,
}

#[derive(Clone)]
pub struct Win {
    hbox: gtk::Box,
    button_box: gtk::ButtonBox,
    open_button: Button,
    save_button: Button,
    area: DrawingArea,
    window: Window,
}

impl Win {
    fn draw(&mut self, cells: Vec<&Pos>, model: &Model, top_left: &Pos) {
        use gdk::prelude::ContextExt;
        let cr = cairo::Context::create_from_window(&self.area.get_window().unwrap());
        cr.set_source_rgb(1., 1., 1.);
        cr.paint();
        cr.scale(model.scale as f64, model.scale as f64);
        cr.set_source_rgb(0., 0., 0.);
        for pos in cells {
            cr.rectangle((pos.x - top_left.x) as f64, (pos.y - top_left.y) as f64, 1., 1.);
        }
        cr.fill();
    }
}

impl Widget for Win {
    type Root = Window;
    type Model = Model;
    type Msg = Msg;

    fn root(&self) -> &Self::Root {
        &self.window
    }

    fn model() -> Self::Model {
        Model::new()
    }

    fn subscriptions(relm: &Relm<Msg>) {
        let stream = Interval::new(Duration::from_millis(30), relm.handle()).unwrap();
        relm.connect_exec_ignore_err(stream, Msg::Tick);
    }

    fn update(&mut self, event: Msg, model: &mut Self::Model) {
        match event {
            Msg::Tick => {
                model.map.next_generation();
                let top_left = pos(model.center.x - model.size.x / 2, model.center.y - model.size.y / 2);
                let cells = model.map.get_alive_cells_in(top_left.clone(), model.size.clone());
                self.draw(cells, model, &top_left);
            },
            Msg::Save => {
                let dialog = FileChooserDialog::new(
                    Some("Save File"),
                    Some(&self.window),
                    gtk::FileChooserAction::Save);
                let cancel: i32 = gtk::ResponseType::Cancel.into();
                let accept: i32 = gtk::ResponseType::Accept.into();
                dialog.add_button("Cancel", cancel);
                dialog.add_button("Save", accept);
                if accept == dialog.run() {
                    if let Some(path) = dialog.get_filename() {
                        model.map.save(path).unwrap();
                    }
                }
                dialog.close();
            },
            Msg::Open => {
                let dialog = FileChooserDialog::new(
                    Some("Open File"),
                    Some(&self.window),
                    gtk::FileChooserAction::Open);
                let cancel: i32 = gtk::ResponseType::Cancel.into();
                let accept: i32 = gtk::ResponseType::Accept.into();
                dialog.add_button("Cancel", cancel);
                dialog.add_button("Open", accept);
                if accept == dialog.run() {
                    if let Some(path) = dialog.get_filename() {
                        model.map = Map::open(path).unwrap();
                    }
                }
                dialog.close();
            },
            Msg::Motion(((x, y), t)) => {
                println!("({}, {}), {}", x, y, t);
            },
            Msg::Quit => gtk::main_quit(),
        }
    }

    fn view(relm: RemoteRelm<Msg>, model: &Self::Model) -> Self {
        let window = Window::new(WindowType::Toplevel);
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let button_box = gtk::ButtonBox::new(gtk::Orientation::Vertical);
        let open_button = Button::new_with_label("Open");
        let save_button = Button::new_with_label("Save");
        let area = DrawingArea::new();
        area.set_size_request(model.size.x * model.scale, model.size.y * model.scale);
        area.set_events(area.get_events() | gdk::POINTER_MOTION_MASK.bits() as i32);
        area.set_events(area.get_events() | gdk::BUTTON_PRESS_MASK.bits() as i32);
        button_box.set_layout(gtk::ButtonBoxStyle::Start);

        button_box.pack_start(&open_button, false, false, 0);
        button_box.pack_start(&save_button, false, false, 0);
        hbox.pack_start(&area, false, false, 0);
        hbox.pack_start(&button_box, false, false, 0);
        window.add(&hbox);
        window.set_title("Game of Life");
        window.show_all();

        connect!(relm, window, connect_delete_event(_, _) (Some(Msg::Quit), Inhibit(false)));
        connect!(relm, area, connect_motion_notify_event(_, ev) (Some(Msg::Motion((ev.get_position(), 0))), Inhibit(false)));
        connect!(relm, save_button, connect_clicked(_), Msg::Save);
        connect!(relm, open_button, connect_clicked(_), Msg::Open);

        Win {
            hbox: hbox,
            button_box: button_box,
            open_button: open_button,
            save_button: save_button,
            area: area,
            window: window,
        }
    }
}
