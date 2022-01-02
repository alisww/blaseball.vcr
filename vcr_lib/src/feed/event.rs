use super::{metadata as feed_metadata, EventDescription};
use crate::utils::encode_varint;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FeedEvent {
    pub id: Uuid,
    pub category: i8,
    pub created: DateTime<Utc>,
    pub day: i16,
    pub description: String,
    #[serde(default)]
    pub nuts: u16,
    pub phase: u8,
    pub player_tags: Option<Vec<Uuid>>,
    pub game_tags: Option<Vec<Uuid>>,
    pub team_tags: Option<Vec<Uuid>>,
    #[serde(rename = "type")]
    pub etype: i16,
    pub tournament: i8,
    pub season: u8,
    #[serde(default)]
    pub metadata: JSONValue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompactedFeedEvent {
    pub id: Uuid,
    pub created: DateTime<Utc>,
    pub category: i8,
    pub day: u8,
    pub description: String,
    #[serde(default)]
    pub player_tags: Vec<u16>,
    pub game_tags: Vec<u16>,
    pub team_tags: Vec<u8>,
    #[serde(rename = "type")]
    pub etype: i16,
    pub tournament: i8,
    #[serde(default)]
    pub metadata: JSONValue,
    pub season: u8,
    pub phase: u8,
}

impl FeedEvent {
    pub fn generate_id(&self, millis_epoch: Option<u32>) -> Vec<u8> {
        let timestamp = match millis_epoch {
            Some(epoch) => {
                let epoch = (epoch as i64) * 1000;
                (self.created.timestamp_millis() - epoch) as u32
            }
            None => self.created.timestamp() as u32,
        };
        [
            self.season.to_be_bytes().to_vec(),
            self.phase.to_be_bytes().to_vec(),
            timestamp.to_be_bytes().to_vec(),
            self.id.as_bytes()[0..2].to_vec(),
        ]
        .concat()
    }
}

impl CompactedFeedEvent {
    pub fn encode(&self) -> Vec<u8> {
        let player_tag_bytes: Vec<u8> = {
            let player_tags: Vec<u8> = self
                .player_tags
                .iter()
                .flat_map(|id| id.to_be_bytes())
                .collect();
            [
                (self.player_tags.len() as u8).to_be_bytes().to_vec(),
                player_tags,
            ]
            .concat()
        };

        let team_tag_bytes: Vec<u8> = {
            let team_tags: Vec<u8> = self
                .team_tags
                .iter()
                .flat_map(|id| id.to_be_bytes())
                .collect();
            [
                (self.team_tags.len() as u8).to_be_bytes().to_vec(),
                team_tags,
            ]
            .concat()
        };

        let game_tag_bytes: Vec<u8> = {
            let game_tags: Vec<u8> = self
                .game_tags
                .iter()
                .flat_map(|id| id.to_be_bytes())
                .collect();
            [
                (self.game_tags.len() as u8).to_be_bytes().to_vec(),
                game_tags,
            ]
            .concat()
        };

        let description_bytes = {
            match EventDescription::from_type(self.etype) {
                EventDescription::Constant(s) => {
                    assert_eq!(self.description, s);
                    vec![]
                }
                EventDescription::ConstantVariant(possibilities) => {
                    vec![(possibilities
                        .iter()
                        .position(|&d| d == self.description)
                        .unwrap_or_else(|| panic!("{}", self.etype))
                        as u8)
                        .to_be()]
                }
                EventDescription::ConstantMiddle(m) => {
                    let (start, end) = self.description.split_once(m).unwrap();
                    let (start, end) = (start.as_bytes().to_vec(), end.as_bytes().to_vec());

                    [
                        encode_varint(start.len() as u16),
                        encode_varint(end.len() as u16),
                        start,
                        end,
                    ]
                    .concat()
                }
                EventDescription::VariableMiddle(possible_m) => {
                    match possible_m
                        .iter()
                        .enumerate()
                        .filter_map(|(i, m)| {
                            let maybe = self.description.split_once(m);
                            if let Some((start, end)) = maybe {
                                let (start, end) =
                                    (start.as_bytes().to_vec(), end.as_bytes().to_vec());
                                Some(
                                    [
                                        (i as u8).to_be_bytes().to_vec(),
                                        encode_varint(start.len() as u16),
                                        encode_varint(end.len() as u16),
                                        start,
                                        end,
                                    ]
                                    .concat(),
                                )
                            } else {
                                None
                            }
                        })
                        .next()
                    {
                        Some(bytes) => bytes,
                        None => {
                            let bytes = self.description.as_bytes().to_vec();
                            [
                                ((possible_m.len() + 1) as u8).to_be_bytes().to_vec(),
                                encode_varint(bytes.len() as u16),
                                bytes,
                            ]
                            .concat()
                        }
                    }
                }
                EventDescription::VariablePrefix(possible_pfx) => {
                    match possible_pfx
                        .iter()
                        .enumerate()
                        .filter_map(|(i, pfx)| {
                            let maybe = self.description.strip_prefix(pfx);
                            if let Some(rest) = maybe {
                                let rest = rest.as_bytes().to_vec();
                                Some(
                                    [
                                        (i as u8).to_be_bytes().to_vec(),
                                        encode_varint(rest.len() as u16),
                                        rest,
                                    ]
                                    .concat(),
                                )
                            } else {
                                None
                            }
                        })
                        .next()
                    {
                        Some(bytes) => bytes,
                        None => {
                            let bytes = self.description.as_bytes().to_vec();
                            [
                                ((possible_pfx.len() + 1) as u8).to_be_bytes().to_vec(),
                                encode_varint(bytes.len() as u16),
                                bytes,
                            ]
                            .concat()
                        }
                    }
                }
                EventDescription::VariableSuffix(possible_sfx) => {
                    match possible_sfx
                        .iter()
                        .enumerate()
                        .filter_map(|(i, sfx)| {
                            let maybe = self.description.strip_suffix(sfx);
                            if let Some(rest) = maybe {
                                let rest = rest.as_bytes().to_vec();
                                Some(
                                    [
                                        (i as u8).to_be_bytes().to_vec(),
                                        encode_varint(rest.len() as u16),
                                        rest,
                                    ]
                                    .concat(),
                                )
                            } else {
                                None
                            }
                        })
                        .next()
                    {
                        Some(bytes) => bytes,
                        None => {
                            let bytes = self.description.as_bytes().to_vec();
                            [
                                ((possible_sfx.len() + 1) as u8).to_be_bytes().to_vec(),
                                encode_varint(bytes.len() as u16),
                                bytes,
                            ]
                            .concat()
                        }
                    }
                }
                EventDescription::Suffix(s) => {
                    let description = self
                        .description
                        .strip_suffix(s)
                        .unwrap()
                        .as_bytes()
                        .to_vec();
                    [encode_varint(description.len() as u16), description].concat()
                }
                EventDescription::Prefix(s) => {
                    let description = self
                        .description
                        .strip_prefix(s)
                        .unwrap()
                        .as_bytes()
                        .to_vec();
                    [encode_varint(description.len() as u16), description].concat()
                }
                EventDescription::Variable => {
                    let description = self.description.as_bytes().to_vec();
                    [encode_varint(description.len() as u16), description].concat()
                }
            }
        };

        // println!(
        //     "type: {}, tags: {}, description: {}",
        //     self.etype,
        //     team_tag_bytes.len() + game_tag_bytes.len() + player_tag_bytes.len(),
        //     description_bytes.len()
        // );

        [
            self.category.to_be_bytes().to_vec(), // 1 byte
            self.etype.to_be_bytes().to_vec(),    // 2 bytes
            self.day.to_be_bytes().to_vec(),      // 1 byte
            ((self.season - 10) | (self.phase << 4)) // 1 byte
                .to_be_bytes()
                .to_vec(),
            if self.phase == 13 {
                self.id.as_bytes().to_vec() // 16 bytes or none
            } else {
                vec![]
            },
            description_bytes, // <length> bytes
            player_tag_bytes,  // n * 2 bytes
            team_tag_bytes,    // n bytes
            game_tag_bytes,    // n * 2 bytes
            feed_metadata::encode_metadata(self.etype, &self.metadata), // usually 3 bytes
        ]
        .concat()
    }

    pub fn encode_stats(&self) -> (usize, usize, usize) {
        let player_tag_bytes: Vec<u8> = {
            let player_tags: Vec<u8> = self
                .player_tags
                .iter()
                .flat_map(|id| id.to_be_bytes())
                .collect();
            [
                (self.player_tags.len() as u8).to_be_bytes().to_vec(),
                player_tags,
            ]
            .concat()
        };

        let team_tag_bytes: Vec<u8> = {
            let team_tags: Vec<u8> = self
                .team_tags
                .iter()
                .flat_map(|id| id.to_be_bytes())
                .collect();
            [
                (self.team_tags.len() as u8).to_be_bytes().to_vec(),
                team_tags,
            ]
            .concat()
        };

        let game_tag_bytes: Vec<u8> = {
            let game_tags: Vec<u8> = self
                .game_tags
                .iter()
                .flat_map(|id| id.to_be_bytes())
                .collect();
            [
                (self.game_tags.len() as u8).to_be_bytes().to_vec(),
                game_tags,
            ]
            .concat()
        };

        let description_bytes = {
            match EventDescription::from_type(self.etype) {
                EventDescription::Constant(s) => {
                    assert_eq!(self.description, s);
                    vec![]
                }
                EventDescription::ConstantVariant(possibilities) => {
                    vec![(possibilities
                        .iter()
                        .position(|&d| d == self.description)
                        .unwrap_or_else(|| panic!("{}", self.etype))
                        as u8)
                        .to_be()]
                }
                EventDescription::ConstantMiddle(m) => {
                    let (start, end) = self.description.split_once(m).unwrap();
                    let (start, end) = (start.as_bytes().to_vec(), end.as_bytes().to_vec());

                    [
                        encode_varint(start.len() as u16),
                        encode_varint(end.len() as u16),
                        start,
                        end,
                    ]
                    .concat()
                }
                EventDescription::VariableMiddle(possible_m) => {
                    match possible_m
                        .iter()
                        .enumerate()
                        .filter_map(|(i, m)| {
                            let maybe = self.description.split_once(m);
                            if let Some((start, end)) = maybe {
                                let (start, end) =
                                    (start.as_bytes().to_vec(), end.as_bytes().to_vec());
                                Some(
                                    [
                                        (i as u8).to_be_bytes().to_vec(),
                                        encode_varint(start.len() as u16),
                                        encode_varint(end.len() as u16),
                                        start,
                                        end,
                                    ]
                                    .concat(),
                                )
                            } else {
                                None
                            }
                        })
                        .next()
                    {
                        Some(bytes) => bytes,
                        None => {
                            let bytes = self.description.as_bytes().to_vec();
                            [
                                ((possible_m.len() + 1) as u8).to_be_bytes().to_vec(),
                                encode_varint(bytes.len() as u16),
                                bytes,
                            ]
                            .concat()
                        }
                    }
                }
                EventDescription::VariablePrefix(possible_pfx) => {
                    match possible_pfx
                        .iter()
                        .enumerate()
                        .filter_map(|(i, pfx)| {
                            let maybe = self.description.strip_prefix(pfx);
                            if let Some(rest) = maybe {
                                let rest = rest.as_bytes().to_vec();
                                Some(
                                    [
                                        (i as u8).to_be_bytes().to_vec(),
                                        encode_varint(rest.len() as u16),
                                        rest,
                                    ]
                                    .concat(),
                                )
                            } else {
                                None
                            }
                        })
                        .next()
                    {
                        Some(bytes) => bytes,
                        None => {
                            let bytes = self.description.as_bytes().to_vec();
                            [
                                ((possible_pfx.len() + 1) as u8).to_be_bytes().to_vec(),
                                encode_varint(bytes.len() as u16),
                                bytes,
                            ]
                            .concat()
                        }
                    }
                }
                EventDescription::VariableSuffix(possible_sfx) => {
                    match possible_sfx
                        .iter()
                        .enumerate()
                        .filter_map(|(i, sfx)| {
                            let maybe = self.description.strip_suffix(sfx);
                            if let Some(rest) = maybe {
                                let rest = rest.as_bytes().to_vec();
                                Some(
                                    [
                                        (i as u8).to_be_bytes().to_vec(),
                                        encode_varint(rest.len() as u16),
                                        rest,
                                    ]
                                    .concat(),
                                )
                            } else {
                                None
                            }
                        })
                        .next()
                    {
                        Some(bytes) => bytes,
                        None => {
                            let bytes = self.description.as_bytes().to_vec();
                            [
                                ((possible_sfx.len() + 1) as u8).to_be_bytes().to_vec(),
                                encode_varint(bytes.len() as u16),
                                bytes,
                            ]
                            .concat()
                        }
                    }
                }
                EventDescription::Suffix(s) => {
                    let description = self
                        .description
                        .strip_suffix(s)
                        .unwrap()
                        .as_bytes()
                        .to_vec();
                    [encode_varint(description.len() as u16), description].concat()
                }
                EventDescription::Prefix(s) => {
                    let description = self
                        .description
                        .strip_prefix(s)
                        .unwrap()
                        .as_bytes()
                        .to_vec();
                    [encode_varint(description.len() as u16), description].concat()
                }
                EventDescription::Variable => {
                    let description = self.description.as_bytes().to_vec();
                    [encode_varint(description.len() as u16), description].concat()
                }
            }
        };

        (
            feed_metadata::encode_metadata(self.etype, &self.metadata).len(),
            description_bytes.len(),
            player_tag_bytes.len() + team_tag_bytes.len() + game_tag_bytes.len(),
        )
    }
}
