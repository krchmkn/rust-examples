fn convert_temperature(num: f32, from: char) -> f32 {
    if from == 'c' {
        ((num * 9) / 5) + 32
    } else if from == 'f' {
        (num - 32) * (5 / 9)
    } else {
        t
    }
}
