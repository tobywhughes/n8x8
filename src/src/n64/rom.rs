use binary_helpers::*;

pub struct RomHeader
{
    pub pi_reg_initializers: Vec<u8>,
    pub clock_rate: u32,
    pub program_counter: u32,
    pub release: u32,
    pub crc1: u32,
    pub crc2: u32,
    pub image_name: Vec<u8>,
    pub manufacturer_id: u32,
    pub cartridge_id: u16,
    pub country_code: u16,
    pub boot_code: Vec<u32>,
}

impl RomHeader
{
    pub fn new(header_data: Vec<u8>) -> RomHeader
    {
        return RomHeader
        {
            pi_reg_initializers: header_data[0x0000..0x0004].to_vec(),
            clock_rate: u8_slice_to_u32(header_data[0x0004..0x0008].to_vec()),
            program_counter: u8_slice_to_u32(header_data[0x0008..0x000C].to_vec()),
            release: u8_slice_to_u32(header_data[0x000C..0x0010].to_vec()),
            crc1: u8_slice_to_u32(header_data[0x0010..0x0014].to_vec()),
            crc2: u8_slice_to_u32(header_data[0x0014..0x0018].to_vec()),
            image_name:  header_data[0x0020..0x0034].to_vec(),
            manufacturer_id: u8_slice_to_u32(header_data[0x0038..0x003C].to_vec()),
            cartridge_id: u8_slice_to_u16(header_data[0x003C..0x003E].to_vec()),
            country_code: u8_slice_to_u16(header_data[0x003E..0x0040].to_vec()),
            boot_code: u8_vector_to_u32_vector(header_data[0x0040..0x1000].to_vec()),
        }
    }
}