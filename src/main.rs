use iced::Length::Fill;
use iced::advanced::graphics::text::cosmic_text::skrifa::raw::collections::IntSet;
use iced::futures::stream::Skip;
use iced::widget::{container, stack};
use iced::{advanced::graphics::futures::backend::default, widget::column};
use iced::{Element, Event, Size, Subscription, event, window};
use iced_video_player::{Video, VideoPlayer};

use std::io::IoSlice;
use std::ops::Not;
use std::{default::Default, path::PathBuf};

use log::{debug, info, warn, error};

enum PlayState {
    Pause,
    Play
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Pause(Option<iced_video_player::Position>),
    Play(Option<iced_video_player::Position>),
    SkipTo(iced_video_player::Position),
    SkipBy(iced_video_player::Position),
    ToggleLoop,
    TogglePlay,

    WindowResize(iced::Size),
}

struct App {
    window_size: iced::Size,
    video: Video,
}

impl Default for App {
    fn default() -> Self {
        info!("Opening file passed to program.");
        Self { window_size: Default::default(), video: Video::new(
                &url::Url::from_file_path(
                    std::env::args().skip(1).collect::<Vec<String>>()[0].as_str()
                ).unwrap()
            ).unwrap()
        }
    }
}

impl App {
    fn new() -> Self {
        Self::default()
    }

    fn run(size: iced::Size) {
        info!("Running app with size: {size:?}");
        iced::application(Self::new, Self::update, Self::view)
            .window_size(size)
            .resizable(true)
            .subscription(Self::subscription)
            .run().unwrap();
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event,status,window|{
            // This is a nightmare to work with, ngl
            match event {
                iced::event::Event::Window(iced::window::Event::Resized(size)) => Some(Message::WindowResize(size)),
                iced::event::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, ..}) => match key {
                    iced::keyboard::key::Key::Character(c) => match c.as_str() {
                        "l" => Some(Message::ToggleLoop),
                        _ => None
                    },
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => Some(Message::TogglePlay),
                    _ => None
                },
                _ => None
            }
        })
    }

    fn update(&mut self, message: Message) {
        info!("Update with message {message:?}");
        use Message::*;
        match message {
            Pause(Some(t)) => {
            },
            Pause(None) => {},

            Play(Some(t)) => {},
            Play(None) => {},

            SkipTo(t) => {},

            SkipBy(a) => {},

            ToggleLoop => self.video.set_looping(self.video.looping().not()),
            TogglePlay => self.video.set_paused(self.video.paused().not()),

            WindowResize(s) => self.window_size = s,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        info!("View tick!");
        stack![
            container(
                VideoPlayer::new(&self.video)
                    .width(Fill)
                    .height(Fill)
            ).width(Fill).height(Fill),
        ].width(Fill).height(Fill).into()
    }
}

fn main() {
    // Prep logging to only log this crate
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("synchro_player", log::LevelFilter::Info)
        .init();

    info!("Logging initialized! Hello, world!");
    App::run(Size::new(200.0, 200.0));
}
