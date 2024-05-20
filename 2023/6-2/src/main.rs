fn main() {
    let time: u128 = 55999793;
    let record: u128 = 401148522741405;
    let mut case_count: u128 = 0;

    for i in 0..=time {
        if i * (time - i) > record {
            case_count += 1;
        }
    }

    dbg!(case_count);
}
