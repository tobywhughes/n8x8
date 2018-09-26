#[cfg(test)]
mod rdram_iface_tests
{
    use n64::rdram_iface::RDRAMInterface;

    #[test]
    fn access_read_and_write_by_address()
    {
        let mut rdram_iface = RDRAMInterface::new();
        for address in (0x00000000..0x00000020).step_by(0x04)
        {
            assert_eq!(rdram_iface.read_u32_from_address(address).unwrap(), 0x00000000_u32);
            rdram_iface.load_u32_to_address(address, 0xFFFFFFFF_u32);
            assert_eq!(rdram_iface.read_u32_from_address(address).unwrap(), 0xFFFFFFFF_u32);
        }
    }

    #[test]
    #[should_panic]
    fn non_aligned_address_fails()
    {
        let mut rdram_iface = RDRAMInterface::new();
        rdram_iface.read_u32_from_address(0x00000001).unwrap();
    }
}