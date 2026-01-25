use std::collections::HashMap;
use crate::clip::ClipId;
use crate::events::Marker;

/// Refreshes the IDs of an [Animation] and the associated [Clip]s.
/// Can return an error if necessary.
pub trait IdRefresher {

    /// An error returned when something goes wrong while updating.
    type Error;

    /// Refreshes a [ClipId] when some value is returned.
    fn refresh_clip_id(&mut self, index: usize, clip_id: ClipId) -> bevy::prelude::Result<ClipId, Self::Error>;

    /// Refreshes the markers of a specific frame in a clip in an animation.
    fn refresh_marker(&mut self, clip_id: ClipId, frame: usize, marker: Marker) -> bevy::prelude::Result<Marker, Self::Error>;
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

    fn refresh_clip_id(&mut self, _: usize, clip_id: ClipId) -> bevy::prelude::Result<ClipId, Self::Error> {
        Ok(self.clip_ids.entry(clip_id).or_insert_with(|| ClipId::new()).clone())
    }

    fn refresh_marker(&mut self, _: ClipId, _: usize, marker: Marker) -> bevy::prelude::Result<Marker, Self::Error> {
        Ok(self.markers.entry(marker).or_insert_with(|| Marker::new()).clone())
    }
}


/// The default implementation of an [IdRefresher]
pub type DefaultIdRefresher = MappingIdRefresher;