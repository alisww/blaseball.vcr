// #[derive(Serialize, Deserialize)]
// pub struct EncodedBlockHeader {
//     pub compressed_len: u32,
//     pub decompressed_len: u32,
//     pub start_time: i64,
//     pub metadata: BlockMetadata,
//     pub event_positions: Vec<(i64, u32)>,
// }
// #[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
// pub struct BlockMetadata {
//     pub tournament: i8,
//     pub season: i8,
//     pub phase: u8,
//     // pub day: u8, //            day: ev.day.try_into().unwrap_or(255),
// }

use tsz_compress::{
    compress::Compressor,
    prelude::{bv::BitVec, *},
};

use crate::feed::BlockMetadata;

use super::EncodedBlockHeader;
#[derive(DeltaEncodable, Compressible, Decompressible, Clone, Copy, Debug)]
pub struct BlockRow {
    pub starts_at_event: i64,
    pub events_len: i64,
    pub compressed_len: i32,
    pub decompressed_len: i32,
    pub tournament: i8,
    pub season: i8,
    pub phase: i8,
}

#[derive(DeltaEncodable, Compressible, Decompressible, Clone, Copy, Debug)]
pub struct EventRow {
    pub time: i64,
    pub position: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PackedHeader {
    blocks: BitVec<u8>,
    events: BitVec<u8>,
}

impl PackedHeader {
    pub fn encode(headers: Vec<EncodedBlockHeader>) -> PackedHeader {
        let mut block_compressor = Compressor::new(1000);
        let mut event_compressor = Compressor::new(1000);
        let mut event = 0;

        for EncodedBlockHeader {
            compressed_len,
            decompressed_len,
            metadata,
            event_positions,
            ..
        } in headers
        {
            block_compressor.compress(BlockRow {
                starts_at_event: event,
                events_len: event_positions.len() as i64,
                compressed_len: compressed_len as i32,
                decompressed_len: decompressed_len as i32,
                tournament: metadata.tournament,
                season: metadata.season,
                phase: metadata.phase.try_into().unwrap(),
            });

            for (event_time, event_pos) in event_positions {
                event_compressor.compress(EventRow {
                    time: event_time,
                    position: event_pos as i32,
                });

                event += 1;
            }
        }

        PackedHeader {
            blocks: block_compressor.finish(),
            events: event_compressor.finish(),
        }
    }

    pub fn decode(self) -> Vec<EncodedBlockHeader> {
        let events = Decompressor::new(&self.events)
            .decompress()
            .collect::<Result<Vec<EventRow>, _>>()
            .unwrap();

        let mut blocks = Decompressor::new(&self.blocks);

        blocks
            .decompress::<BlockRow>()
            .map(|block| {
                let BlockRow {
                    starts_at_event,
                    events_len,
                    compressed_len,
                    decompressed_len,
                    tournament,
                    season,
                    phase,
                } = block.unwrap();
                let block_events = events
                    [starts_at_event as usize..(starts_at_event as usize) + (events_len as usize)]
                    .iter()
                    .map(|EventRow { time, position }| (*time, *position as u32))
                    .collect::<Vec<_>>();
                EncodedBlockHeader {
                    compressed_len: compressed_len as u32,
                    decompressed_len: decompressed_len as u32,
                    start_time: block_events[0].0,
                    metadata: BlockMetadata {
                        tournament,
                        season,
                        phase: phase as u8,
                    },
                    event_positions: block_events,
                }
            })
            .collect()
    }
}
