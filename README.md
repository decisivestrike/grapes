# Grapes

A wrapper over gtk4-rs

**G**TK **r**e**a**ctive **p**rimitiv**es**

Reactivity inspired by web frameworks:

```rust
fn counter() -> impl IsA<gtk::Widget> {
    let count = state(0);
    let button = Button::statefull(&count);

    button.connect_clicked(move |_| count.update(|v| *v += 1));

    button
}
```

Create background tasks and subscribe on them:

```rust
service!(TickService -> i32, async |tx| {
    let mut count = 1;

    loop {
        tx.send(count).unwrap();
        count += 1;
        sleep(Duration::from_secs(1)).await;
    }
});
```

Components created to reduce the amount of boilerplate code:

```rust
#[derive(GtkCompatible, Clone)]
struct Ticker {
    #[root]
    label: Label,
    #[state]
    count: State<i32>,
}

impl Component for Ticker {
    const NAME: &str = "ticker";
    type Props = ();

    fn new(_: ()) -> Self {
        let count = state(0);

        // You can easily use service here
        count.connect_service::<TickService>();

        let label = Label::statefull(&count);

        Self { label, count }
    }
}
```
