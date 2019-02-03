use rand::Rng;

#[inline]
fn fill_random_buf(len: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let numbers: Vec<u8> = (0..len)
        .map(|_| {
            // 1 (inclusive) to 21 (exclusive)
            rng.gen_range(1, 21)
        })
        .collect();
    numbers
}
