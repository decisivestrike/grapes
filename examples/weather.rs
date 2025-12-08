use core::fmt;
use grapes::{
    Component, Connectable, GtkCompatible, Reactive, broadcast,
    extensions::GrapesBoxExt,
    glib::object::IsA,
    gtk::{
        self, Label, Orientation, Widget,
        gio::prelude::{ApplicationExt, ApplicationExtManual},
        prelude::GtkWindowExt,
    },
    state,
    tokio::time::sleep,
};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct CurrentWeather {
    temperature_2m: f64,
    wind_speed_10m: f64,
    is_day: u8,
}

impl fmt::Display for CurrentWeather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Temperature: {} ℃\nWind Speed: {} km/h\n{}",
            self.temperature_2m,
            self.wind_speed_10m,
            if self.is_day == 1 { "Day" } else { "Night" }
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    current: CurrentWeather,
}

pub async fn get_weather() -> Result<CurrentWeather, Box<dyn std::error::Error>>
{
    let url = "https://api.open-meteo.com/v1/forecast";

    let client = Client::new();

    let resp = client
        .get(url)
        .query(&[
            ("latitude", "40.7128"),
            ("longitude", "-74.0060"),
            ("current", "temperature_2m,wind_speed_10m,is_day"),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<WeatherResponse>()
        .await?;

    Ok(resp.current)
}

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
        weather.connect_service::<WeatherService>();

        let label = Label::statefull(&weather);

        Self { label }
    }
}

broadcast!(WeatherService -> CurrentWeather, async |tx| {
    loop {
        let weather = get_weather().await.unwrap_or_default();
        tx.send(weather).unwrap();
        sleep(Duration::from_secs(600)).await;
    }
});

fn weather() -> impl IsA<Widget> {
    let clock = Weather::new(());

    let vbox = gtk::Box::new(Orientation::Vertical, 0);
    vbox.append_ref(clock);
    vbox
}

fn main() {
    let application = gtk::Application::builder()
        .application_id("grapes.weather")
        .build();

    application.connect_activate(create_window);
    application.run();
}

fn create_window(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Weather")
        .default_width(350)
        .default_height(270)
        .build();

    let widget = weather();

    window.set_child(Some(&widget));
    window.present();
}
