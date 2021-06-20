use iced::{
    widget::svg::{Handle, Svg},
    Align, Button, Column, Container, Element, HorizontalAlignment, Length, Row, Rule, Scrollable,
    Space, Text,
};

use crate::{locale::InterfaceSymbol, style_iced};

use super::{
    common_elements::{
        build_part_to_button, ARM_ICON, BUTTON_SPACING, CHEST_ICON, HELMET_ICON, ICON_SIZE,
        LEG_ICON, SCROLL_PADDING, TALISMAN_ICON, WAIST_ICON,
    },
    MainApp, Msg, Page,
};

pub trait BuildsPage {
    fn get_builds_page(&mut self) -> Element<Msg>;
}

impl BuildsPage for MainApp {
    fn get_builds_page(&'_ mut self) -> Element<'_, Msg> {
        let builds = &self.saved_builds;
        let mut builds_scrolls = Scrollable::new(&mut self.state_builds_scroll)
            .align_items(Align::Center)
            .spacing(10)
            .padding(SCROLL_PADDING);
        let size = builds.len();
        if size == 0 {
            builds_scrolls = builds_scrolls.push(Text::new(InterfaceSymbol::NoResult));
        } else {
            for ((key, (name, build)), state_button) in builds
                .iter()
                .enumerate()
                .zip(self.states_saved_builds_button.iter_mut())
            {
                let mut details_button = Button::new(
                    &mut state_button.6,
                    Text::new(name)
                        .width(Length::Units(200))
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                .width(Length::Units(200)) // seems repetitive but needed for centering
                // the icons
                .style(style_iced::Button::Talisman);
                details_button = details_button.on_press(Msg::SavedBuildDetails(name.clone()));
                let row_build = Row::new()
                    .align_items(Align::Center)
                    .spacing(BUTTON_SPACING)
                    .push(
                        Button::new(
                            &mut state_button.7,
                            Text::new(InterfaceSymbol::Remove)
                                .width(Length::Units(100))
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .style(style_iced::Button::Remove)
                        .on_press(Msg::RemoveSavedBuild(name.clone()))
                        .width(Length::Units(100)),
                    )
                    .push(details_button)
                    .push(build_part_to_button(&mut state_button.0, &build.helmet))
                    .push(build_part_to_button(&mut state_button.1, &build.chest))
                    .push(build_part_to_button(&mut state_button.2, &build.arm))
                    .push(build_part_to_button(&mut state_button.3, &build.waist))
                    .push(build_part_to_button(&mut state_button.4, &build.leg))
                    .push(build_part_to_button(&mut state_button.5, &build.talisman));
                builds_scrolls = builds_scrolls.push(row_build);
                if key < size - 1 {
                    builds_scrolls = builds_scrolls.push(Rule::horizontal(1))
                }
            }
        }

        // to center the titles
        let space_width = if SCROLL_PADDING < BUTTON_SPACING {
            0
        } else {
            SCROLL_PADDING - BUTTON_SPACING
        };

        let mut col_titles = Row::new()
            .spacing(BUTTON_SPACING)
            .push(Space::with_width(Length::Units(space_width)))
            .push(Space::with_width(Length::Units(200)))
            .push(Space::with_width(Length::Units(100)));

        for icon in [
            HELMET_ICON.to_vec(),
            CHEST_ICON.to_vec(),
            ARM_ICON.to_vec(),
            WAIST_ICON.to_vec(),
            LEG_ICON.to_vec(),
            TALISMAN_ICON.to_vec(),
        ] {
            col_titles = col_titles.push(
                Container::new(Svg::new(Handle::from_memory(icon)).width(Length::Units(ICON_SIZE)))
                    .width(Length::Fill)
                    .center_x(),
            );
        }
        Container::new(
            Column::new()
                .push(col_titles.push(Space::with_width(Length::Units(space_width))))
                .push(builds_scrolls.width(Length::Fill).height(Length::Fill))
                .push(
                    Row::new().push(Space::with_width(Length::Fill)).push(
                        Button::new(
                            &mut self.state_lang_button,
                            Text::new(InterfaceSymbol::Back),
                        )
                        .on_press(Msg::ChangePage(Page::Main)),
                    ),
                ),
        )
        .padding(5)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
