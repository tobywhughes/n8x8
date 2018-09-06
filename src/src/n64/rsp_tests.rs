#[cfg(test)]
mod rsp_tests
{
    use n64::rsp::RealitySignalProcessor;

    #[test]
    fn load_all_dmem_and_imem_values_by_u8_vector()
    {
        let test_vector: Vec<u8> = vec![0xFF; 0x1000];
        let mut rsp: RealitySignalProcessor = RealitySignalProcessor::new();
        rsp.copy_bytes_from_u8_vector(0, test_vector.to_vec(), test_vector.len());
        rsp.copy_bytes_from_u8_vector(0x1000, test_vector.to_vec(), test_vector.len());
        for address in 0..0x1000
        {
            assert_eq!(0xFF, rsp.read_u8_from_address(address).unwrap());
            assert_eq!(0xFF, rsp.read_u8_from_address(0x1000 + address).unwrap());
        }
    }
}