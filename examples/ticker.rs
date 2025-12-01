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
    tokio::time::sleep,
};
use std::time::Duration;

#[derive(GtkCompatible, Clone)]
struct Ticker {
    #[root]
    label: Label,
}

impl Component for Ticker {
    const NAME: &str = "ticker";

    type Message = i32;
    type Props = ();

    fn new(_: ()) -> Self {
        let label = gtk::Label::new(None);
        let ticker = Self { label };
        ticker.connect_service::<TickService>();
        ticker
    }

    fn update(&self, time: i32) {
        self.label.set_label(&time.to_string());
    }
}

service!(TickService -> i32, async |tx| {
    let mut count = 1;

    loop {
        tx.send(count).unwrap();
        count += 1;
        sleep(Duration::from_secs(1)).await;
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
