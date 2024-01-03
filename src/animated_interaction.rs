use bevy::prelude::*;
use sickle_math::ease::{Ease, ValueEasing};

use crate::{FluxInteraction, FluxInteractionStopwatch, FluxInteractionUpdate};

pub struct AnimatedInteractionPlugin;

impl Plugin for AnimatedInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            AnimatedInteractionUpdate.after(FluxInteractionUpdate),
        );
    }
}

#[derive(SystemSet, Clone, Hash, Debug, Eq, PartialEq)]
pub struct AnimatedInteractionUpdate;

#[derive(Debug, Copy, Clone, Reflect)]
pub enum AnimationProgress {
    Start,
    Inbetween(f32),
    End,
}

#[derive(Debug, Copy, Clone, Reflect)]
pub struct AnimationConfig {
    pub duration: f32,
    pub easing: Ease,
    pub out_duration: Option<f32>,
    pub out_easing: Option<Ease>,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration: Default::default(),
            easing: Default::default(),
            out_duration: Default::default(),
            out_easing: Default::default(),
        }
    }
}

#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
pub struct AnimatedInteractionState<T: Component> {
    pub context: Option<T>,
    pub progress: AnimationProgress,
}

impl<T: Component> Default for AnimatedInteractionState<T> {
    fn default() -> Self {
        Self {
            context: None,
            progress: AnimationProgress::Start,
        }
    }
}

#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
pub struct AnimatedInteraction<T: Component> {
    pub context: Option<T>,
    pub tween: AnimationConfig,
    pub hover: Option<AnimationConfig>,
    pub press: Option<AnimationConfig>,
    pub cancel: Option<AnimationConfig>,
    pub reset_delay: Option<f32>,
}

impl<T: Component> Default for AnimatedInteraction<T> {
    fn default() -> Self {
        Self {
            context: None,
            tween: AnimationConfig {
                duration: 0.1,
                ..default()
            },
            hover: Default::default(),
            press: AnimationConfig {
                duration: 0.05,
                out_duration: Some(0.),
                ..default()
            }
            .into(),
            cancel: AnimationConfig {
                duration: 0.1,
                ..default()
            }
            .into(),
            reset_delay: Default::default(),
        }
    }
}

pub fn add_animated_interaction_state<T: Component + Default>(
    mut commands: Commands,
    q_animated: Query<
        Entity,
        (
            With<FluxInteraction>,
            With<AnimatedInteraction<T>>,
            Without<AnimatedInteractionState<T>>,
        ),
    >,
) {
    for entity in &q_animated {
        commands
            .entity(entity)
            .insert(AnimatedInteractionState::<T>::default());
    }
}

pub fn update_animated_interaction_state<T: Component>(
    mut q_interaction: Query<(
        &AnimatedInteraction<T>,
        &FluxInteraction,
        &FluxInteractionStopwatch,
        &mut AnimatedInteractionState<T>,
    )>,
) {
    for (animation, interaction, stopwatch, mut animation_state) in &mut q_interaction {
        let (base_tween, hover, press, cancel) = (
            animation.tween,
            animation.hover,
            animation.press,
            animation.cancel,
        );

        let elapsed = stopwatch.0.elapsed_secs();

        let progress = match *interaction {
            FluxInteraction::Pressed => {
                let tween = press.unwrap_or(base_tween);
                let tween_time = tween.duration.max(0.);

                if tween_time == 0. {
                    AnimationProgress::End
                } else {
                    let tween_ratio = (elapsed / tween_time).clamp(0., 1.).ease(tween.easing);
                    AnimationProgress::Inbetween(tween_ratio)
                }
            }
            FluxInteraction::Released => {
                let tween = press.unwrap_or(base_tween);
                let tween_time = tween.out_duration.unwrap_or(tween.duration).max(0.);

                if tween_time == 0. {
                    AnimationProgress::End
                } else {
                    let easing = tween.out_easing.unwrap_or(tween.easing);
                    let tween_ratio = (elapsed / tween_time).clamp(0., 1.).ease(easing);

                    if tween_ratio == 1. {
                        AnimationProgress::End
                    } else {
                        AnimationProgress::Inbetween(tween_ratio)
                    }
                }
            }
            FluxInteraction::PressCanceled => {
                let tween = cancel.unwrap_or(base_tween);
                let tween_time = tween.duration.max(0.);

                let reset_delay = animation.reset_delay.unwrap_or(tween_time).max(0.);
                let reset_length = tween.out_duration.unwrap_or(tween_time).max(0.);

                if elapsed < reset_delay {
                    AnimationProgress::Start
                } else {
                    let easing = tween.out_easing.unwrap_or(tween.easing);
                    let tween_ratio = ((elapsed - reset_delay) / reset_length)
                        .clamp(0., 1.)
                        .ease(easing);

                    if tween_time == 0. || tween_ratio == 1. {
                        AnimationProgress::End
                    } else {
                        AnimationProgress::Inbetween(tween_ratio)
                    }
                }
            }
            FluxInteraction::PointerEnter => {
                let tween = hover.unwrap_or(base_tween);
                let tween_time = tween.duration.max(0.);

                if tween_time == 0. {
                    AnimationProgress::End
                } else {
                    let tween_ratio = (elapsed / tween_time).clamp(0., 1.).ease(tween.easing);
                    if tween_ratio == 1. {
                        AnimationProgress::End
                    } else {
                        AnimationProgress::Inbetween(tween_ratio)
                    }
                }
            }
            FluxInteraction::PointerLeave => {
                let tween = hover.unwrap_or(base_tween);
                let tween_time = tween.out_duration.unwrap_or(tween.duration).max(0.);

                if tween_time == 0. {
                    AnimationProgress::End
                } else {
                    let easing = tween.out_easing.unwrap_or(tween.easing);
                    let tween_ratio = (elapsed / tween_time).clamp(0., 1.).ease(easing);
                    if tween_ratio == 1. {
                        AnimationProgress::End
                    } else {
                        AnimationProgress::Inbetween(tween_ratio)
                    }
                }
            }
            _ => AnimationProgress::End,
        };

        animation_state.progress = progress;
    }
}