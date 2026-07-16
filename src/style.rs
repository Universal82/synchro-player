use iced::color;

pub static NO_SHADOW: iced::Shadow = iced::Shadow {
    color: iced::color!(0,0,0),
    offset: iced::Vector {
        x: 0.0,
        y: 0.0
    },
    blur_radius: 0.0,
};

pub static NO_RADIUS: iced::border::Radius = iced::border::Radius {
    top_left: 0.0,
    top_right: 0.0,
    bottom_right: 0.0,
    bottom_left: 0.0,
};

pub static NO_BORDER: iced::border::Border = iced::border::Border {
    color: color!(0,0,0),
    width: 0.0,
    radius: NO_RADIUS,
};

pub fn control_panel(_: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: None,
        background: Some(iced::Background::Color(color!(127,127,127))),
        border: NO_BORDER,
        shadow: NO_SHADOW,
        snap: true,
    }
}

pub fn seekbar(_: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: None,
        background: Some(iced::Background::Color(color!(127,0,0))),
        border: NO_BORDER,
        shadow: NO_SHADOW,
        snap: true,
    }
}

pub fn media_button(_: &iced::Theme, _: iced::widget::button::Status) -> iced::widget::button::Style {
    iced::widget::button::Style {
        text_color: color!(0,0,0),
        background: None,
        border: NO_BORDER,
        shadow: NO_SHADOW,
        snap: true,
    }
}
