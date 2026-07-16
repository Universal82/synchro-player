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

pub fn background(_: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: None,
        background: Some(iced::Background::Color(color!(0,0,0))),
        border: NO_BORDER,
        shadow: NO_SHADOW,
        snap: true,
    }
}

pub fn control_panel(_: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: None,
        background: Some(iced::Background::Color(color!(30,30,30))),
        border: NO_BORDER,
        shadow: NO_SHADOW,
        snap: true,
    }
}

pub fn seekbar(_: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: None,
        background: Some(iced::Background::Color(color!(0,0,0))),
        border: NO_BORDER,
        shadow: NO_SHADOW,
        snap: true,
    }
}

pub fn seekbar_progress(_: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: None,
        background: Some(iced::Background::Color(color!(200,0,0))),
        border: NO_BORDER,
        shadow: NO_SHADOW,
        snap: true,
    }
}


pub fn media_button(_: &iced::Theme, _: iced::widget::button::Status) -> iced::widget::button::Style {
    iced::widget::button::Style {
        text_color: color!(255,255,255),
        background: None,
        border: NO_BORDER,
        shadow: NO_SHADOW,
        snap: true,
    }
}

pub fn media_button_icon(_: &iced::Theme, _: iced::widget::svg::Status) -> iced::widget::svg::Style {
    iced::widget::svg::Style {
        color: Some(color!(255,255,255)),
    }
}

pub fn media_button_icon_inactive(_: &iced::Theme, _: iced::widget::svg::Status) -> iced::widget::svg::Style {
    iced::widget::svg::Style {
        color: Some(color!(200,200,200)),
    }
}