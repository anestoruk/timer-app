#![windows_subsystem = "windows"]

use crate::Message::EventHappened;
use iced::event::{self};
use iced::time::{self, Duration};
use iced::widget::text::Catalog;
use iced::widget::{button, column, container, text, text_input};
use iced::window::{icon, Settings};
use iced::{border, mouse, Element, Event, Fill, Size, Subscription, Theme};
use std::str::FromStr;
use winrt_notification::{Sound, Toast};

pub fn main() -> iced::Result {
    let bytes = include_bytes!("../resources/icon.png");
    let icon = icon::from_file_data(bytes, None).unwrap();
    iced::application("Timer App", TimerApp::update, TimerApp::view)
        .window(Settings {
            icon: Some(icon),
            decorations: true,
            ..Settings::default()
        })
        .subscription(TimerApp::subscription)
        .resizable(false)
        .window_size(Size { width: 500.0, height: 500.0 })
        .theme(|_| Theme::Nightfly)
        .run()
}

struct TimerApp {
    counter: i32,
    delay: i32,
    delay_text: String,
    text: String,
    log_message: String,
}

impl Default for TimerApp {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    UpdateDelay(String),
    UpdateText(String),
    EventHappened,
    Notification,
    SetLogMessage(String),
}

impl TimerApp {
    fn new() -> Self {
        Self {
            counter: 0,
            delay: 3600,
            delay_text: String::from("3600"),
            text: String::from("Break time! 🦆"),
            log_message: String::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.counter += 1;

                let delay = if self.delay > 0 { self.delay } else { 600 };
                if self.counter % delay == 0 {
                    Toast::new(Toast::POWERSHELL_APP_ID)
                        .title(&self.text)
                        .sound(Some(Sound::Reminder))
                        .duration(winrt_notification::Duration::Short)
                        .show()
                        .expect("unable to toast")
                }
            }

            Message::UpdateDelay(delay_text) => {
                self.delay_text = delay_text;
                match i32::from_str(self.delay_text.as_str()) {
                    Ok(i32_delay) => {
                        self.delay = i32_delay;
                        self.update(Message::SetLogMessage(String::new()));
                    }
                    Err(e) => {
                        self.delay_text = String::from("");
                        self.delay = 0;
                        self.update(Message::SetLogMessage(format!("{}", e)));
                    }
                }
            }

            Message::UpdateText(text) => {
                self.text = text;
            }

            Message::SetLogMessage(text) => {
                self.log_message = text;
            }

            _ => {}
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column![

            text(format!("Running for {} seconds", self.counter)),

            text(format!("Next break in {} seconds", self.delay - (self.counter % self.delay))),

            button(text("Increment"))
                .width(200)
                .on_press(Message::Increment),

            text_input("Delay in seconds", &self.delay_text)
                .width(200)
                .on_input(Message::UpdateDelay),

            text_input("Notification text", &self.text)
                .width(200)
                .on_input(Message::UpdateText),

            text(&self.log_message)
                .size(10)
                .width(200)
                .height(100),
            ].spacing(20))
            .style(|theme: &Theme| {
                container::Style::default().border(
                    border::color(theme.extended_palette().primary.weak.color).width(5)
                )
            })
            .padding(10)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let events_sub = event::listen()
            .map(|e| match e {
                Event::Mouse(event) => {
                    match event {
                        mouse::Event::ButtonPressed(button) => {
                            let log = format!("Button {:?} pressed", button);
                            Message::SetLogMessage(log)
                        }
                        _ => EventHappened
                    }
                }
                _ => EventHappened
            });

        let time_sub = time::every(Duration::from_secs(1))
            .map(|_| Message::Increment);

        Subscription::batch(vec![events_sub, time_sub])
    }
}
