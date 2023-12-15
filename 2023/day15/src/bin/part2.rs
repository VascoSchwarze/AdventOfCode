#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let steps: Vec<&str> = input.lines().flat_map(|line| line.split(',')).collect();

    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    for step in steps {
        if step.contains('-') {
            let label = &step[0..step.len() - 1];
            let box_idx = hash(label);
            let lens_idx = boxes[box_idx as usize]
                .iter()
                .position(|lens| lens.label == label);
            if let Some(lens_idx) = lens_idx {
                boxes[box_idx as usize].remove(lens_idx);
            }
        } else {
            let (label, focal_length) = step.split_once('=').expect("Should contain an =.");
            let focal_length = focal_length.parse::<u32>().expect("Should be a number.");

            let box_idx = hash(label);
            let lens_idx = boxes[box_idx as usize]
                .iter()
                .position(|lens| lens.label == label);
            if let Some(lens_idx) = lens_idx {
                boxes[box_idx as usize][lens_idx].focal_length = focal_length;
            } else {
                boxes[box_idx as usize].push(Lens {
                    label: label.to_string(),
                    focal_length,
                })
            }
        }
    }

    let mut sum: u32 = 0;
    for (box_idx, lens_box) in boxes.iter().enumerate() {
        for (lens_idx, lens) in lens_box.iter().enumerate() {
            sum += (1 + box_idx as u32) * (1 + lens_idx as u32) * lens.focal_length;
        }
    }
    println!("{sum}");
}

fn hash(string: &str) -> u32 {
    string.chars().fold(0, |acc, c| {
        let val = c as u32;
        ((acc + val) * 17) % 256
    })
}
