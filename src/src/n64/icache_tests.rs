#[cfg(test)]
mod icache_tests
{
    use n64::icache::*;

    #[test]
    fn can_translate_virtual_address_to_cache_line() {
        let mut icache: ICache = ICache::new();

        let line_index_high: u32 = icache.parse_line_index_from_virtual_index(0x00003FE0);
        let line_index_low: u32 = icache.parse_line_index_from_virtual_index(0xFFFFC01F);


        assert_eq!(511, line_index_high);
        assert_eq!(0, line_index_low);
    }
}