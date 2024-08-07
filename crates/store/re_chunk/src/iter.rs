use std::sync::Arc;

use arrow2::{
    array::{Array as ArrowArray, PrimitiveArray},
    Either,
};
use itertools::izip;

use re_log_types::{TimeInt, Timeline};
use re_types_core::{Component, ComponentName};

use crate::{Chunk, ChunkTimeline, RowId};

// ---

// NOTE: Regarding the use of (recursive) `Either` in this file: it is _not_ arbitrary.
//
// They _should_ all follow this model:
// * The first layer is always the emptiness layer: `Left` is empty, `Right` is non-empty.
// * The second layer is the temporarily layer: `Left` is static, `Right` is temporal.
// * Any layers beyond that follow the same pattern: `Left` doesn't have something, while `Right` does.

impl Chunk {
    /// Returns an iterator over the indices (`(TimeInt, RowId)`) of a [`Chunk`], for a given timeline.
    ///
    /// If the chunk is static, `timeline` will be ignored.
    ///
    /// See also:
    /// * [`Self::iter_component_indices`].
    /// * [`Self::iter_indices_owned`].
    #[inline]
    pub fn iter_indices(&self, timeline: &Timeline) -> impl Iterator<Item = (TimeInt, RowId)> + '_ {
        if self.is_static() {
            Either::Right(Either::Left(izip!(
                std::iter::repeat(TimeInt::STATIC),
                self.row_ids()
            )))
        } else {
            let Some(time_chunk) = self.timelines.get(timeline) else {
                return Either::Left(std::iter::empty());
            };

            Either::Right(Either::Right(izip!(time_chunk.times(), self.row_ids())))
        }
    }

    /// Returns an iterator over the indices (`(TimeInt, RowId)`) of a [`Chunk`], for a given
    /// timeline and component.
    ///
    /// If the chunk is static, `timeline` will be ignored.
    ///
    /// This is different than [`Self::iter_indices`] in that it will only yield indices for rows
    /// at which there is data for the specified `component_name`.
    ///
    /// See also [`Self::iter_indices`].
    pub fn iter_component_indices(
        &self,
        timeline: &Timeline,
        component_name: &ComponentName,
    ) -> impl Iterator<Item = (TimeInt, RowId)> + '_ {
        let Some(list_array) = self.components.get(component_name) else {
            return Either::Left(std::iter::empty());
        };

        if self.is_static() {
            let indices = izip!(std::iter::repeat(TimeInt::STATIC), self.row_ids());

            if let Some(validity) = list_array.validity() {
                Either::Right(Either::Left(Either::Left(
                    indices
                        .enumerate()
                        .filter_map(|(i, o)| validity.get_bit(i).then_some(o)),
                )))
            } else {
                Either::Right(Either::Left(Either::Right(indices)))
            }
        } else {
            let Some(time_chunk) = self.timelines.get(timeline) else {
                return Either::Left(std::iter::empty());
            };

            let indices = izip!(time_chunk.times(), self.row_ids());

            if let Some(validity) = list_array.validity() {
                Either::Right(Either::Right(Either::Left(
                    indices
                        .enumerate()
                        .filter_map(|(i, o)| validity.get_bit(i).then_some(o)),
                )))
            } else {
                Either::Right(Either::Right(Either::Right(indices)))
            }
        }
    }

    /// Returns an iterator over the offsets (`(offset, len)`) of a [`Chunk`], for a given
    /// component.
    ///
    /// I.e. each `(offset, len)` pair describes the position of a component batch in the
    /// underlying arrow array of values.
    pub fn iter_component_offsets(
        &self,
        component_name: &ComponentName,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let Some(list_array) = self.components.get(component_name) else {
            return Either::Left(std::iter::empty());
        };

        let offsets = list_array.offsets().iter().map(|idx| *idx as usize);
        let lengths = list_array.offsets().lengths();

        if let Some(validity) = list_array.validity() {
            Either::Right(Either::Left(
                izip!(offsets, lengths)
                    .enumerate()
                    .filter_map(|(i, o)| validity.get_bit(i).then_some(o)),
            ))
        } else {
            Either::Right(Either::Right(izip!(offsets, lengths)))
        }
    }

    /// Returns an iterator over the raw arrays of a [`Chunk`], for a given component.
    ///
    /// See also:
    /// * [`Self::iter_component`].
    /// * [`Self::iter_primitive`].
    pub fn iter_component_arrays(
        &self,
        component_name: &ComponentName,
    ) -> impl Iterator<Item = Box<dyn ArrowArray>> + '_ {
        let Some(list_array) = self.components.get(component_name) else {
            return Either::Left(std::iter::empty());
        };

        Either::Right(list_array.iter().flatten())
    }

    /// Returns an iterator over the raw primitive values of a [`Chunk`], for a given component.
    ///
    /// This is a very fast path: the entire column will be downcasted at once, and then every
    /// component batch will be a slice reference into that global slice.
    /// Use this when working with simple arrow datatypes and performance matters (e.g. scalars,
    /// points, etc).
    ///
    /// * [`Self::iter_component_arrays`].
    /// * [`Self::iter_component`].
    #[inline]
    pub fn iter_primitive<T: arrow2::types::NativeType>(
        &self,
        component_name: &ComponentName,
    ) -> impl Iterator<Item = &[T]> + '_ {
        let Some(list_array) = self.components.get(component_name) else {
            return Either::Left(std::iter::empty());
        };

        let Some(values) = list_array
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<T>>()
        else {
            if cfg!(debug_assertions) {
                panic!("downcast failed for {component_name}, data discarded");
            } else {
                re_log::error_once!("downcast failed for {component_name}, data discarded");
            }
            return Either::Left(std::iter::empty());
        };
        let values = values.values().as_slice();

        // NOTE: No need for validity checks here, `iter_offsets` already takes care of that.
        Either::Right(
            self.iter_component_offsets(component_name)
                .map(move |(idx, len)| &values[idx..idx + len]),
        )
    }
}

