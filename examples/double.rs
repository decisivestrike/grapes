use grapes::{
    derived,
    glib::clone,
    gtk::{self, Button, Label, Widget, prelude::*},
    reactivity::Reactive,
    state,
};

fn double() -> impl IsA<Widget> {
    let count = state(0);
    let doubled = derived(clone!(
        #[strong]
        count,
        move || *count.get() * 2
    ));

    let button = Button::statefull(&doubled);
    button.connect_clicked(clone!(
        #[strong]
        count,
        move |_| count.update(|v| *v += 1)
    ));

    let label = Label::derived(move || format!("{count} doubled is {doubled}"));

    let container = gtk::Box::new(gtk::Orientation::Vertical, 4);

    container.append(&button);
    container.append(&label);

    container
}

fn main() {
    let application = gtk::Application::builder()
        .application_id("grapes.double")
        .build();

    application.connect_activate(create_window);
    application.run();
}

fn create_window(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Double")
        .default_width(350)
        .default_height(270)
        .build();

    let widget = double();

    window.set_child(Some(&widget));
    window.present();
}
