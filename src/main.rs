use std::io;
use std::io::{Read, Seek, SeekFrom, BufReader};
use std::env;
use std::fs::File;

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
struct Metadata {
    aggregation_type: u32,
    max_retention: u32,
    x_file_factor: f32,
    archive_count: u32
}

#[derive(Debug)]
struct ArchiveInfo {
    offset: u32,
    seconds_per_point: u32,
    points: u32
}

#[derive(Debug)]
struct DataPoint {
    interval: u32,
    data: f64
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} source", args[0]);
        return Ok(())
    }
    let filename = &args[1];
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let meta = metadata_read(&mut reader);
    let archives = archiveinfo_read(&mut reader, meta.archive_count);
    let datapoint = datapoint_read(&mut reader, archives[0].offset);

    println!("{:?}", meta);
    println!("{:?}", archives);
    println!("{:?}", datapoint);
    Ok(())
}

fn metadata_read<R>(reader: &mut R) -> Metadata 
where R: Read {
    let header = Metadata {
        aggregation_type: reader.read_u32::<BigEndian>().unwrap(),
        max_retention: reader.read_u32::<BigEndian>().unwrap(),
        x_file_factor: reader.read_f32::<BigEndian>().unwrap(),
        archive_count: reader.read_u32::<BigEndian>().unwrap()
    };
    header
}

fn archiveinfo_read<R>(reader: &mut R, num_archives: u32) -> Vec<ArchiveInfo>
where R: Read {
    let mut archives: Vec<ArchiveInfo> = Vec::new();
    for _x in 0..num_archives {
        let info = ArchiveInfo {
        offset: reader.read_u32::<BigEndian>().unwrap(),
        seconds_per_point: reader.read_u32::<BigEndian>().unwrap(),
        points: reader.read_u32::<BigEndian>().unwrap()
        };
        archives.push(info);
    }
    archives
}

fn datapoint_read<R>(reader: &mut R, datapoint_offset: u32) -> DataPoint
where R: Read + Seek {
    reader.seek(SeekFrom::Start(datapoint_offset.into())).unwrap();
    let dp = DataPoint {
        interval: reader.read_u32::<BigEndian>().unwrap(),
        data: reader.read_f64::<BigEndian>().unwrap()
    };
    dp
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;
    
    const METADATA_SIZE: usize = size_of::<Metadata>();
    const ARCHIVEINFO_SIZE: usize = size_of::<ArchiveInfo>();
    const DATAPOINT_SIZE: usize = size_of::<DataPoint>();
    
    #[test]
    fn metadata_parsing() {
        let buf: [u8; METADATA_SIZE] = [0; METADATA_SIZE];
        let mut reader = BufReader::new(&buf[..]);
        let header = metadata_read(&mut reader);

        assert_eq!(header.aggregation_type, 0);
        assert_eq!(header.max_retention, 0);
        assert_eq!(header.x_file_factor, 0.0);
        assert_eq!(header.archive_count, 0);
    }

    #[test]
    fn archiveinfo_parsing() {
        let buf: [u8; METADATA_SIZE + ARCHIVEINFO_SIZE] = [0; METADATA_SIZE + ARCHIVEINFO_SIZE];
        let mut reader = BufReader::new(&buf[..]);
        let archive = archiveinfo_read(&mut reader, 1);

        assert_eq!(archive[0].offset, 0);
        assert_eq!(archive[0].seconds_per_point, 0);
        assert_eq!(archive[0].points, 0);
    }

    #[test]
    fn datapoint_parsing() {
        let buf: [u8; DATAPOINT_SIZE] = [0; DATAPOINT_SIZE];
        let curs = io::Cursor::new(buf);
        let mut reader = BufReader::new(curs);
        let dp = datapoint_read(&mut reader, 0);

        assert_eq!(dp.interval, 0);
        assert_eq!(dp.data, 0.0);
    }
}