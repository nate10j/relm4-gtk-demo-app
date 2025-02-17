use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

struct AppModel {
    counter: u8,
    error: bool,
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

struct AppWidgets {
    label: gtk::Label,
}

impl SimpleComponent for AppModel {

    /// The type of the messages that this component can receive.
    type Input = AppMsg;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = u8;
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Relm 4 Demo App")
            .default_width(300)
            .default_height(100)
            .build()
    }

    /// Initialize the UI and model.
    fn init(
        counter: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { counter, error: false };

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let inc_button = gtk::Button::with_label("Increment");
        let dec_button = gtk::Button::with_label("Decrement");

        let label = gtk::Label::new(Some(&format!("Counter: {}", model.counter)));
        label.set_margin_all(5);

        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        vbox.append(&inc_button);
        vbox.append(&dec_button);
        vbox.append(&label);

        inc_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::Increment);
            }
        ));

        dec_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(AppMsg::Decrement);
            }
        ));

        let widgets = AppWidgets { label };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Increment => {
                if self.counter == u8::MAX {
                    self.error = true;
                    return;
                }
                self.error = false;
                self.counter += 1;
            }
            AppMsg::Decrement => {
                if self.counter == u8::MIN {
                    self.error = true;
                    return;
                }
                self.error = false;
                self.counter -= 1;
            }
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        widgets
            .label
            .set_label(&format!("Counter: {}", self.counter));

        if self.error {
            widgets.label.set_label("error cannot go out of bounds of uint 8");
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple_manual");
    app.run::<AppModel>(0);
}
