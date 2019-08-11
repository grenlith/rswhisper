#[derive(Debug)]
pub struct Metadata {
    pub aggregation_type: u32,
    pub max_retention: u32,
    pub x_file_factor: f32,
    pub archive_count: u32
}

#[derive(Debug)]
pub struct ArchiveInfo {
    pub offset: u32,
    pub seconds_per_point: u32,
    pub points: u32
}

#[derive(Debug)]
pub struct DataPoint {
    pub interval: u32,
    pub data: f64
}


pub mod metadata;
pub mod archives;
pub mod datapoint;