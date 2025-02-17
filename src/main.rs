use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, GtkWindowExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

struct AppModel {
    counter: u8,
    error: bool
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = ();

    view! {
        gtk::Window {
            set_title: Some("Relm 4 demo app"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,
                gtk::Button {
                    set_label: "+",
                    connect_clicked => AppMsg::Increment
                },

                gtk::Button {
                    set_label: "-",
                    connect_clicked => AppMsg::Decrement
                },
                if model.error {
                    gtk::Label {
                        set_label: "error: exceeds limit of u8 variable",
                        set_margin_all: 5,
                    }
                } else {
                    gtk::Label {
                        #[watch]
                        set_label: &format!("Count: {}", model.counter),
                        set_margin_all: 5,
                    }
                }
            }
        }
    }

    /// Initialize the UI and model.
    fn init(
        _params: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel { counter: 0, error: false };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Increment => {
                if self.counter == 255 {
                    self.error = true;
                    return
                }
                self.counter += 1;
                self.error = false;
            }
            AppMsg::Decrement => {
                if self.counter == 0 {
                    self.error = true;
                    return
                }
                self.counter -= 1;
                self.error = false;
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple_manual");
    app.run::<AppModel>(());
}
