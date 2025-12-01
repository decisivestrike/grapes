# Grapes

A wrapper over gtk4-rs

**G**TK **r**e**a**ctive **p**rimitiv**es**

```rust
fn counter() -> impl IsA<gtk::Widget> {
    let count = state(0);
    let button = Button::statefull(&count);

    button.connect_clicked(move |_| count.update(|v| *v += 1));

    button
}
```
