use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;
use std::io;

use super::DataPoint;

pub fn read_seq<R>(reader: &mut R) -> Result<DataPoint, io::Error>
where R: Read {
    let dp = DataPoint {
        interval: reader.read_u32::<BigEndian>()?,
        data: reader.read_f64::<BigEndian>()?
    };
    Ok(dp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;
    use io::BufReader;
    const DATAPOINT_SIZE: usize = size_of::<DataPoint>();

    #[test]
    fn datapoint_seq_parsing() {
        let buf: [u8; DATAPOINT_SIZE] = [0; DATAPOINT_SIZE];
        let mut reader = BufReader::new(&buf[..]);
        let dp = read_seq(&mut reader);
        let dp = dp.unwrap();

        assert_eq!(dp.interval, 0);
        assert_eq!(dp.data, 0.0);
    }
}