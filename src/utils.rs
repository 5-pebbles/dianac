use arbitrary_int::{u12, u6};

pub fn tuple_as_usize(tuple: (u6, u6)) -> usize {
    ((u16::from(tuple.0) << 6) | u16::from(tuple.1)) as usize
}

pub fn tuple_as_u12(tuple: (u6, u6)) -> u12 {
    u12::new((u16::from(tuple.0) << 6) | u16::from(tuple.1))
}
