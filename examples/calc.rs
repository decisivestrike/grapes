use grapes::{
    State, derived, effect,
    glib::clone,
    gtk::{self, Widget, prelude::*},
    state,
};
use gtk::glib;

pub fn calculator() -> Widget {
    let first_button = gtk::Button::new();
    let second_button = gtk::Button::new();
    let sum_label = gtk::Label::new(None);
    let mul_label = gtk::Label::new(None);

    let first_operand = state(0.0);
    let second_operand = state(0.0);

    let sum = derived(clone!(
        #[strong]
        first_operand,
        #[strong]
        second_operand,
        move || first_operand.get() + second_operand.get()
    ));

    let mul = derived(clone!(
        #[strong]
        first_operand,
        #[strong]
        second_operand,
        move || first_operand.get() * second_operand.get()
    ));

    first_button.connect_clicked(clone!(
        #[strong]
        first_operand,
        move |_| first_operand.update(|v| *v += 1.0)
    ));

    second_button.connect_clicked(clone!(
        #[strong]
        second_operand,
        move |_| second_operand.update(|v| *v += 1.0)
    ));

    effect(clone!(
        #[weak]
        first_button,
        #[strong]
        first_operand,
        move || {
            let label = format!("{:.3}", first_operand);
            first_button.set_label(&label);
        }
    ));

    effect(clone!(
        #[weak]
        second_button,
        #[strong]
        second_operand,
        move || {
            let label = format!("{:.3}", second_operand);
            second_button.set_label(&label);
        }
    ));

    effect(clone!(
        #[weak]
        sum_label,
        #[strong]
        first_operand,
        #[strong]
        second_operand,
        #[strong]
        sum,
        move || {
            let label = format!(
                "{:.3} + {:.3} = {:.3}",
                first_operand, second_operand, sum
            );
            sum_label.set_label(&label);
        }
    ));

    effect(clone!(
        #[weak]
        mul_label,
        #[strong]
        first_operand,
        #[strong]
        second_operand,
        #[strong]
        mul,
        move || {
            let label = format!(
                "{:.3} * {:.3} = {:.3}",
                first_operand, second_operand, mul
            );
            mul_label.set_label(&label);
        }
    ));

    let container = gtk::Box::new(gtk::Orientation::Vertical, 8);
    container.append(&first_button);
    container.append(&second_button);
    container.append(&sum_label);
    container.append(&mul_label);

    container.into()
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

    let widget = calculator();

    window.set_child(Some(&widget));
    window.present();
}
