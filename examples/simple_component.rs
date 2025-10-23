use grapes::{
    GtkCompatible,
    gtk::{Button, Label, prelude::*},
    reactivity::Reactive,
    state,
};

#[derive(GtkCompatible, Clone, Debug, Default, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct Clock {
    #[root]
    label: Label,
}

fn simple_component() -> impl IsA<::grapes::gtk::Widget> {
    let count = state(0);
    let button = Button::reactive(&count);
    let clock = Clock::default();

    button.connect_clicked(move |_| count.update(|v| *v += 1));

    let vbox = ::grapes::gtk::Box::new(::grapes::gtk::Orientation::Vertical, 3);
    vbox.append(&clock);
    vbox.append(&button);

    vbox
}

fn main() {
    let application = ::grapes::gtk::Application::builder()
        .application_id("grapes.simple_component")
        .build();

    application.connect_activate(create_window);
    application.run();
}

fn create_window(application: &::grapes::gtk::Application) {
    let window = ::grapes::gtk::ApplicationWindow::builder()
        .application(application)
        .title("Simple Component")
        .default_width(350)
        .default_height(270)
        .build();

    let widget = simple_component();

    window.set_child(Some(&widget));
    window.present();
}
