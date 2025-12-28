use grapes::{
    State,
    gtk::{self, Button, prelude::*},
    reactive::Reactive,
    state,
};

fn counter() -> impl IsA<gtk::Widget> {
    let count = state(0);
    let button = Button::statefull(&count);

    button.connect_clicked(move |_| count.update(|v| *v += 1));

    button
}

fn main() {
    let application = gtk::Application::builder()
        .application_id("grapes.counter")
        .build();

    application.connect_activate(create_window);
    application.run();
}

fn create_window(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Counter")
        .default_width(350)
        .default_height(270)
        .build();

    let widget = counter();

    window.set_child(Some(&widget));
    window.present();
}
