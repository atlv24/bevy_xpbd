use bevy::{ecs::entity::hash_set::EntityHashSet, prelude::*};

use crate::prelude::*;

/// Rules that determine which colliders are taken into account in [spatial queries](crate::spatial_query).
///
/// # Example
///
/// ```
#[cfg_attr(feature = "2d", doc = "use avian2d::prelude::*;")]
#[cfg_attr(feature = "3d", doc = "use avian3d::prelude::*;")]
/// use bevy::prelude::*;
///
/// fn setup(mut commands: Commands) {
#[cfg_attr(
    feature = "2d",
    doc = "    let object = commands.spawn(Collider::circle(0.5)).id();"
)]
#[cfg_attr(
    feature = "3d",
    doc = "    let object = commands.spawn(Collider::sphere(0.5)).id();"
)]
///
///     // A query filter that has three collision layers and excludes the `object` entity
///     let query_filter = SpatialQueryFilter::from_mask(0b1011).with_excluded_entities([object]);
///
///     // Spawn a ray caster with the query filter
///     commands.spawn(RayCaster::default().with_query_filter(query_filter));
/// }
/// ```
#[derive(Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", reflect(Serialize, Deserialize))]
pub struct SpatialQueryFilter<'a> {
    /// Specifies which [collision layers](CollisionLayers) will be included in the [spatial query](crate::spatial_query).
    pub mask: LayerMask,
    /// Allows filtering candidates before spatial tests are performed.
    pub predicate: Option<&'a dyn Fn(Entity) -> bool>,
}

impl<'a> std::fmt::Debug for SpatialQueryFilter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("SpatialQueryFilter")
            .field("mask", &self.mask)
            .finish()
    }
}

impl<'a> PartialEq for SpatialQueryFilter<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask
    }
}

impl<'a> Default for SpatialQueryFilter<'a> {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl<'a> SpatialQueryFilter<'a> {
    /// The default [`SpatialQueryFilter`] configuration that includes all collision layers
    /// and has no predicate.
    pub const DEFAULT: Self = Self {
        mask: LayerMask::ALL,
        predicate: None,
    };

    /// Creates a new [`SpatialQueryFilter`] with the given [`LayerMask`] determining
    /// which [collision layers] will be included in the [spatial query].
    ///
    /// [collision layers]: CollisionLayers
    /// [spatial query]: crate::spatial_query
    pub fn from_mask(mask: impl Into<LayerMask>) -> Self {
        Self {
            mask: mask.into(),
            ..default()
        }
    }

    /// Sets the [`LayerMask`] of the filter configuration. Only colliders with the corresponding
    /// [collision layer memberships] will be included in the [spatial query].
    ///
    /// [collision layer memberships]: CollisionLayers
    /// [spatial query]: crate::spatial_query
    pub fn with_mask(mut self, masks: impl Into<LayerMask>) -> Self {
        self.mask = masks.into();
        self
    }

    /// Excludes the given entities from the [spatial query](crate::spatial_query).
    pub fn with_predicate<'b: 'a>(mut self, predicate: &'b dyn Fn(Entity) -> bool) -> Self {
        self.predicate = Some(predicate);
        self
    }

    /// Tests if an entity should be included in [spatial queries] based on the filter configuration.
    ///
    /// [spatial queries]: crate::spatial_query
    pub fn test(&self, entity: Entity, layers: CollisionLayers) -> bool {
        self.predicate
            .map(|predicate| predicate(entity))
            .unwrap_or(true)
            && CollisionLayers::new(LayerMask::ALL, self.mask)
                .interacts_with(CollisionLayers::new(layers.memberships, LayerMask::ALL))
    }
}
