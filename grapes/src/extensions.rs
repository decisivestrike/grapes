use gtk::Widget;
use gtk::prelude::BoxExt;

pub trait GrapesBoxExt {
    fn append_ref(&self, child: impl AsRef<Widget>);
}

impl GrapesBoxExt for gtk::Box {
    fn append_ref(&self, child: impl AsRef<Widget>) {
        self.append(child.as_ref());
    }
}
