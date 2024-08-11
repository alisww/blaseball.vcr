use std::io;

pub fn encode_varint(i: u16) -> Vec<u8> {
    if i >= 255 {
        vec![255_u8.to_be_bytes().to_vec(), i.to_be_bytes().to_vec()].concat()
    } else {
        (i as u8).to_be_bytes().to_vec()
    }
}

#[macro_export]
macro_rules! decode_varint {
    ($read:expr) => {{
        let mut first_byte: [u8; 1] = [0; 1];
        $read.read_exact(&mut first_byte)?;
        let length_byte = u8::from_be_bytes(first_byte);
        if length_byte == 255 {
            let mut longer_bytes: [u8; 2] = [0; 2];
            $read.read_exact(&mut longer_bytes)?;
            u16::from_be_bytes(longer_bytes)
        } else {
            length_byte as u16
        }
    }};
}

#[macro_export]
macro_rules! read_u8 {
    ($read:expr) => {{
        let mut byte: [u8; 1] = [0; 1];
        $read.read_exact(&mut byte)?;
        u8::from_be_bytes(byte)
    }};
}

#[macro_export]
macro_rules! read_i8 {
    ($read:expr) => {{
        let mut byte: [u8; 1] = [0; 1];
        $read.read_exact(&mut byte)?;
        i8::from_be_bytes(byte)
    }};
}

#[macro_export]
macro_rules! read_u16 {
    ($read:expr) => {{
        let mut bytes: [u8; 2] = [0; 2];
        $read.read_exact(&mut bytes)?;
        u16::from_be_bytes(bytes)
    }};
}

#[macro_export]
macro_rules! read_i16 {
    ($read:expr) => {{
        let mut bytes: [u8; 2] = [0; 2];
        $read.read_exact(&mut bytes)?;
        i16::from_be_bytes(bytes)
    }};
}

#[macro_export]
macro_rules! read_u32 {
    ($read:expr) => {{
        let mut bytes: [u8; 4] = [0; 4];
        $read.read_exact(&mut bytes)?;
        u32::from_be_bytes(bytes)
    }};
}

#[macro_export]
macro_rules! read_i64 {
    ($read:expr) => {{
        let mut bytes: [u8; 8] = [0; 8];
        $read.read_exact(&mut bytes)?;
        i64::from_be_bytes(bytes)
    }};
}

pub fn is_eof<T>(err: &io::Result<T>) -> bool {
    match err {
        Ok(_) => false,
        Err(e) => e.kind() == io::ErrorKind::UnexpectedEof,
    }
}

