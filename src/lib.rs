
#[repr(C, align(4))]
pub struct Header {
    pub sync: u16,
    pub channel_id: u16,
    pub packet_length: u32,
    pub data_length: u32,
    pub data_version: u8,
    pub sequence_number: u8,
    pub packet_flags: u8,
    pub data_type: u8,
    pub reference_time: [u8;6],
    pub checksum: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse() {
    //     let bytes = [
    //         1, 0, 
    //         0, 0,
    //         0, 0, 0, 0,
    //         0, 0, 0, 0,
    //         0, 0, 0, 0,
    //         0, 0, 0, 0,
    //         0, 0, 0, 0,
    //         0, 0, 0, 0,
    //     ];

    //     let _ = Header::parse(&bytes);
    // }
}