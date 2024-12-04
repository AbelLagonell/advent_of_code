use crate::helper::read_integer_file;

pub fn record_safety(filepath: &str) -> u32 {
    let lists = read_integer_file::read_file_rows(filepath);
    let mut safety = Vec::<bool>::new();

    for vec in &lists {
        let mut local_safe = true;
        let rot: bool = vec[1] - vec[0] > 0; // false is decreasing
        for index in 1..vec.len() {
            let local_rot = vec[index] - vec[index - 1];
            print!("{}, ", local_rot);
            if index == 1 {
                continue;
            } else if (local_rot > 0) != rot {
                local_safe = false;
                break;
            } else if local_rot.abs() != 3 && local_rot.abs() != 2 && local_rot.abs() != 1 {
                local_safe = false;
                break;
            }
        }
        safety.push(local_safe);
        println!("");
    }

    let mut count = 0;
    for bl in &safety {
        if *bl {
            count += 1;
        }
    }

    println!("{}", safety.len());

    count
}