// ---

pub struct ChunkIndicesIter {
    chunk: Arc<Chunk>,

    time_chunk: Option<ChunkTimeline>,
    index: usize,
}

impl Iterator for ChunkIndicesIter {
    type Item = (TimeInt, RowId);

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;
        self.index += 1;

        let row_id = {
            let (times, incs) = self.chunk.row_ids_raw();
            let times = times.values().as_slice();
            let incs = incs.values().as_slice();

            let time = *times.get(i)?;
            let inc = *incs.get(i)?;

            RowId::from_u128(((time as u128) << 64) | (inc as u128))
        };

        if let Some(time_chunk) = &self.time_chunk {
            let time = *time_chunk.times_raw().get(i)?;
            let time = TimeInt::new_temporal(time);
            Some((time, row_id))
        } else {
            Some((TimeInt::STATIC, row_id))
        }
    }
}

impl Chunk {
    /// Returns an iterator over the indices (`(TimeInt, RowId)`) of a [`Chunk`], for a given timeline.
    ///
    /// If the chunk is static, `timeline` will be ignored.
    ///
    /// The returned iterator outlives `self`, thus it can be passed around freely.
    /// The tradeoff is that `self` must be an `Arc`.
    ///
    /// See also [`Self::iter_indices`].
    #[inline]
    pub fn iter_indices_owned(
        self: Arc<Self>,
        timeline: &Timeline,
    ) -> impl Iterator<Item = (TimeInt, RowId)> {
        if self.is_static() {
            Either::Left(ChunkIndicesIter {
                chunk: self,
                time_chunk: None,
                index: 0,
            })
        } else {
            self.timelines.get(timeline).cloned().map_or_else(
                || Either::Right(Either::Left(std::iter::empty())),
                |time_chunk| {
                    Either::Right(Either::Right(ChunkIndicesIter {
                        chunk: self,
                        time_chunk: Some(time_chunk),
                        index: 0,
                    }))
                },
            )
        }
    }
}

// ---

/// The actual iterator implementation for [`Chunk::iter_component`].
pub struct ChunkComponentIter<C, IO> {
    values: Vec<C>,
    offsets: IO,
}

/// The intermediate state for [`ChunkComponentIter`].
///
/// Required so that we can return references to the inner data.
pub struct ChunkComponentIterRef<'a, C, IO> {
    values: &'a [C],
    offsets: &'a mut IO,
}

impl<'a, C: Component, IO: Iterator<Item = (usize, usize)>> IntoIterator
    for &'a mut ChunkComponentIter<C, IO>
{
    type Item = &'a [C];

    type IntoIter = ChunkComponentIterRef<'a, C, IO>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ChunkComponentIterRef {
            values: &self.values,
            offsets: &mut self.offsets,
        }
    }
}

impl<'a, C: Component, IO: Iterator<Item = (usize, usize)>> Iterator
    for ChunkComponentIterRef<'a, C, IO>
{
    type Item = &'a [C];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.offsets
            .next()
            .map(move |(idx, len)| &self.values[idx..idx + len])
    }
}

impl Chunk {
    /// Returns an iterator over the deserialized batches of a [`Chunk`], for a given component.
    ///
    /// This is a dedicated fast path: the entire column will be downcasted and deserialized at
    /// once, and then every component batch will be a slice reference into that global slice.
    /// Use this when working with complex arrow datatypes and performance matters (e.g. ranging
    /// through enum types across many timestamps).
    ///
    /// See also:
    /// * [`Self::iter_component`].
    /// * [`Self::iter_primitive`].
    #[inline]
    pub fn iter_component<C: Component>(
        &self,
    ) -> ChunkComponentIter<C, impl Iterator<Item = (usize, usize)> + '_> {
        let Some(list_array) = self.components.get(&C::name()) else {
            return ChunkComponentIter {
                values: vec![],
                offsets: Either::Left(std::iter::empty()),
            };
        };

