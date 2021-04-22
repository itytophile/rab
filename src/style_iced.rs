use iced::{button, container, Background, Color, Vector};

pub enum Container {
    Fire,
    Thunder,
    Ice,
    Water,
    Dragon,
    Defense,
    Talisman,
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        match self {
            Container::Fire => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::from_rgb(0.64, 0.34, 0.37))),
                border_radius: 10.0,
                ..container::Style::default()
            },
            Container::Thunder => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::from_rgb(0.71, 0.67, 0.41))),
                border_radius: 10.0,
                ..container::Style::default()
            },
            Container::Ice => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::from_rgb(0.49, 0.67, 0.67))),
                border_radius: 10.0,
                ..container::Style::default()
            },
            Container::Water => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::from_rgb(0.45, 0.57, 0.67))),
                border_radius: 10.0,
                ..container::Style::default()
            },
            Container::Dragon => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::from_rgb(0.53, 0.50, 0.62))),
                border_radius: 10.0,
                ..container::Style::default()
            },
            Container::Defense => container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::from_rgb(0.7, 0.7, 0.7))),
                ..container::Style::default()
            },
            Container::Talisman => container::Style {
                border_radius: 10.0,
                background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
                ..container::Style::default()
            },
        }
    }
}

pub enum Button {
    Remove,
    Add,
    Search,
    Result,
    Talisman,
    Edit,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::Remove => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.95, 0.27, 0.41))),
                border_radius: 5.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            },
            Button::Add => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.28, 0.78, 0.56))),
                border_radius: 5.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            },
            Button::Search => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.28, 0.37, 0.78))),
                border_radius: 5.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            },
            Button::Result => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.94, 0.96, 0.98))),
                border_radius: 5.0,
                text_color: Color::from_rgb(0.16, 0.44, 0.66),
                ..button::Style::default()
            },
            Button::Talisman => button::Style {
                background: Some(Background::Color(Color::from_rgb(0.24, 0.56, 0.82))),
                border_radius: 5.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            },
            Button::Edit => button::Style {
                background: Some(Background::Color(Color::from_rgb(1., 0.88, 0.54))),
                text_color: Color::BLACK,
                ..button::Style::default()
            },
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            background: match self {
                Button::Result => Some(Background::Color(Color::from_rgb(0.89, 0.94, 0.98))),
                _ => active.background,
            },
            shadow_offset: match self {
                Button::Result => active.shadow_offset,
                _ => active.shadow_offset + Vector::new(0.0, 1.0),
            },
            ..active
        }
    }
}
