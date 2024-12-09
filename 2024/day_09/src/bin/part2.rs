#[derive(Clone, Debug)]
struct FileInfo {
    id: u32,
    start: usize,
    len: usize,
    sorted: bool,
}

#[derive(Clone, Debug)]
struct FreeInfo {
    start: usize,
    len: usize,
    // sorted: bool,
}

#[derive(Clone, Debug)]
enum Block {
    Free { f: FreeInfo },
    File { f: FileInfo },
}

fn print_disk(disk: &Vec<Block>) {
    for f in disk {
        match f {
            Block::File { f } => print!(" {:?} ", f.id),
            Block::Free { f: _ } => print!(" . "),
        }
    }
    println!();
}

fn parse(input: &str) -> Vec<Block> {
    let mut ret: Vec<Block> = vec![];
    let mut block_no = 0;
    let mut file_counter = 0;

    for (index, c) in input.chars().enumerate() {
        let len = c.to_digit(10).unwrap() as usize;
        if index % 2 == 0 {
            let file = Block::File {
                f: FileInfo {
                    id: file_counter,
                    start: block_no,
                    len,
                    sorted: false,
                },
            };

            // println!("{index} - File: {:?}", file);
            // for _ in 0..len {
            //     ret.push(file.clone());
            // }
            ret.push(file.clone());
            file_counter += 1;
        } else {
            let free = Block::Free {
                f: FreeInfo {
                    start: block_no,
                    len,
                },
            };

            // println!("{index} - Free: {:?}", free);
            // for _ in 0..len {
            //     ret.push(free.clone());
            // }
            ret.push(free.clone());
        }
        block_no += len;
    }
    // print_disk(&ret);

    ret
}

fn get_disk_blocks(disk: &Vec<Block>) -> usize {
    match disk.last().unwrap() {
        Block::File { f } => f.start + f.len,
        Block::Free { f } => f.start + f.len,
    }
}

fn defragment_disk(disk: &mut Vec<Block>) -> Vec<Block> {
    let mut ret: Vec<Block> = Vec::with_capacity(get_disk_blocks(disk));

    for i in 0..disk.len() {
        match &disk[i] {
            Block::File { f } => {
                for _ in 0..f.len {
                    ret.push(disk[i].clone());
                }
            }
            Block::Free { f } => {
                for _ in 0..f.len {
                    ret.push(disk[i].clone());
                }
            }
        }
    }

    println!("Initial disk:");
    print_disk(&ret);
    print_disk(&disk);

    loop {

        let mut last_file = disk.remove(disk.len() - 1);
        while let Block::Free { f: _ } = last_file {
            last_file = disk.remove(disk.len() - 1);
        }

        if let Block::File { f: mut lf } = last_file {
            // Iterate over free spaces that matches
            for i in 0..disk.len() {
                if let Block::Free { mut f } = disk[i].clone() {
                    if f.len >= lf.len {
                        for j in 0..lf.len {
                            ret[lf.start + j] = Block::Free {
                                f: FreeInfo {
                                    start: lf.start,
                                    len: lf.len,
                                },
                            };
                        }

                        lf.start = f.start;
                        for j in 0..lf.len {
                            ret[f.start + j] = Block::File { f: lf.clone() };
                        }
                        f.len = f.len - lf.len;
                        f.start += lf.len;

                        disk[i] = Block::Free { f };
                        break;
                    }
                }
            }

        }

        // print_disk(disk);
        // print_disk(&ret);
        if disk.is_empty() {
            break;
        }
    }

    print_disk(&ret);

    ret
}

fn calculate_checksum(disk: Vec<Block>) -> u64 {
    let mut ret: u64 = 0;

    for (index, b) in disk.iter().enumerate() {
        ret += index as u64
            * match b {
                Block::File { f } => f.id as u64,
                Block::Free { f: _ } => 0,
            }
    }
    ret
}

fn part2(input: &str) -> u64 {
    let mut disk = parse(input);

    let defrag = defragment_disk(&mut disk);

    calculate_checksum(defrag)
}

fn main() {
    println!("Part2!");

    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_larger() {
        let dummy = "2333133121414131402";

        assert_eq!(part2(dummy), 2858);
    }
}