        let values = list_array.values();
        let values = match C::from_arrow(&**values) {
            Ok(values) => values,
            Err(err) => {
                if cfg!(debug_assertions) {
                    panic!(
                        "deserialization failed for {}, data discarded: {}",
                        C::name(),
                        re_error::format_ref(&err),
                    );
                } else {
                    re_log::error_once!(
                        "deserialization failed for {}, data discarded: {}",
                        C::name(),
                        re_error::format_ref(&err),
                    );
                }
                return ChunkComponentIter {
                    values: vec![],
                    offsets: Either::Left(std::iter::empty()),
                };
            }
        };

        // NOTE: No need for validity checks here, `iter_offsets` already takes care of that.
        ChunkComponentIter {
            values,
            offsets: Either::Right(self.iter_component_offsets(&C::name())),
        }
    }
}

// ---

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use itertools::{izip, Itertools};
    use re_log_types::{example_components::MyPoint, EntityPath, TimeInt, TimePoint};

    use crate::{Chunk, RowId, Timeline};

    #[test]
    fn iter_indices_temporal() -> anyhow::Result<()> {
        let entity_path = EntityPath::from("this/that");

        let row_id1 = RowId::new();
        let row_id2 = RowId::new();
        let row_id3 = RowId::new();
        let row_id4 = RowId::new();
        let row_id5 = RowId::new();

        let timeline_frame = Timeline::new_sequence("frame");

        let timepoint1 = [(timeline_frame, 1)];
        let timepoint2 = [(timeline_frame, 3)];
        let timepoint3 = [(timeline_frame, 5)];
        let timepoint4 = [(timeline_frame, 7)];
        let timepoint5 = [(timeline_frame, 9)];

        let points1 = &[MyPoint::new(1.0, 1.0)];
        let points2 = &[MyPoint::new(2.0, 2.0)];
        let points3 = &[MyPoint::new(3.0, 3.0)];
        let points4 = &[MyPoint::new(4.0, 4.0)];
        let points5 = &[MyPoint::new(5.0, 5.0)];

        let chunk = Arc::new(
            Chunk::builder(entity_path.clone())
                .with_component_batches(row_id1, timepoint1, [points1 as _])
                .with_component_batches(row_id2, timepoint2, [points2 as _])
                .with_component_batches(row_id3, timepoint3, [points3 as _])
                .with_component_batches(row_id4, timepoint4, [points4 as _])
                .with_component_batches(row_id5, timepoint5, [points5 as _])
                .build()?,
        );

        {
            let got = Arc::clone(&chunk)
                .iter_indices_owned(&timeline_frame)
                .collect_vec();
            let expected = izip!(
                chunk
                    .timelines
                    .get(&timeline_frame)
                    .map(|time_chunk| time_chunk.times().collect_vec())
                    .unwrap_or_default(),
                chunk.row_ids()
            )
            .collect_vec();

            similar_asserts::assert_eq!(expected, got);
        }

        Ok(())
    }

    #[test]
    fn iter_indices_static() -> anyhow::Result<()> {
        let entity_path = EntityPath::from("this/that");

        let row_id1 = RowId::new();
        let row_id2 = RowId::new();
        let row_id3 = RowId::new();
        let row_id4 = RowId::new();
        let row_id5 = RowId::new();

        let timeline_frame = Timeline::new_sequence("frame");

        let points1 = &[MyPoint::new(1.0, 1.0)];
        let points2 = &[MyPoint::new(2.0, 2.0)];
        let points3 = &[MyPoint::new(3.0, 3.0)];
        let points4 = &[MyPoint::new(4.0, 4.0)];
        let points5 = &[MyPoint::new(5.0, 5.0)];

        let chunk = Arc::new(
            Chunk::builder(entity_path.clone())
                .with_component_batches(row_id1, TimePoint::default(), [points1 as _])
                .with_component_batches(row_id2, TimePoint::default(), [points2 as _])
                .with_component_batches(row_id3, TimePoint::default(), [points3 as _])
                .with_component_batches(row_id4, TimePoint::default(), [points4 as _])
                .with_component_batches(row_id5, TimePoint::default(), [points5 as _])
                .build()?,
        );

        {
            let got = Arc::clone(&chunk)
                .iter_indices_owned(&timeline_frame)
                .collect_vec();
            let expected = izip!(std::iter::repeat(TimeInt::STATIC), chunk.row_ids()).collect_vec();

            similar_asserts::assert_eq!(expected, got);
        }

        Ok(())
    }
}
