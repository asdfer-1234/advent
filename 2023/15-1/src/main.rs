fn main() {
    dbg!(include_str!("../input")
        .split(',')
        .map(|x| {
            x.as_bytes()
                .iter()
                .fold(0u8, |init, x| init.wrapping_add(*x).wrapping_mul(17))
        })
        .fold(0u32, |init, x| init + x as u32));
}
