use std::collections::HashMap;
use bevy::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{clip::Clip, easing::Easing};
use crate::clip::ClipId;
use crate::events::Marker;

/// The duration of an [Animation].
#[derive(Debug, Clone, Copy, Reflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[reflect(Debug)]
pub enum AnimationDuration {
    /// Specifies the duration of one frame in milliseconds (default = `PerFrame(100)`).
    PerFrame(u32),
    /// Specifies the duration of one repetition of the animation in milliseconds.
    PerRepetition(u32),
}

impl Default for AnimationDuration {
    fn default() -> Self {
        Self::PerFrame(100)
    }
}

/// How many times an [Animation] repeats.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[reflect(Debug, PartialEq, Hash)]
pub enum AnimationRepeat {
    /// Loops forever (default).
    #[default]
    Loop,
    /// Repeats n times.
    Times(usize),
}

/// The direction in which the frames of an [Animation] are played.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[reflect(Debug, PartialEq, Hash)]
pub enum AnimationDirection {
    /// Frames play from left to right (default).
    #[default]
    Forwards,
    /// Frames play from right to left.
    Backwards,
    /// Alternates at each repetition of the animation, starting from left to right.
    PingPong,
}

/// A playable animation to assign to a [SpritesheetAnimation](crate::prelude::SpritesheetAnimation) component.
///
/// Use [Spritesheet::create_animation()](crate::prelude::Spritesheet::create_animation) to build new animations.
///
/// An animation is composed of one or several [Clips](crate::prelude::Clip).
/// - For simple animation, you can directly add frames to the default clip (it doesn't need to be created explicitly).
/// - For more sophisticated animations, you can create new clips with [start_clip()](crate::prelude::AnimationBuilder::start_clip) and [copy_clip()](crate::prelude::AnimationBuilder::copy_clip).
///
/// # Parameters
///
/// Playback parameters like [duration](crate::prelude::AnimationDuration), [repetitions](crate::prelude::AnimationRepeat), [direction](crate::prelude::AnimationDirection) and [easing](crate::prelude::Easing) can be specified.
///
/// Those animation-level parameters will be combined with the parameters of the underlying [Clips](crate::prelude::Clip).
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_spritesheet_animation::prelude::*;
/// # fn f(assets: &AssetServer) {
/// let image = assets.load("character.png");
///
/// let spritesheet = Spritesheet::new(&image, 8, 4);
///
/// let animation = spritesheet
///     .create_animation()
///     // Global animation parameters
///     .set_repetitions(AnimationRepeat::Loop)
///     .set_easing(Easing::In(EasingVariety::Quadratic))
///     // Clip 1 (default clip, doesn't need to be created explicitly)
///     .add_row(3)
///     .set_clip_duration(AnimationDuration::PerRepetition(2000))
///     // Clip 2
///     .start_clip()
///     .add_row(5)
///     .set_clip_repetitions(10)
///     .set_clip_direction(AnimationDirection::PingPong)
///     // Get the final animation
///     .build();
/// # }
/// ```
#[derive(Asset, Debug, Clone, Reflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[reflect(Debug)]
pub struct Animation {
    pub(crate) clips: Vec<Clip>,

    pub(crate) duration: Option<AnimationDuration>,
    pub(crate) repetitions: Option<AnimationRepeat>,
    pub(crate) direction: Option<AnimationDirection>,
    pub(crate) easing: Option<Easing>,
}

impl Animation {
    /// Private empty animation constructor.
    ///
    /// Users should not create empty animations.
    /// Our animation builder starts with such an empty animation though.
    pub(crate) fn empty() -> Self {
        Self {
            clips: vec![Clip::empty()],
            duration: None,
            repetitions: None,
            direction: None,
            easing: None,
        }
    }

    /// The [Clips](crate::prelude::Clip) that compose this animation
    pub fn clips(&self) -> &[Clip] {
        &self.clips
    }

    /// The optional duration of this animation
    pub fn duration(&self) -> &Option<AnimationDuration> {
        &self.duration
    }

    /// The optional number of repetitions of this animation
    pub fn repetitions(&self) -> &Option<AnimationRepeat> {
        &self.repetitions
    }

    /// The optional direction of this animation
    pub fn direction(&self) -> &Option<AnimationDirection> {
        &self.direction
    }

    /// The optional easing of this animation
    pub fn easing(&self) -> &Option<Easing> {
        &self.easing
    }

    /// Refreshes the IDs with the provided [refresher]
    pub fn refresh_ids<R: IdRefresher>(&mut self, refresher: &mut R) -> Result<(), R::Error> {
        for (index, clip) in self.clips.iter_mut().enumerate() {
            clip.refresh_ids(index, refresher)?;
        }
        Ok(())
    }
}


/// Refreshes the IDs of an [Animation] and the associated [Clip]s.
/// Can return an error if necessary.
pub trait IdRefresher {

    /// An error returned when something goes wrong while updating.
    type Error;

    /// Refreshes a [ClipId] when some value is returned.
    fn refresh_clip_id(&mut self, index: usize, clip_id: ClipId) -> Result<ClipId, Self::Error>;

    /// Refreshes the markers of a specific frame in a clip in an animation.
    fn refresh_marker(&mut self, clip_id: ClipId, frame: usize, marker: Marker) -> Result<Marker, Self::Error>;
}


/// A primitive implementation for refreshing IDs based on [HashMap].
#[derive(Default)]
pub struct MappingIdRefresher {
    clip_ids: HashMap<ClipId, ClipId>,
    markers: HashMap<Marker, Marker>,
}

impl MappingIdRefresher {

    /// Initialize a new refresher
    pub fn new(clip_ids: HashMap<ClipId, ClipId>, markers: HashMap<Marker, Marker>) -> Self {
        Self { clip_ids, markers }
    }

    /// Get the [ClipId] mapping
    pub fn clip_id_mapping(&self) -> &HashMap<ClipId, ClipId> {
        &self.clip_ids
    }

    /// Get the [Marker] mapping
    pub fn marker_mapping(&self) -> &HashMap<Marker, Marker> {
        &self.markers
    }

    /// Get the [ClipId] and [Marker] mappings.
    fn into_inner(self) -> (HashMap<ClipId, ClipId>, HashMap<Marker, Marker>) {
        (self.clip_ids, self.markers)
    }
}

/// A marker that something can never happen
pub struct Impossible;

impl IdRefresher for MappingIdRefresher {
    type Error = Impossible;

    fn refresh_clip_id(&mut self, _: usize, clip_id: ClipId) -> Result<ClipId, Self::Error> {
        Ok(self.clip_ids.entry(clip_id).or_insert_with(|| ClipId::new()).clone())
    }

    fn refresh_marker(&mut self, _: ClipId, _: usize, marker: Marker) -> Result<Marker, Self::Error> {
        Ok(self.markers.entry(marker).or_insert_with(|| Marker::new()).clone())
    }
}


/// The default implementation of an [IdRefresher]
pub type DefaultIdRefresher = MappingIdRefresher;