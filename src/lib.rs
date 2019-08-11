pub mod metadata;
pub mod archives;
pub mod datapoint;

pub struct Metadata {
    pub aggregation_type: u32,
    pub max_retention: u32,
    pub x_file_factor: f32,
    pub archive_count: u32
}
impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "t: {}, mr: {}, xff: {}, ac: {}",
            self.aggregation_type,
            self.max_retention,
            self.x_file_factor,
            self.archive_count)
    }
}

pub struct ArchiveInfo {
    pub offset: u32,
    pub seconds_per_point: u32,
    pub points: u32
}
impl std::fmt::Display for ArchiveInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "o: {}, spp: {}, p: {}",
            self.offset,
            self.seconds_per_point,
            self.points)
    }
}


pub struct DataPoint {
    pub interval: u32,
    pub data: f64
}
impl std::fmt::Display for DataPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "i: {}, d: {}", self.interval, self.data)
    }
}