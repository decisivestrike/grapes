use grapes::{
    derived, effect,
    glib::{self, clone},
    gtk::{self, Button, Label, Widget, prelude::*},
    reactivity::Reactive,
    state,
};

fn calculator() -> impl IsA<Widget> {
    let first_operand = state(0);
    let second_operand = state(0);

    let sum = derived(clone!(
        #[strong]
        first_operand,
        #[strong]
        second_operand,
        move || &first_operand + &second_operand
    ));

    let mul = derived(clone!(
        #[strong]
        first_operand,
        #[strong]
        second_operand,
        move || &first_operand * &second_operand
    ));

    let first_button = Button::reactive(&first_operand);
    let second_button = Button::reactive(&second_operand);

    let sum_label = Label::new(Some(&"0"));
    let mul_label = Label::new(Some(&"0"));

    first_button.connect_clicked(clone!(
        #[strong]
        first_operand,
        move |_| first_operand.update(|v| *v += 1)
    ));

    second_button.connect_clicked(clone!(
        #[strong]
        second_operand,
        move |_| second_operand.update(|v| *v += 1)
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
            let label = format!("{first_operand} + {second_operand} = {sum}");
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
            let label = format!("{first_operand} * {second_operand} = {mul}");
            mul_label.set_label(&label);
        }
    ));

    let container = gtk::Box::new(gtk::Orientation::Vertical, 4);

    container.append(&first_button);
    container.append(&second_button);
    container.append(&sum_label);
    container.append(&mul_label);

    container
}

fn main() {
    let application = gtk::Application::builder()
        .application_id("grapes.calculator")
        .build();

    application.connect_activate(create_window);
    application.run();
}

fn create_window(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Calculator")
        .default_width(350)
        .default_height(270)
        .build();

    let widget = calculator();

    window.set_child(Some(&widget));
    window.present();
}
