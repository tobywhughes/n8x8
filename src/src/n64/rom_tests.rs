#[cfg(test)]
mod rom_tests
{
    use RomHeader;

    #[test]
    fn rom_header_parsed_values() 
    {
        let mut test_vec: Vec<u8> = vec![0; 0x1000];
        for i in 0..0x40
        {
            test_vec[i] = i as u8;
        }

        for i in 0x40..0x1000
        {
            test_vec[i] = 0xFF;
        }

        let boot_code_compare = vec![0xFFFFFFFF; 0x3F0];

        let rom_header = RomHeader::new(test_vec);

        assert_eq!(rom_header.pi_reg_initializers, vec![0x00, 0x01, 0x02, 0x03]);
        assert_eq!(rom_header.clock_rate, 0x04050607);
        assert_eq!(rom_header.program_counter, 0x08090A0B);
        assert_eq!(rom_header.release, 0x0C0D0E0F);
        assert_eq!(rom_header.crc1, 0x10111213);
        assert_eq!(rom_header.crc2, 0x14151617);
        assert_eq!(rom_header.image_name, vec![0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33]);
        assert_eq!(rom_header.manufacturer_id, 0x38393A3B);
        assert_eq!(rom_header.cartridge_id, 0x3C3D);
        assert_eq!(rom_header.country_code, 0x3E3F);
        assert_eq!(rom_header.boot_code, boot_code_compare);
    }
}