#[macro_export]
macro_rules! call_method_by_type {
    ($obj:ident $sep:tt $method_name:ident, $args:tt, $ty:expr, $last_case:block) => {
        match $ty {
            "gameupdate" => $obj$sep$method_name::<GameUpdate>$args,
            "bossfight" => $obj$sep$method_name::<Bossfight>$args,
            "communitychestprogress" => $obj$sep$method_name::<CommunityChestProgress>$args,
            "division" => $obj$sep$method_name::<Division>$args,
            "league" => $obj$sep$method_name::<League>$args,
            "playoffmatchup" => $obj$sep$method_name::<Playoffmatchup>$args,
            "playoffround" => $obj$sep$method_name::<Playoffround>$args,
            "playoffs" => $obj$sep$method_name::<Playoffs>$args,
            "season" => $obj$sep$method_name::<Season>$args,
            "sim" => $obj$sep$method_name::<Sim>$args,
            "stadium" => $obj$sep$method_name::<Stadium>$args,
            "standings" => $obj$sep$method_name::<Standings>$args,
            "subleague" => $obj$sep$method_name::<Subleague>$args,
            "team" => $obj$sep$method_name::<Team>$args,
            "sunsun" => $obj$sep$method_name::<Sunsun>$args,
            "temporal" => $obj$sep$method_name::<Temporal>$args,
            "tiebreakers" => $obj$sep$method_name::<Tiebreakers>$args,
            "tournament" => $obj$sep$method_name::<Tournament>$args,
            "bonusresult" => $obj$sep$method_name::<Bonusresult>$args,
            "decreeresult" => $obj$sep$method_name::<Decreeresult>$args,
            "eventresult" => $obj$sep$method_name::<Eventresult>$args,
            "fuelprogress" => $obj$sep$method_name::<FuelProgressWrapper>$args,
            "giftprogress" => $obj$sep$method_name::<Giftprogress>$args,
            "globalevents" => $obj$sep$method_name::<GlobaleventsWrapper>$args,
            "idols" => $obj$sep$method_name::<Idols>$args,
            "item" => $obj$sep$method_name::<Item>$args,
            "librarystory" => $obj$sep$method_name::<LibrarystoryWrapper>$args,
            "nullified" => $obj$sep$method_name::<Nullified>$args,
            "offseasonrecap" => $obj$sep$method_name::<Offseasonrecap>$args,
            "offseasonsetup" => $obj$sep$method_name::<Offseasonsetup>$args,
            "player" => $obj$sep$method_name::<Player>$args,
            "renovationprogress" => $obj$sep$method_name::<Renovationprogress>$args,
            "risingstars" => $obj$sep$method_name::<Risingstars>$args,
            "shopsetup" => $obj$sep$method_name::<Shopsetup>$args,
            "teamelectionstats" => $obj$sep$method_name::<Teamelectionstats>$args,
            "vault" => $obj$sep$method_name::<Vault>$args,
            "stadiumprefabs" => $obj$sep$method_name::<Stadiumprefabs>$args,
            "thebook" => $obj$sep$method_name::<Thebook>$args,
            "thebeat" => $obj$sep$method_name::<Thebeat>$args,
            "teamstatsheet" => $obj$sep$method_name::<Teamstatsheet>$args,
            "glossarywords" => $obj$sep$method_name::<Glossarywords>$args,
            "peanutpower" => $obj$sep$method_name::<Peanutpower>$args,
            "gammasim" => $obj$sep$method_name::<Gammasim>$args,
            "gammaelections" => $obj$sep$method_name::<Gammaelections>$args,
            "gammaelectionresults" => $obj$sep$method_name::<Gammaelectionresults>$args,
            "gammaelectiondetails" => $obj$sep$method_name::<Gammaelectiondetails>$args,
            "gammaelection" => $obj$sep$method_name::<Gammaelection>$args,
            "gammabracket" => $obj$sep$method_name::<Gammabracket>$args,
            "gamestatsheet" => $obj$sep$method_name::<Gamestatsheet>$args,
            "feedseasonlist" => $obj$sep$method_name::<Feedseasonlist>$args,
            "fanart" => $obj$sep$method_name::<Fanart>$args,
            "dayssincelastincineration" => $obj$sep$method_name::<Dayssincelastincineration>$args,
            "championcallout" => $obj$sep$method_name::<Championcallout>$args,
            "availablechampionbets" => $obj$sep$method_name::<Availablechampionbets>$args,
            "attributes" => $obj$sep$method_name::<Attributes>$args,
            "playerstatsheet" => $obj$sep$method_name::<Playerstatsheet>$args,
            // "stream" => $obj$sep$method_name::<StreamEntityWrapper<thisidisstaticyo::StreamData>>$args,
            _ => $last_case
        }
    }
}

