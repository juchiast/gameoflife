use ::*;
use relm::{Relm, RemoteRelm, Widget};
use gtk::prelude::*;
use gtk::{Window, WindowType, DrawingArea, Button, ScrolledWindow};

use map::*;

#[derive(Clone)]
pub struct Model {}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[derive(Clone)]
pub struct Win {
    hbox: gtk::Box,
    button_box: gtk::ButtonBox,
    pause_button: Button,
    next_button: Button,
    clear_button: Button,
    zoom_in_button: Button,
    zoom_out_button: Button,
    area: DrawingArea,
    scroller: ScrolledWindow,
    window: Window,
}

impl Widget for Win {
    type Container = Window;
    type Model = Model;
    type Msg = Msg;

    fn container(&self) -> &Self::Container {
        &self.window
    }

    fn model() -> Model {
        Model {
        }
    }

    fn update(&mut self, event: Msg, _model: &mut Model) {
        match event {
            Quit => gtk::main_quit()
        }
    }

    fn view(relm: RemoteRelm<Msg>, _model: &Self::Model) -> Self {
        let window = Window::new(WindowType::Toplevel);
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let button_box = gtk::ButtonBox::new(gtk::Orientation::Vertical);
        let pause_button = Button::new_with_label("Start");
        let random_button = Button::new_with_label("Randomize");
        let next_button = Button::new_with_label("Next");
        let clear_button = Button::new_with_label("Clear");
        let zoom_in_button = Button::new_with_label("Zoom in");
        let zoom_out_button = Button::new_with_label("Zoom out");
        let area = DrawingArea::new();
        let scroller = ScrolledWindow::new(None, None);
        scroller.set_size_request(600, 600);
        // disable auto-hide scrollbar
        scroller.set_overlay_scrolling(false);

        connect!(relm, window, connect_delete_event(_, _) (Some(Msg::Quit), Inhibit(false)));

        button_box.set_layout(gtk::ButtonBoxStyle::Start);
        button_box.pack_start(&pause_button, false, false, 0);
        button_box.pack_start(&next_button, false, false, 0);
        button_box.pack_start(&random_button, false, false, 0);
        button_box.pack_start(&clear_button, false, false, 0);
        button_box.pack_start(&zoom_in_button, false, false, 0);
        button_box.pack_start(&zoom_out_button, false, false, 0);
        scroller.add(&area);
        hbox.pack_start(&scroller, false, false, 0);
        hbox.pack_start(&button_box, false, false, 0);
        window.add(&hbox);
        window.set_title("Game of Life");
        window.show_all();
        Win {
            hbox: hbox,
            button_box: button_box,
            pause_button: pause_button,
            next_button: next_button,
            clear_button: clear_button,
            zoom_in_button: zoom_in_button,
            zoom_out_button: zoom_out_button,
            area: area,
            scroller: scroller,
            window: window,
        }
    }
}

