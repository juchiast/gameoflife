use gtk::prelude::*;
use gtk::FileChooserDialog;
use gtk::{Button, DrawingArea, Window, WindowType};
use relm::{DrawHandler, Relm, Update, Widget};

use gol::*;

pub struct MyModel {
    map: Map,
    size: Pos,
    center: Pos,
    scale: i32,
    mouse: Option<Pos>,
    draw_handler: DrawHandler<DrawingArea>,
}

impl MyModel {
    fn new() -> Self {
        MyModel {
            map: Map::acorn(),
            size: pos(500, 300),
            center: pos(0, 0),
            scale: 2,
            mouse: None,
            draw_handler: DrawHandler::new().unwrap(),
        }
    }
}

#[derive(Msg)]
pub enum MyMsg {
    Motion(((f64, f64), gdk::ModifierType)),
    Open,
    Quit,
    Next,
}

pub struct Win {
    window: Window,
    model: MyModel,
}

impl Win {
    fn draw(&mut self, cells: &[Pos], top_left: Pos) {
        let cr = self.model.draw_handler.get_context();
        cr.set_source_rgb(1., 1., 1.);
        cr.paint();
        cr.scale(self.model.scale.into(), self.model.scale.into());
        cr.set_source_rgb(0., 0., 0.);
        for pos in cells {
            cr.rectangle(
                (pos.x - top_left.x).into(),
                (pos.y - top_left.y).into(),
                1.,
                1.,
            );
        }
        cr.fill();
    }
}

impl Update for Win {
    type Model = MyModel;
    type ModelParam = ();
    type Msg = MyMsg;

    fn model(_: &Relm<Self>, _: ()) -> MyModel {
        MyModel::new()
    }

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        relm::interval(relm.stream(), 1000 / 30, || MyMsg::Next);
    }

    fn update(&mut self, event: MyMsg) {
        match event {
            MyMsg::Next => {
                self.model.map.next_generation();
                let top_left = pos(
                    self.model.center.x - self.model.size.x / 2,
                    self.model.center.y - self.model.size.y / 2,
                );
                let cells = self.model.map.get_alive_cells_in(top_left, self.model.size);
                self.draw(&cells, top_left);
            }
            MyMsg::Open => {
                let dialog = FileChooserDialog::new(
                    Some("Open File"),
                    Some(&self.window),
                    gtk::FileChooserAction::Open,
                );
                let cancel = gtk::ResponseType::Cancel.into();
                let accept = gtk::ResponseType::Accept.into();
                dialog.add_button("Cancel", cancel);
                dialog.add_button("Open", accept);
                if let Ok(p) = std::env::current_dir() {
                    dialog.set_current_folder(p);
                } else if let Some(p) = dirs::home_dir() {
                    dialog.set_current_folder(p);
                }
                if accept == dialog.run() {
                    if let Some(path) = dialog.get_filename() {
                        self.model.map = rle::read_file(path).unwrap();
                        self.model.center = pos(0, 0);
                    }
                }
                dialog.close();
            }
            MyMsg::Motion(((x, y), t)) => {
                let p = pos(x as i32, y as i32);
                if (t & gdk::ModifierType::BUTTON1_MASK).bits() != 0 {
                    if let Some(mut old_pos) = self.model.mouse {
                        let new_center = pos(
                            self.model.center.x + (old_pos.x - p.x) / self.model.scale,
                            self.model.center.y + (old_pos.y - p.y) / self.model.scale,
                        );
                        if new_center.x != self.model.center.x {
                            old_pos.x = p.x;
                        }
                        if new_center.y != self.model.center.y {
                            old_pos.y = p.y;
                        }
                        self.model.center = new_center;
                        self.model.mouse = Some(old_pos);
                    } else {
                        self.model.mouse = Some(p);
                    }
                } else {
                    self.model.mouse = None;
                }
            }
            MyMsg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, mut model: MyModel) -> Self {
        let window = Window::new(WindowType::Toplevel);

        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let button_box = gtk::ButtonBox::new(gtk::Orientation::Vertical);

        let open_button = Button::new_with_label("Open");
        let save_button = Button::new_with_label("Save");
        save_button.set_sensitive(false);

        let area = DrawingArea::new();
        area.set_size_request(model.size.x * model.scale, model.size.y * model.scale);
        area.add_events(gdk::EventMask::POINTER_MOTION_MASK.bits() as i32);
        area.add_events(gdk::EventMask::BUTTON_PRESS_MASK.bits() as i32);
        model.draw_handler.init(&area);

        button_box.set_layout(gtk::ButtonBoxStyle::Start);
        button_box.pack_start(&open_button, false, false, 0);
        button_box.pack_start(&save_button, false, false, 0);
        hbox.pack_start(&area, false, false, 0);
        hbox.pack_start(&button_box, false, false, 0);
        window.add(&hbox);
        window.set_title("Game of Life");
        window.show_all();

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(MyMsg::Quit), Inhibit(false))
        );
        connect!(
            relm,
            area,
            connect_motion_notify_event(_, ev),
            return (
                Some(MyMsg::Motion((ev.get_position(), ev.get_state()))),
                Inhibit(false)
            )
        );
        connect!(relm, open_button, connect_clicked(_), MyMsg::Open);

        Win { window, model }
    }
}
