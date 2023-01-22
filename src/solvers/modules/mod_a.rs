pub fn for_around_2D<F: Fn(i32)>(f: F, v: Vec<Vec<i32>>, r: usize) {
    for i in 0..2 * r + 1 {
        for j in 0..2 * r + 1 {
            let i = i as i32 - r as i32;
            let j = j as i32 - r as i32;
            if i == 0 && j == 0 {
                continue;
            }
            match v.get((v.len() as i32 + i) as usize) {
                Some(a) => match a.get((a.len() as i32 + j) as usize) {
                    Some(t) => f(*t),
                    None => (),
                },
                None => (),
            }
        }
    }
}
