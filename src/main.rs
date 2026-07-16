mod style;

use iced::Length::{Fill, Shrink};
use iced::advanced::graphics::futures::backend::default;
use iced::advanced::graphics::text::cosmic_text::skrifa::raw::collections::IntSet;
use iced::futures::stream::Skip;
use iced::widget::text::Format;
use iced::widget::{Space, button, column, container, image, mouse_area, row, stack, svg, text};
use iced::{Alignment, Color, Element, Event, Size, Subscription, alignment, event, time, window};
use iced_video_player::{Position, Video, VideoPlayer};
use std::io::IoSlice;
use std::ops::Not;
use std::str::FromStr;
use std::time::Duration;
use std::{default::Default, path::PathBuf};

use log::{debug, error, info, warn};

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

fn FormatTimestamp(t: u64) -> String {
    if t < 60 * 60 {
        format!("{:0>2}:{:0>2}", (t / 60) % 60, t % 60)
    } else {
        format!("{}:{:0>2}:{:0>2}", t / 3600, (t / 60) % 60, t % 60)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Message {
    Pause(iced_video_player::Position),
    Play(iced_video_player::Position),
    SkipTo(iced_video_player::Position),
    SkipForwardBy(iced_video_player::Position),
    SkipBackwardBy(iced_video_player::Position),
    ToggleLoop,
    TogglePlay,
    Tick,
    WindowResize(iced::Size),
}

struct App {
    window_size: iced::Size,
    video: Video,
    video_paused: bool,
    server: Option<std::net::Ipv4Addr>,
}

impl Default for App {
    fn default() -> Self {
        info!("Opening file passed to program.");
        Self {
            window_size: Default::default(),
            video: Video::new(
                &url::Url::from_file_path(
                    std::env::args().skip(1).collect::<Vec<String>>()[0].as_str(),
                )
                .unwrap(),
            )
            .unwrap(),
            server: None,
            video_paused: false,
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
            .window(window::Settings {
                size,
                min_size: Some(Size {
                    width: 200.0,
                    height: 200.0,
                }),
                ..Default::default()
            })
            .resizable(true)
            .subscription(Self::subscription)
            .run()
            .unwrap();
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            event::listen_with(|event, status, window| {
                // This is a nightmare to work with, ngl
                match event {
                    iced::event::Event::Window(iced::window::Event::Resized(size)) => {
                        Some(Message::WindowResize(size))
                    }
                    iced::event::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                        key, ..
                    }) => match key {
                        iced::keyboard::key::Key::Character(c) => match c.as_str() {
                            "l" => Some(Message::ToggleLoop),
                            "," => Some(Message::SkipBackwardBy(Position::Frame(1))),
                            "." => Some(Message::SkipForwardBy(Position::Frame(1))),
                            _ => None,
                        },
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => {
                            Some(Message::TogglePlay)
                        }
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) => Some(
                            Message::SkipBackwardBy(Position::Time(Duration::from_secs(5))),
                        ),
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => Some(
                            Message::SkipForwardBy(Position::Time(Duration::from_secs(5))),
                        ),
                        _ => None,
                    },
                    _ => None,
                }
            }),
            iced::time::every(Duration::from_millis(50)).map(|_| Message::Tick),
        ])
    }

    fn update(&mut self, message: Message) {
        if message != Message::Tick {
            info!("Update with message {message:?}");
        }
        use Message::*;
        match message {
            SkipTo(pos) => {
                // This is where the bulk of the networking stuff will happen so I have to do less boilerplate code on the other messages
                if let Err(_) = self.video.seek(pos, true) {
                    error!("Could not seek to frame {pos:?}")
                }
            }

            Pause(pos) => {
                self.video_paused = true;
                self.video.set_paused(true);
                if let Err(_) = self.video.seek(pos, true) {
                    error!("Could not seek to {pos:?}")
                }
            }

            Play(pos) => {
                self.video_paused = false;
                self.video.set_paused(false);
                if let Err(_) = self.video.seek(pos, true) {
                    error!("Could not seek to {pos:?}")
                }
            }

            SkipForwardBy(Position::Time(dur)) => {
                // Hook into Message::SkipTo
                self.update(Message::SkipTo(Position::Time(self.video.position() + dur)));
            }

            SkipBackwardBy(Position::Time(dur)) => {
                // Hook into Message::SkipTo
                self.update(Message::SkipTo(Position::Time(helpers::safe_duration_sub(
                    self.video.position(),
                    dur,
                ))));
            }

            SkipForwardBy(Position::Frame(f)) => {
                // Hook into Message::SkipTo
                self.update(Message::SkipTo(Position::Time(
                    self.video.position()
                        + Duration::from_millis(f * (1000.0 / self.video.framerate()) as u64),
                )));
            }

            SkipBackwardBy(Position::Frame(f)) => {
                // Hook into Message::SkipTo
                self.update(Message::SkipTo(Position::Time(helpers::safe_duration_sub(
                    self.video.position(),
                    Duration::from_millis(f * (1000.0 / self.video.framerate()) as u64),
                ))));
            }

            TogglePlay => {
                // Hook into Message::Pause and Message::Play
                match self.video.paused() {
                    true => self.update(Message::Play(Position::Time(self.video.position()))),
                    false => self.update(Message::Pause(Position::Time(self.video.position()))),
                };
            }

            // Not intended to be networked
            ToggleLoop => self.video.set_looping(self.video.looping().not()),
            WindowResize(s) => self.window_size = s,

            _ => {}
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let controls = column![
            container(
                container(Space::new())
                    .style(style::seekbar_progress)
                    .height(Fill)
                    .width(
                        self.window_size.width
                            * (self.video.position().as_millis() as f32
                                / self.video.duration().as_millis() as f32)
                    )
            )
            .width(Fill)
            .height(4)
            .style(style::seekbar),
            container(
                row![
                    button(
                        svg("src/assets/replay-5.svg")
                            .height(30)
                            .width(30)
                            .style(style::media_button_icon)
                    )
                    .style(style::media_button)
                    .on_press(Message::SkipBackwardBy(Position::Time(Duration::from_secs(5)))),
                    button(
                        svg(match self.video_paused {
                            true => "src/assets/play.svg",
                            false => "src/assets/pause.svg",
                        })
                        .width(30)
                        .height(30)
                        .style(style::media_button_icon)
                    )
                    .on_press(Message::TogglePlay)
                    .style(style::media_button),
                    button(
                        svg("src/assets/forward-5.svg")
                            .width(30)
                            .height(30)
                            .style(style::media_button_icon)
                    )
                    .style(style::media_button)
                    .on_press(Message::SkipForwardBy(Position::Time(Duration::from_secs(5)))),
                    text(format!(
                        "{} / {}",
                        FormatTimestamp(self.video.position().as_secs()),
                        FormatTimestamp(self.video.duration().as_secs())
                    )),
                    container(
                        button(
                            if self.video.looping() {
                                svg("src/assets/repeat-on.svg").style(style::media_button_icon)
                            } else {
                                svg("src/assets/repeat.svg").style(style::media_button_icon_inactive)
                            }
                            .width(30)
                            .height(30)
                        )
                        .style(style::media_button)
                        .on_press(Message::ToggleLoop)
                    )
                    .align_right(Fill)
                ]
                .padding([0, 10])
                .align_y(alignment::Vertical::Center)
                .height(Fill)
            )
            .width(Fill)
            .height(50)
            .style(style::control_panel)
        ];

        stack![
            container(Space::new())
                .width(Fill)
                .height(Fill)
                .style(style::background),
            container(
                mouse_area(
                    VideoPlayer::new(&self.video)
                        .width(Fill)
                        .height(Fill)
                        .content_fit(iced::ContentFit::Contain)
                )
                .on_press(Message::TogglePlay)
            )
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Top)
            .width(Fill)
            .height(self.window_size.height - 50.0),
            container(controls)
                .width(Fill)
                .height(Fill)
                .align_y(alignment::Vertical::Bottom)
                .align_x(alignment::Horizontal::Center)
        ]
        .width(Fill)
        .height(Fill)
        .into()
    }
}

fn main() {
    // Prep logging to only log this crate
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("synchro_player", log::LevelFilter::Info)
        .init();

    info!("Logging initialized! Hello, world!");
    App::run(Size::new(1000.0, 563.0));
}
