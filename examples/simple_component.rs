use std::time::Duration;

use grapes::{
    Component, GtkCompatible, State,
    gtk::{self, Button, Label, Orientation, Widget, prelude::*},
    reactivity::Reactive,
    service, state,
    tokio::sync::broadcast::Sender,
};

#[derive(GtkCompatible, Clone, Debug, Default, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct Clock {
    time: State<String>,
    #[root]
    label: Label,
}

impl Component for Clock {
    type Message = String;
    type Props = ();

    fn new(_: Self::Props) -> Self {
        let time = state("0".to_string());
        let label = Label::reactive(&time);

        let clock = Self { time, label };

        clock.connect_service::<TimeService>();

        clock
    }

    fn update(&self, message: Self::Message) {
        self.time.set(message);
    }
}

#[service(TimeService)]
async fn time_service(tx: Sender<String>) {
    let mut time = 1;

    loop {
        tx.send(time.to_string()).unwrap();

        time += 1;

        grapes::tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

fn simple_component() -> impl IsA<Widget> {
    let count = state(0);
    let button = Button::reactive(&count);
    let clock = Clock::new(());

    button.connect_clicked(move |_| count.update(|v| *v += 1));

    let vbox = gtk::Box::new(Orientation::Vertical, 3);

    vbox.append(&clock);
    vbox.append(&button);

    vbox
}

fn main() {
    let application = gtk::Application::builder()
        .application_id("grapes.simple_component")
        .build();

    application.connect_activate(create_window);
    application.run();
}

fn create_window(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Simple Component")
        .default_width(350)
        .default_height(270)
        .build();

    let widget = simple_component();

    window.set_child(Some(&widget));
    window.present();
}
