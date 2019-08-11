use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;
use std::io;

use super::{Metadata, ArchiveInfo};

pub fn read_all<R>(reader: &mut R, meta: &Metadata) -> Result<Vec<ArchiveInfo>, io::Error>
where R: Read {
    let mut archives: Vec<ArchiveInfo> = Vec::new();
    let num_archives = meta.archive_count;
    for _x in 0..num_archives {
        let info = ArchiveInfo {
        offset: reader.read_u32::<BigEndian>()?,
        seconds_per_point: reader.read_u32::<BigEndian>()?,
        points: reader.read_u32::<BigEndian>()?
        };
        archives.push(info);
    }
    Ok(archives)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata;
    use std::mem::size_of;
    use io::BufReader;
    
    const METADATA_SIZE: usize = size_of::<Metadata>();
    const ARCHIVEINFO_SIZE: usize = size_of::<ArchiveInfo>();

    #[test]
    fn archiveinfo_parsing() {
        let buf: [u8; METADATA_SIZE + ARCHIVEINFO_SIZE] = [0; METADATA_SIZE + ARCHIVEINFO_SIZE];
        let mut reader = BufReader::new(&buf[..]);
        let mut meta = metadata::read(&mut reader).unwrap();
        meta.archive_count = 1;
        let archive = read_all(&mut reader, &meta);
        let archive = archive.unwrap();

        assert_eq!(archive[0].offset, 0);
        assert_eq!(archive[0].seconds_per_point, 0);
        assert_eq!(archive[0].points, 0);
    }
}