use binrw::binread;
use chrono::{DateTime, Utc};

#[binread]
#[derive(Debug)]
#[br(little)]
pub struct Custom {
    #[br(temp)]
    camera_shoot: u8,
    #[br(temp)]
    video_shoot: u8,
    pub h_speed: f32,
    pub distance: f32,
    #[br(map = |x: i64| DateTime::from_timestamp(x / 1000, (x % 1000 * 1000000) as u32).unwrap())]
    pub updateTimeStamp: DateTime<Utc>,
}
