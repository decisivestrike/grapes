use grapes::{
    Component, GtkCompatible,
    extensions::GrapesBoxExt,
    glib::object::IsA,
    gtk::{
        self, Label, Orientation, Widget,
        gio::prelude::{ApplicationExt, ApplicationExtManual},
        prelude::GtkWindowExt,
    },
    service,
};
use std::time::Duration;

#[derive(GtkCompatible, Clone)]
struct Ticker {
    #[root]
    label: Label,
}

impl Component for Ticker {
    type Message = String;
    type Props = ();

    fn new(_: ()) -> Self {
        let label = gtk::Label::new(None);
        let clock = Self { label };
        clock.connect_service::<TimeService>();
        clock
    }

    fn update(&self, time: String) {
        self.label.set_label(&time);
    }
}

service!(TimeService -> String, async |tx| {
    let mut count = 1;

    loop {
        tx.send(count.to_string()).unwrap();

        count += 1;

        grapes::tokio::time::sleep(Duration::from_secs(1)).await;
    }
});

fn ticker() -> impl IsA<Widget> {
    let clock = Ticker::new(());

    let vbox = gtk::Box::new(Orientation::Vertical, 0);
    vbox.append_ref(clock);
    vbox
}

fn main() {
    let application = gtk::Application::builder()
        .application_id("grapes.ticker")
        .build();

    application.connect_activate(create_window);
    application.run();
}

fn create_window(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Ticker")
        .default_width(350)
        .default_height(270)
        .build();

    let widget = ticker();

    window.set_child(Some(&widget));
    window.present();
}
