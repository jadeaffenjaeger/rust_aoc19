use std::env;
use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const PIC_SIZE: usize = WIDTH * HEIGHT;

fn count_digits(layer: &[u32], digit: u32) -> u32 {
    layer.iter().filter(|&&x| x == digit).count() as u32
}

fn combine_layers(layer: &[u32], image: &mut [u32]) {
    let combine_pixels = |top, bottom| match top {
        2 => bottom,
        _ => top,
    };

    for (p1, p2) in image.iter_mut().zip(layer.iter()) {
        *p1 = combine_pixels(*p1, *p2);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();

    let image: Vec<u32> = contents
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    if let Some(max_layer) = image.chunks(PIC_SIZE).min_by_key(|x| count_digits(x, 0)) {
        let prod = count_digits(max_layer, 2) * count_digits(max_layer, 1);
        println!("Solution Part 1: {:?}", prod);
    }

    let mut output: [u32; PIC_SIZE] = [2; PIC_SIZE];
    image
        .chunks(PIC_SIZE)
        .map(|x| combine_layers(x, &mut output))
        .count();

    println!("Solution Part 2:");
    let num_to_display = |num| match num {
        1 => "█",
        _ => " ",
    };
    for line in output.chunks(WIDTH) {
        let line: String = line.iter().map(|x| num_to_display(*x)).collect();
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let vec = vec![0, 1, 1, 2, 1, 0];
        assert_eq!(count_digits(&vec, 0), 2);
        assert_eq!(count_digits(&vec, 1), 3);
        assert_eq!(count_digits(&vec, 2), 1);
        assert_eq!(count_digits(&vec[0..2], 1), 1);
    }

    #[test]
    fn test_combine() {
        let mut out = [0, 2, 2, 2];
        let l2 = [1, 1, 2, 2];
        let l3 = [2, 2, 1, 2];
        let l4 = [0, 0, 0, 0];

        combine_layers(&l2, &mut out);
        combine_layers(&l3, &mut out);
        combine_layers(&l4, &mut out);

        assert_eq!(out, [0, 1, 1, 0]);
    }
}
