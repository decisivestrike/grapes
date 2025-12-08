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
broadcast!(WeatherService -> CurrentWeather, async |tx| {
    loop {
        let weather = get_weather().await.unwrap_or_default();
        tx.send(weather).unwrap();
        sleep(Duration::from_secs(600)).await;
    }
});
```

Reactive component via derive macros:

```rust
#[derive(GtkCompatible, Clone)]
struct Weather {
    #[root]
    label: Label,
}

impl Component for Weather {
    const NAME: &str = "weather";
    type Props = ();

    fn new(_: ()) -> Self {
        let weather = state(CurrentWeather::default());
        // You can easily use service here
        weather.connect_service::<WeatherService>();
        let label = Label::statefull(&weather);
        Self { label }
    }
}
```

## Why use Grapes?

GTK is powerful but low-level. Grapes brings higher-level abstractions for reactive programming while preserving full control over the underlying api — no black magic, just convenient helpers.
