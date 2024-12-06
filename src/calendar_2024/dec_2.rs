use crate::helper::read_integer_file;

pub fn record_safety(filepath: &str) -> u32 {
    let lists = read_integer_file::read_file_rows(filepath);
    let mut safety = Vec::<bool>::new();

    for vec in &lists {
        let mut local_safe = true;
        let rot: bool = vec[1] - vec[0] < 0;
        for index in 1..vec.len() {
            let local_rot = vec[index] - vec[index - 1];
            if (local_rot < 0) != rot {
                local_safe = false;
                break;
            } else if local_rot.abs() > 3 || local_rot.abs() < 1 {
                local_safe = false;
                break;
            }
        }
        safety.push(local_safe);
    }

    let mut count = 0;
    for bl in &safety {
        if *bl {
            count += 1;
        }
    }

    count
}

pub fn record_safety_with_dampening(filepath: &str) -> u32 {
    let lists = read_integer_file::read_file_rows(filepath);
    let mut safety = Vec::<i8>::new();

    for vec in &lists {
        let mut local_safe = 0;
        let rot: bool = vec[1] - vec[0] < 0;
        for index in 1..vec.len() {
            let local_rot = vec[index] - vec[index - 1];
            if (local_rot < 0) != rot {
                local_safe += 1;
            } else if local_rot.abs() > 3 || local_rot.abs() < 1 {
                local_safe += 1;
            }
        }
        safety.push(local_safe);
    }

    let mut count = 0;
    for index in 0..safety.len() {
        if safety[index] < 2 {
            count += 1;
        }
    }

    count
}
