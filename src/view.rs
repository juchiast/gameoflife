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

impl Widget for Win {
    type Root = Window;
    type Model = Model;
    type Msg = Msg;

    fn root(&self) -> &Self::Root {
        &self.window
    }

    fn model() -> Self::Model {
        Model {
            map: Map::blom(),
            size: pos(0, 0),
        }
    }

    fn subscriptions(relm: &Relm<Msg>) {
        let stream = Interval::new(Duration::from_millis(30), relm.handle()).unwrap();
        relm.connect_exec_ignore_err(stream, Msg::Tick);
    }

    fn update(&mut self, event: Msg, model: &mut Self::Model) {
        match event {
            Msg::Tick => {
                model.map.next_generation();
                use gdk::prelude::ContextExt;
                let cr = cairo::Context::create_from_window(&self.area.get_window().unwrap());
                cr.set_source_rgb(1., 1., 1.);
                cr.paint();
                cr.scale(2., 2.);
                cr.set_source_rgb(0., 0., 0.);
                for pos in &model.map.alive_cells {
                    cr.rectangle(pos.x as f64+100., pos.y as f64+100., 1., 1.);
                }
                cr.fill();
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

    fn view(relm: RemoteRelm<Msg>, _model: &Self::Model) -> Self {
        let window = Window::new(WindowType::Toplevel);
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let button_box = gtk::ButtonBox::new(gtk::Orientation::Vertical);
        let open_button = Button::new_with_label("Open");
        let save_button = Button::new_with_label("Save");
        let area = DrawingArea::new();
        area.set_size_request(500, 500);
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
