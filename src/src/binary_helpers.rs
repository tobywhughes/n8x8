use std::io::{Error, ErrorKind};

pub fn u8_slice_to_u32(u8_slice: Vec<u8>) -> u32
{
    if u8_slice.len() != 4
    {
        panic!("u32 must be made from 4 u8 values");
    }
    return (u8_slice[0] as u32) << 24 | (u8_slice[1] as u32) << 16 | (u8_slice[2] as u32) << 8 | (u8_slice[3] as u32)
}

pub fn u8_slice_to_u16(u8_slice: Vec<u8>) -> u16
{
    if u8_slice.len() != 2
    {
        panic!("u16 must be made from 2 u8 values");
    }
    return (u8_slice[0] as u16) << 8 | (u8_slice[1] as u16)
}

pub fn u8_vector_to_u32_vector(u8_vec: Vec<u8>) -> Vec<u32>
{
    if (u8_vec.len() % 4 != 0) || (u8_vec.len() == 0)
    {
        panic!("u8 vector must be of nonzero length and must have a length divisible by 4");
    }
    let mut u32_vector: Vec<u32> = vec![0; u8_vec.len() / 4];
    for u32_vector_index in 0..u32_vector.len()
    {
        let u8_slice: Vec<u8> = u8_vec[(u32_vector_index * 4)..((u32_vector_index + 1) * 4)].to_vec();
        u32_vector[u32_vector_index] = u8_slice_to_u32(u8_slice);
    }
    return u32_vector;
}

pub fn u32_to_u8_vector_by_loc(value: u32, loc: usize, u8_vec: &mut Vec<u8>)
{
    if loc > u8_vec.len() -4
    {
        panic!("loc out of u8 vector scope");
    }
    for offset in 0..4
    {
        u8_vec[loc + offset] = (value >> (24 - (8 * offset)) & 0x000000FF) as u8;
    }
}

pub fn add_u16_to_u32_as_i16_overflow(u32_val: u32, u16_val: u16) -> u32
{
    let u32_val_i = u32_val as i64;
    let u16_val_i = u16_val as i16 as i64;
    let result: i64 = u32_val_i + u16_val_i;
    (result & 0x00000000FFFFFFFF) as u32
}

pub fn add_u16_to_u32_as_i16_trap(u32_val: u32, u16_val: u16) -> Result<u32, Error>
{
    let u32_val_i = u32_val as i64;
    let u16_val_i = u16_val as i16 as i64;
    let result: i64 = u32_val_i + u16_val_i;
    if (result & 0xFFFFFFFF00000000) == 0
    {
        Ok(result as u32)
    }
    else
    {
        Err(Error::new(ErrorKind::Other, "Overflow Error. Trap Handler Not Implemented."))
    }
}



#[cfg(test)]
mod binary_helpers_tests
{
    use binary_helpers::*;

    #[test]
    fn u8_slice_to_u32_pass_test() {
        let test_vec: Vec<u8> = vec![0x01, 0x23, 0x45, 0x67];
        let result: u32 = u8_slice_to_u32(test_vec);
        assert_eq!(result, 0x01234567)
    }

    #[test]
    #[should_panic]
    fn u8_slice_to_u32_fail_len_test() {
        let test_vec: Vec<u8> = vec![0x01, 0x23, 0x45];
        u8_slice_to_u32(test_vec);
    }

    #[test]
    fn u8_slice_to_u16_pass_test() {
        let test_vec: Vec<u8> = vec![0x01, 0x23];
        let result: u16 = u8_slice_to_u16(test_vec);
        assert_eq!(result, 0x0123);
    }

    #[test]
    #[should_panic]
    fn u8_slice_to_u16_fail_len_test() {
        let test_vec: Vec<u8> = vec![0x01, 0x23, 0x45];
        u8_slice_to_u16(test_vec);
    }

    #[test]
    fn u8_vector_to_u32_vector_pass_test() {
        let mut u8_test_vec: Vec<u8> = vec![0; 0x400];
        let mut u32_test_vec: Vec<u32> = vec![0; 0x100];
        for value in 0..0x100
        {
            u8_test_vec[(value * 4) + 3] = value as u8;
            u32_test_vec[value] = value as u32;
        }
        let converted_vector: Vec<u32> = u8_vector_to_u32_vector(u8_test_vec);
        assert_eq!(converted_vector, u32_test_vec);
    }

    #[test]
    fn u32_to_u8_vector_by_loc_test() {
        let u32_test_val = 0x12345678_u32;
        let mut u8_test_vec: Vec<u8> = vec![0; 0x08];
        u32_to_u8_vector_by_loc(u32_test_val, 0x04, &mut u8_test_vec);
        assert_eq!(u8_test_vec[0x04], 0x12);
        assert_eq!(u8_test_vec[0x05], 0x34);
        assert_eq!(u8_test_vec[0x06], 0x56);
        assert_eq!(u8_test_vec[0x07], 0x78);
    }

    #[test]
    fn add_add_u16_to_u32_as_i16_overflow_test() {
        //Regular
        assert_eq!(add_u16_to_u32_as_i16_overflow(0x00000001_u32, 0x0001_u16), 0x00000002_u32);
        //Negative
        assert_eq!(add_u16_to_u32_as_i16_overflow(0x00000001_u32, 0xFFFF_u16), 0x00000000_u32);
        //Overflow
        assert_eq!(add_u16_to_u32_as_i16_overflow(0xFFFFFFFF_u32, 0x0001_u16), 0x00000000_u32);
        //Negative Overflow
        assert_eq!(add_u16_to_u32_as_i16_overflow(0x00000000_u32, 0xFFFF_u16), 0xFFFFFFFF_u32);
    }

    #[test]
    fn add_add_u16_to_u32_as_i16_trap_test() {
        //Regular
        assert_eq!(add_u16_to_u32_as_i16_trap(0x00000001_u32, 0x0001_u16).unwrap(), 0x00000002_u32);
        //Negative
        assert_eq!(add_u16_to_u32_as_i16_trap(0x00000001_u32, 0xFFFF_u16).unwrap(), 0x00000000_u32);
        //Overflow
        assert!(add_u16_to_u32_as_i16_trap(0xFFFFFFFF_u32, 0x0001_u16).is_err());
        //Negative Overflow
        assert!(add_u16_to_u32_as_i16_trap(0x00000000_u32, 0xFFFF_u16).is_err());
    }
}