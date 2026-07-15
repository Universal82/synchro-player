use iced::Length::Fill;
use iced::advanced::graphics::text::cosmic_text::skrifa::raw::collections::IntSet;
use iced::futures::stream::Skip;
use iced::widget::{container, stack};
use iced::{advanced::graphics::futures::backend::default, widget::column};
use iced::{Element, Event, Size, Subscription, event, window};
use iced_video_player::{Video, VideoPlayer};

use std::io::IoSlice;
use std::ops::Not;
use std::time::Duration;
use std::{default::Default, path::PathBuf};

use log::{debug, info, warn, error};

enum PlayState {
    Pause,
    Play
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Pause(iced_video_player::Position),
    Play(iced_video_player::Position),
    SkipTo(iced_video_player::Position),
    SkipForwardBy(Duration),
    SkipBackwardBy(Duration),
    ToggleLoop,
    TogglePlay,

    WindowResize(iced::Size),
}

mod helpers {
    use std::time::Duration;

    // fn position_as_frame(time: Duration, framerate: f64) -> u32 {
    //     (time*framerate). as u32
    // }
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
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) => Some(Message::SkipBackwardBy(Duration::from_secs(5))),
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => Some(Message::SkipForwardBy(Duration::from_secs(5))),
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

            SkipForwardBy(dur) => {
                if let Err(_) = self.video.seek(self.video.position()+dur, true) {
                    error!("Could not seek forward by {dur:?}")
                }
            },

            SkipBackwardBy(dur) => {
                if let Err(_) = self.video.seek(self.video.position()-dur, true) {
                    error!("Could not seek backward by {dur:?}")
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
