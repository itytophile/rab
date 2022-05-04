use super::{
    common_elements::{
        build_part_to_button, ARM_ICON, BUTTON_SPACING, CHEST_ICON, HELMET_ICON, ICON_LENGTH,
        LEG_ICON, SCROLL_PADDING, TALISMAN_ICON, WAIST_ICON,
    },
    MainApp, Msg, Page,
};
use crate::{locale::InterfaceSymbol, style_iced};
use iced::{
    alignment, pure,
    widget::svg::{Handle, Svg},
    Alignment, Length, Rule, Space, Text,
};

pub trait BuildsPage {
    fn get_builds_page(&self) -> pure::widget::Container<'_, Msg>;
}

impl BuildsPage for MainApp {
    fn get_builds_page(&'_ self) -> pure::widget::Container<'_, Msg> {
        let builds = &self.saved_builds;
        let mut builds_scrolls = pure::column()
            .align_items(Alignment::Center)
            .spacing(10)
            .padding(SCROLL_PADDING);
        let size = builds.len();
        if size == 0 {
            builds_scrolls = builds_scrolls.push(Text::new(InterfaceSymbol::NoResult));
        } else {
            for (key, (name, build)) in builds.iter().enumerate() {
                let mut details_button = pure::button(
                    Text::new(name)
                        .width(Length::Units(200))
                        .horizontal_alignment(alignment::Horizontal::Center),
                )
                .width(Length::Units(200)) // seems repetitive but needed for centering
                // the icons
                .style(style_iced::Button::Talisman);
                details_button = details_button.on_press(Msg::SavedBuildDetails(name.clone()));
                let row_build = pure::row()
                    .align_items(Alignment::Center)
                    .spacing(BUTTON_SPACING)
                    .push(
                        pure::button(
                            Text::new(InterfaceSymbol::Remove)
                                .width(Length::Units(100))
                                .horizontal_alignment(alignment::Horizontal::Center),
                        )
                        .style(style_iced::Button::Remove)
                        .on_press(Msg::RemoveSavedBuild(name.clone()))
                        .width(Length::Units(100)),
                    )
                    .push(details_button)
                    .push(build_part_to_button(&build.helmet))
                    .push(build_part_to_button(&build.chest))
                    .push(build_part_to_button(&build.arm))
                    .push(build_part_to_button(&build.waist))
                    .push(build_part_to_button(&build.leg))
                    .push(build_part_to_button(&build.talisman));
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

        let mut col_titles = pure::row()
            .spacing(BUTTON_SPACING)
            .push(Space::with_width(Length::Units(space_width)))
            .push(Space::with_width(Length::Units(200)))
            .push(Space::with_width(Length::Units(100)));

        for icon in [
            HELMET_ICON,
            CHEST_ICON,
            ARM_ICON,
            WAIST_ICON,
            LEG_ICON,
            TALISMAN_ICON,
        ] {
            col_titles = col_titles.push(
                pure::container(
                    Svg::new(Handle::from_memory(icon)).width(ICON_LENGTH),
                )
                .width(Length::Fill)
                .center_x(),
            );
        }
        pure::container(
            pure::column()
                .push(col_titles.push(Space::with_width(Length::Units(space_width))))
                .push(pure::scrollable(
                    builds_scrolls.width(Length::Fill).height(Length::Fill),
                ))
                .push(
                    pure::row().push(Space::with_width(Length::Fill)).push(
                        pure::button(Text::new(InterfaceSymbol::Back))
                            .on_press(Msg::ChangePage(Page::Main)),
                    ),
                ),
        )
        .padding(5)
        .width(Length::Fill)
        .height(Length::Fill)
    }
}
