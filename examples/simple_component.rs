use std::ops::Deref;

use grapes::{
    Component, GtkCompatible,
    gtk::{Button, Widget, prelude::*},
    reactivity::Reactive,
    state,
};

#[derive(Clone, Debug, Default, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct Clock {
    label: ::grapes::gtk::Label,
}

impl Deref for Clock {
    type Target = ::grapes::gtk::Widget;

    fn deref(&self) -> &Self::Target {
        self.as_widget_ref()
    }
}

impl GtkCompatible for Clock {
    type Root = ::grapes::gtk::Label;

    fn root(&self) -> Self::Root {
        self.label.clone()
    }

    fn as_widget_ref(&self) -> &::grapes::gtk::Widget {
        self.label.upcast_ref()
    }
}

impl Component for Clock {
    type Message = String;

    fn update(&self, _message: Self::Message) {}
}

impl std::convert::Into<::grapes::gtk::Widget> for Clock {
    fn into(self) -> ::grapes::gtk::Widget {
        grapes::component::GtkCompatible::root(&self).into()
    }
}

impl ::std::convert::AsRef<::grapes::gtk::Widget> for Clock {
    fn as_ref(&self) -> &::grapes::gtk::Widget {
        self.as_widget_ref()
    }
}

impl ::std::borrow::Borrow<::grapes::gtk::Widget> for Clock {
    fn borrow(&self) -> &::grapes::gtk::Widget {
        self.as_widget_ref()
    }
}

impl ::grapes::glib::translate::IntoGlibPtr<*mut ::grapes::gtk::ffi::GtkLabel> for Clock {
    fn into_glib_ptr(self) -> *mut ::grapes::gtk::ffi::GtkLabel {
        self.label.into_glib_ptr()
    }
}

impl<'a> ::grapes::glib::translate::ToGlibPtr<'a, *mut ::grapes::gtk::ffi::GtkLabel> for Clock {
    type Storage = &'a Clock;

    fn to_glib_none(
        &'a self,
    ) -> ::grapes::gtk::glib::translate::Stash<'a, *mut ::grapes::gtk::ffi::GtkLabel, Self> {
        use grapes::glib::translate::IntoGlibPtr;
        let ptr = self.label.clone().into_glib_ptr();
        ::grapes::gtk::glib::translate::Stash(ptr, self)
    }
}

impl ::grapes::glib::value::ToValueOptional for Clock {
    fn to_value_optional(maybe_component: Option<&Self>) -> ::grapes::glib::Value {
        match maybe_component {
            Some(component) => component.to_value(),
            None => ::grapes::glib::Value::from(None::<&::grapes::gtk::Label>),
        }
    }
}

impl ::grapes::glib::value::ToValue for Clock {
    fn to_value(&self) -> ::grapes::glib::Value {
        self.label.to_value()
    }

    fn value_type(&self) -> ::grapes::glib::Type {
        self.label.value_type()
    }
}

unsafe impl<'a> ::grapes::glib::value::FromValue<'a> for Clock {
    type Checker = <::grapes::gtk::Label as ::grapes::glib::value::FromValue<'a>>::Checker;

    unsafe fn from_value(_: &'a ::grapes::glib::Value) -> Self {
        unimplemented!()
        // unsafe {
        //     let label =
        //         <::grapes::gtk::Label as ::grapes::glib::value::FromValue>::from_value(value);
        //     Clock {
        //         label: label.clone(),
        //     }
        // }
    }
}

impl ::grapes::glib::value::ValueType for Clock {
    type Type = <::grapes::gtk::Label as ValueType>::Type;
}

impl ::grapes::glib::types::StaticType for Clock {
    fn static_type() -> ::grapes::glib::Type {
        ::grapes::gtk::Label::static_type()
    }
}

impl ::grapes::glib::translate::UnsafeFrom<::grapes::glib::object::ObjectRef> for Clock {
    unsafe fn unsafe_from(_: ::grapes::glib::object::ObjectRef) -> Self {
        unimplemented!()
        // unsafe {
        //     let label = ::grapes::gtk::Label::unsafe_from(obj);
        //     Clock { label }
        // }
    }
}

impl ::std::convert::From<Clock> for ::grapes::glib::object::ObjectRef {
    fn from(value: Clock) -> Self {
        value
            .label
            .upcast::<::grapes::glib::object::Object>()
            .into()
    }
}

unsafe impl ::grapes::glib::object::ObjectType for Clock {
    type GlibType = <::grapes::gtk::Label as ObjectType>::GlibType;

    type GlibClassType = <::grapes::gtk::Label as ObjectType>::GlibClassType;

    fn as_object_ref(&self) -> &::grapes::glib::object::ObjectRef {
        self.label.as_object_ref()
    }

    fn as_ptr(&self) -> *mut Self::GlibType {
        self.label.as_ptr()
    }

    unsafe fn from_glib_ptr_borrow(_: &*mut Self::GlibType) -> &Self {
        panic!("from_glib_ptr_borrow is not supported for components");
    }
}

unsafe impl ::grapes::glib::object::IsA<Widget> for Clock {}

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
