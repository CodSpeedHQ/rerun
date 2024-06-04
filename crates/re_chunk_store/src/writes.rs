use std::sync::Arc;

use arrow2::array::{Array as _, ListArray as ArrowListArray};

use re_chunk::{Chunk, RowId};

use crate::{
    ChunkStore, ChunkStoreChunkStats, ChunkStoreDiff, ChunkStoreDiffKind, ChunkStoreError,
    ChunkStoreEvent, ChunkStoreResult,
};

// ---

// TODO: both duplicated chunks and duplicated rows are effectively UB
// -> chunks are considered idempotent and such not reinserted
// -> rows are fully UB since we dont even look at them

impl ChunkStore {
    // TODO: document why this returns a vec
    pub fn insert_chunk(
        &mut self,
        chunk: &Arc<Chunk>,
    ) -> ChunkStoreResult<Option<ChunkStoreEvent>> {
        if self.chunks_per_chunk_id.contains_key(&chunk.id()) {
            // We assume that chunk IDs are unique, and that reinserting a chunk has no effect.
            re_log::warn_once!(
                "Chunk #{} was inserted more than once (this has no effect)",
                chunk.id()
            );
            return Ok(None);
        }

        if !chunk.is_sorted() {
            return Err(ChunkStoreError::UnsortedChunk);
        }

        re_tracing::profile_function!(format!("{}", chunk.row_id_range().0));

        self.insert_id += 1;

        let row_id_range = chunk.row_id_range();
        let row_id_min = row_id_range.0;
        let row_id_max = row_id_range.1;

        self.chunks_per_chunk_id.insert(chunk.id(), chunk.clone());
        self.chunk_id_per_min_row_id.insert(row_id_min, chunk.id());

        if chunk.is_static() {
            for component_name in chunk.component_names() {
                // TODO: explain
                self.static_chunk_ids_per_entity
                    .entry(chunk.entity_path().clone())
                    .or_default()
                    .entry(component_name)
                    .and_modify(|cur_chunk_id| {
                        let cur_row_id_max = self
                            .chunks_per_chunk_id
                            .get(cur_chunk_id)
                            .map_or(RowId::ZERO, |chunk| chunk.row_id_range().1);
                        if row_id_max > cur_row_id_max {
                            *cur_chunk_id = chunk.id();
                        }
                    })
                    .or_insert_with(|| chunk.id());
            }

            self.static_chunks_stats += ChunkStoreChunkStats::from_chunk(chunk);
        } else {
            // TODO: it's fine, really -- just index on everything, who cares

            let temporal_chunk_ids_per_component = self
                .temporal_chunk_ids_per_entity
                .entry(chunk.entity_path().clone())
                .or_default();

            for component_name in chunk.component_names() {
                let temporal_chunk_ids_per_timeline = temporal_chunk_ids_per_component
                    .entry(component_name)
                    .or_default();

                for (&timeline, time_chunk) in chunk.timelines() {
                    let temporal_chunk_ids_per_time =
                        temporal_chunk_ids_per_timeline.entry(timeline).or_default();

                    let time_range = time_chunk.time_range();
                    temporal_chunk_ids_per_time
                        .per_start_time
                        .entry(time_range.min())
                        .or_default()
                        .insert(chunk.id());
                    temporal_chunk_ids_per_time
                        .per_end_time
                        .entry(time_range.max())
                        .or_default()
                        .insert(chunk.id());
                }
            }

            self.temporal_chunks_stats += ChunkStoreChunkStats::from_chunk(chunk);
        }

        for (&component_name, list_array) in chunk.components() {
            self.type_registry.insert(
                component_name,
                ArrowListArray::<i32>::get_child_type(list_array.data_type()).clone(),
            );
        }

        let event = ChunkStoreEvent {
            store_id: self.id.clone(),
            store_generation: self.generation(),
            event_id: self
                .event_id
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            diff: ChunkStoreDiff::addition(chunk.clone()),
        };

        {
            let events = &[event.clone()];

            if cfg!(debug_assertions) {
                let any_event_other_than_addition = events
                    .iter()
                    .any(|e| e.kind != ChunkStoreDiffKind::Addition);
                assert!(!any_event_other_than_addition);
            }

            Self::on_events(events);
        }

        Ok(Some(event))
    }
}
