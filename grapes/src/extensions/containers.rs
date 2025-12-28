use gtk::{
    Widget,
    prelude::{BoxExt, WidgetExt},
};
use std::iter::successors;

pub trait GrapesBoxExt {
    fn append_ref(&self, child: impl AsRef<Widget>);

    fn children(&self) -> impl Iterator<Item = Widget>;
}

impl GrapesBoxExt for gtk::Box {
    fn append_ref(&self, child: impl AsRef<Widget>) {
        self.append(child.as_ref());
    }

    fn children(&self) -> impl Iterator<Item = Widget> {
        successors(self.first_child(), |child| child.next_sibling())
    }
}
