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
}