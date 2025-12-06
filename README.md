# Grapes

Grapes is a library designed for convenient and reactive development with [gtk4-rs](https://github.com/gtk-rs/gtk4-rs) in Rust. Inspired by modern web frameworks, it provides primitives for reactive state management, simplifying GUI programming.

With Grapes, you can easily build reactive interfaces, manage state updates automatically, and write more declarative code.

## Key Features

- Reactive state and change management that automatically updates the UI.
- Macros for background tasks (services) to simplify working with asynchronous operations.
- Convenient components and derive macros to reduce boilerplate code.
- GTK extensions (extension traits) for easier manipulation of common GTK widgets like `gtk::Box`.

## Usage Examples

Simple reactive counter:

```rust
fn counter() -> impl IsA<gtk::Widget> {
    let count = state(0);
    let button = Button::statefull(&count);

    button.connect_clicked(move |_| count.update(|v| *v += 1));

    button
}
```

Background service definition with macro:

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

Reactive component via derive macros:

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

## Why use Grapes?

GTK is powerful but low-level. Grapes brings higher-level abstractions for reactive programming while preserving full control over the underlying api — no black magic, just convenient helpers.
