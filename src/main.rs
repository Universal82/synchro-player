use iced::Length::Fill;
use iced::advanced::graphics::text::cosmic_text::skrifa::raw::collections::IntSet;
use iced::futures::stream::Skip;
use iced::widget::{container, stack};
use iced::{advanced::graphics::futures::backend::default, widget::column};
use iced::{Element, Event, Size, Subscription, event, window};
use iced_video_player::{Position, Video, VideoPlayer};

use std::io::IoSlice;
use std::ops::Not;
use std::str::FromStr;
use std::time::Duration;
use std::{default::Default, path::PathBuf};

use log::{debug, info, warn, error};

mod helpers {
    use std::time::Duration;

    pub fn safe_duration_sub(d1: Duration, d2: Duration) -> Duration {
        if d2 > d1 {
            return Duration::from_secs(0);
        } else {
            return d1 - d2;
        }
    }
}

enum PlayState {
    Pause,
    Play
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Pause(iced_video_player::Position),
    Play(iced_video_player::Position),
    SkipTo(iced_video_player::Position),
    SkipForwardBy(iced_video_player::Position),
    SkipBackwardBy(iced_video_player::Position),
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
        let out = Self::default();
        info!("Current video framerate: {}", out.video.framerate());
        out
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
                        "," => Some(Message::SkipBackwardBy(Position::Frame(1))), // watching the "<" key, but it needs to be checking the "," for programming reasons
                        "." => Some(Message::SkipForwardBy(Position::Frame(1))), // watching the ">" key, but it needs to be checking the "." for programming reasons
                        _ => None
                    },
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => Some(Message::TogglePlay),
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) => Some(Message::SkipBackwardBy(Position::Time(Duration::from_secs(5)))),
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => Some(Message::SkipForwardBy(Position::Time(Duration::from_secs(5)))),
                    // iced::keyboard::Key::Named(iced::keyboard::key::Named::) => Some(Message::SkipBackwardBy(Position::Time(Duration::from_secs(5)))),
                    // iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => Some(Message::SkipForwardBy(Position::Time(Duration::from_secs(5)))),
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
            Pause(iced_video_player::Position::Frame(frame)) => {
                self.video.set_paused(true);
                if let Err(_) = self.video.seek(iced_video_player::Position::Frame(frame), true) {
                    error!("Could not seek to frame {frame}")
                }
            },

            Play(iced_video_player::Position::Frame(frame)) => {
                self.video.set_paused(false);
                if let Err(_) = self.video.seek(iced_video_player::Position::Frame(frame), true) {
                    error!("Could not seek to frame {frame}")
                }
            },

            SkipTo(iced_video_player::Position::Frame(frame)) => {
                if let Err(_) = self.video.seek(iced_video_player::Position::Frame(frame), true) {
                    error!("Could not seek to frame {frame}")
                }
            },

            SkipForwardBy(Position::Time(dur)) => {
                if let Err(_) = self.video.seek(self.video.position()+dur, true) {
                    error!("Could not seek forward by {dur:?}s")
                }
            },

            SkipBackwardBy(Position::Time(dur)) => {
                if let Err(_) = self.video.seek(helpers::safe_duration_sub(self.video.position(), dur), true) {
                    error!("Could not seek backward by {dur:?}s")
                }
            },

            SkipForwardBy(Position::Frame(f)) => {
                if let Err(_) = self.video.seek(self.video.position() + Duration::from_millis((1000.0/self.video.framerate()) as u64), true) {
                    error!("Could not seek forward by {f} frame")
                }
            },

            SkipBackwardBy(Position::Frame(f)) => {
                if let Err(_) = self.video.seek(helpers::safe_duration_sub(self.video.position(), Duration::from_millis((1000.0/self.video.framerate()) as u64)), true) {
                    error!("Could not seek backward by {f} frame")
                }
            },

            ToggleLoop => self.video.set_looping(self.video.looping().not()),
            TogglePlay => self.video.set_paused(self.video.paused().not()),

            WindowResize(s) => self.window_size = s,

            _ => {}
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
