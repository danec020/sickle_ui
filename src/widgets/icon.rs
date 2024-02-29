use bevy::prelude::*;

use crate::{ui_builder::*, ui_style::SetImageExt};

pub struct IconPlugin;

impl Plugin for IconPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Icon;

impl Icon {
    fn bundle() -> impl Bundle {
        ImageBundle {
            style: Style {
                width: Val::Px(16.),
                height: Val::Px(16.),
                ..default()
            },
            ..default()
        }
    }
}

pub trait UiIconExt<'w, 's> {
    fn icon<'a>(&'a mut self, path: impl Into<String>) -> UiBuilder<'w, 's, 'a>;
}

impl<'w, 's> UiIconExt<'w, 's> for UiBuilder<'w, 's, '_> {
    fn icon<'a>(&'a mut self, path: impl Into<String>) -> UiBuilder<'w, 's, 'a> {
        let mut icon = self.spawn((Icon::bundle(), Icon));

        icon.style().image(path);

        icon
    }
}