#[macro_export]
macro_rules! call_method_by_type_with_custom_impls {
    ($obj:ident $sep:tt $method_name:ident, $args:tt, $ty:expr, $last_case:block) => {
        match $ty {
            "gameupdate" => $obj$sep$method_name::<GameUpdate>$args,
            "bossfight" => $obj$sep$method_name::<Bossfight>$args,
            "communitychestprogress" => $obj$sep$method_name::<CommunityChestProgress>$args,
            "division" => $obj$sep$method_name::<Division>$args,
            "league" => $obj$sep$method_name::<League>$args,
            "playoffmatchup" => $obj$sep$method_name::<Playoffmatchup>$args,
            "playoffround" => $obj$sep$method_name::<Playoffround>$args,
            "playoffs" => $obj$sep$method_name::<Playoffs>$args,
            "season" => $obj$sep$method_name::<Season>$args,
            "sim" => $obj$sep$method_name::<Sim>$args,
            "stadium" => $obj$sep$method_name::<Stadium>$args,
            "standings" => $obj$sep$method_name::<Standings>$args,
            "subleague" => $obj$sep$method_name::<Subleague>$args,
            "team" => $obj$sep$method_name::<Team>$args,
            "sunsun" => $obj$sep$method_name::<Sunsun>$args,
            "temporal" => $obj$sep$method_name::<Temporal>$args,
            "tiebreakers" => $obj$sep$method_name::<Tiebreakers>$args,
            "tournament" => $obj$sep$method_name::<Tournament>$args,
            "bonusresult" => $obj$sep$method_name::<Bonusresult>$args,
            "decreeresult" => $obj$sep$method_name::<Decreeresult>$args,
            "eventresult" => $obj$sep$method_name::<Eventresult>$args,
            "fuelprogress" => $obj$sep$method_name::<FuelProgressWrapper>$args,
            "giftprogress" => $obj$sep$method_name::<Giftprogress>$args,
            "globalevents" => $obj$sep$method_name::<GlobaleventsWrapper>$args,
            "idols" => $obj$sep$method_name::<Idols>$args,
            "item" => $obj$sep$method_name::<Item>$args,
            "librarystory" => $obj$sep$method_name::<LibrarystoryWrapper>$args,
            "nullified" => $obj$sep$method_name::<Nullified>$args,
            "offseasonrecap" => $obj$sep$method_name::<Offseasonrecap>$args,
            "offseasonsetup" => $obj$sep$method_name::<Offseasonsetup>$args,
            "player" => $obj$sep$method_name::<Player>$args,
            "renovationprogress" => $obj$sep$method_name::<Renovationprogress>$args,
            "risingstars" => $obj$sep$method_name::<Risingstars>$args,
            "shopsetup" => $obj$sep$method_name::<Shopsetup>$args,
            "teamelectionstats" => $obj$sep$method_name::<Teamelectionstats>$args,
            "vault" => $obj$sep$method_name::<Vault>$args,
            "stadiumprefabs" => $obj$sep$method_name::<Stadiumprefabs>$args,
            "thebook" => $obj$sep$method_name::<Thebook>$args,
            "thebeat" => $obj$sep$method_name::<Thebeat>$args,
            "teamstatsheet" => $obj$sep$method_name::<Teamstatsheet>$args,
            "glossarywords" => $obj$sep$method_name::<Glossarywords>$args,
            "peanutpower" => $obj$sep$method_name::<Peanutpower>$args,
            "gammasim" => $obj$sep$method_name::<Gammasim>$args,
            "gammaelections" => $obj$sep$method_name::<Gammaelections>$args,
            "gammaelectionresults" => $obj$sep$method_name::<Gammaelectionresults>$args,
            "gammaelectiondetails" => $obj$sep$method_name::<Gammaelectiondetails>$args,
            "gammaelection" => $obj$sep$method_name::<Gammaelection>$args,
            "gammabracket" => $obj$sep$method_name::<Gammabracket>$args,
            "gamestatsheet" => $obj$sep$method_name::<Gamestatsheet>$args,
            "feedseasonlist" => $obj$sep$method_name::<Feedseasonlist>$args,
            "fanart" => $obj$sep$method_name::<Fanart>$args,
            "dayssincelastincineration" => $obj$sep$method_name::<Dayssincelastincineration>$args,
            "championcallout" => $obj$sep$method_name::<Championcallout>$args,
            "availablechampionbets" => $obj$sep$method_name::<Availablechampionbets>$args,
            "attributes" => $obj$sep$method_name::<Attributes>$args,
            "playerstatsheet" => $obj$sep$method_name::<Playerstatsheet>$args,
            "tributes" => $obj$sep$method_name::<Tributes>$args,
            "stream" => $obj$sep$method_name::<StreamEntityWrapper<thisidisstaticyo::StreamData>>$args,
            _ => $last_case
        }
    }
}

pub fn timestamp_from_nanos(ns: i64) -> iso8601_timestamp::Timestamp {
    iso8601_timestamp::Timestamp::UNIX_EPOCH + iso8601_timestamp::Duration::nanoseconds(ns)
}

pub fn timestamp_from_millis(ms: i64) -> iso8601_timestamp::Timestamp {
    iso8601_timestamp::Timestamp::UNIX_EPOCH + iso8601_timestamp::Duration::milliseconds(ms)
}

pub fn timestamp_to_nanos(t: iso8601_timestamp::Timestamp) -> i64 {
    let d = t.duration_since(iso8601_timestamp::Timestamp::UNIX_EPOCH);
    (d.whole_seconds() * 1_000_000_000) + (d.subsec_nanoseconds() as i64)
}

pub fn timestamp_to_millis(t: iso8601_timestamp::Timestamp) -> i64 {
    let d = t.duration_since(iso8601_timestamp::Timestamp::UNIX_EPOCH);
    (d.whole_seconds() * 1_000) + (d.subsec_nanoseconds() as i64 / 1_000_000)
}

pub fn secs_to_nanos(t: i64) -> i64 {
    t * 1_000_000_000
}

// pub fn millis_to_nanos(t: i64) -> i64 {
//     t * 1_000_000
// }

// pub fn nanos_to_millis(t: i64) -> i64 {

// }

#[cfg(test)]
mod test {
    use crate::{timestamp_from_nanos, timestamp_to_nanos};

    #[test]
    fn timestamp() {
        let ts = iso8601_timestamp::Timestamp::parse("2023-06-20T18:54:09+00:00").unwrap();
        assert_eq!(timestamp_to_nanos(ts), 1687287249000000000);
        assert_eq!(timestamp_from_nanos(1687287249000000000), ts);
    }
}
