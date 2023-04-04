pub fn convert(num: f32, from: char) -> Option<f32> {
    const NUM_5: f32 = 5.0;
    const NUM_9: f32 = 9.0;
    const NUM_32: f32 = 32.0;
    const C: char = 'c';
    const F: char = 'f';

    match from {
        C => Some(((num * NUM_9) / NUM_5) + NUM_32),
        F => Some((num - NUM_32) * (NUM_5 / NUM_9)),
        _ => None,
    }
}
