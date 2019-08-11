use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;
use std::io;

use super::Metadata;

pub fn read<R>(reader: &mut R) -> Result<Metadata, io::Error>
where R: Read {
    Ok(Metadata {
        aggregation_type: reader.read_u32::<BigEndian>()?,
        max_retention: reader.read_u32::<BigEndian>()?,
        x_file_factor: reader.read_f32::<BigEndian>()?,
        archive_count: reader.read_u32::<BigEndian>()?
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;
    use io::BufReader;
    
    const METADATA_SIZE: usize = size_of::<Metadata>();
    
    #[test]
    fn metadata_parsing() {
        let buf: [u8; METADATA_SIZE] = [0; METADATA_SIZE];
        let mut reader = BufReader::new(&buf[..]);
        let header = read(&mut reader);
        let header = header.unwrap();

        assert_eq!(header.aggregation_type, 0);
        assert_eq!(header.max_retention, 0);
        assert_eq!(header.x_file_factor, 0.0);
        assert_eq!(header.archive_count, 0);
    }
}