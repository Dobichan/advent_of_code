use crate::AoCSolution;

const YEAR: u16 = 2024;
const DAY: u8 = 9;
pub struct Solution {}

#[derive(Clone, Debug)]
struct FileInfo {
    id: u32,
    start: usize,
    length: usize,
}

#[derive(Clone, Debug)]
struct FreeInfo {
    start: usize,
    length: usize,
}

#[derive(Clone, Debug)]
enum Block {
    Free { f: FreeInfo },
    File { f: FileInfo },
}

fn _print_disk(disk: &Vec<Block>) {
    for f in disk {
        match f {
            Block::File { f } => print!(" {:?} ", f.id),
            Block::Free { f: _ } => print!(" . "),
        }
    }
    println!();
}

fn parse(input: &str, condensed: bool) -> Vec<Block> {
    let mut ret = Vec::with_capacity(1024 * 1024);
    let mut block_no = 0;
    let mut file_counter = 0;
    let mut is_file = true;

    for c in input.trim().chars() {
        let length = c.to_digit(10).unwrap() as usize;
        if is_file {
            let file = Block::File {
                f: FileInfo {
                    id: file_counter,
                    start: block_no,
                    length,
                },
            };
            if condensed {
                ret.push(file.clone());
            } else {
                for _ in 0..length {
                    ret.push(file.clone());
                }
            }

            file_counter += 1;
            is_file = false;
        } else {
            let free = Block::Free {
                f: FreeInfo {
                    start: block_no,
                    length,
                },
            };
            if condensed {
                ret.push(free.clone());
            } else {
                for _ in 0..length {
                    ret.push(free.clone());
                }
            }
            is_file = true;
        }
        block_no += length;
    }
    // _print_disk(&ret);
    ret
}

fn defragment(disk: &Vec<Block>) -> Vec<Block> {
    let mut ret = Vec::with_capacity(50 * 1024 * 1024);
    let mut start: usize = 0;
    let mut end = disk.len() - 1;

    while start <= end {
        let block = &disk[start];
        match block {
            Block::File { f: _ } => {
                ret.push(block.clone());
                start += 1;
            }
            Block::Free { f: _ } => {
                let b: &Block = &disk[end];
                if let Block::File { f: _ } = b {
                    ret.push(b.clone());
                    start += 1;
                }
                end -= 1;
            }
        }
    }
    // _print_disk(&ret);
    ret
}

fn defragment_type2(disk: &mut Vec<Block>) -> Vec<Block> {
    let mut ret = Vec::with_capacity(50 * 1024 * 1024);

    for i in 0..disk.len() {
        match &disk[i] {
            Block::File { f } => {
                for _ in 0..f.length {
                    ret.push(disk[i].clone());
                }
            }
            Block::Free { f } => {
                for _ in 0..f.length {
                    ret.push(disk[i].clone());
                }
            }
        }
    }

    // println!("Initial disk:");
    // _print_disk(disk);
    // _print_disk(&ret);

    loop {
        let mut last_file = disk.remove(disk.len() - 1);
        while let Block::Free { f: _ } = last_file {
            last_file = disk.remove(disk.len() - 1);
        }

        if let Block::File { f: mut lf } = last_file {
            // Iterate over free spaces that matches
            for i in 0..disk.len() {
                if let Block::Free { mut f } = disk[i].clone() {
                    if f.length >= lf.length {
                        for j in 0..lf.length {
                            ret[lf.start + j] = Block::Free {
                                f: FreeInfo {
                                    start: lf.start,
                                    length: lf.length,
                                },
                            };
                        }

                        lf.start = f.start;
                        for j in 0..lf.length {
                            ret[f.start + j] = Block::File { f: lf.clone() };
                        }
                        f.length = f.length - lf.length;
                        f.start += lf.length;

                        disk[i] = Block::Free { f };
                        break;
                    }
                }
            }
        }

        // _print_disk(disk);
        // _print_disk(&ret);
        if disk.is_empty() {
            break;
        }
    }
    // _print_disk(&ret);

    ret
}

fn calculate_checksum(disk: &Vec<Block>) -> u64 {
    let mut ret = 0;
    for (index, block) in disk.iter().enumerate() {
        ret += index as u64
            * match block {
                Block::File { f } => f.id as u64,
                Block::Free { f: _ } => 0,
            }
    }

    ret
}

impl AoCSolution for Solution {
    fn year(&self) -> u16 {
        YEAR
    }
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&mut self, input: &str) -> String {
        let disk = parse(input, false);
        let defrag = defragment(&disk);

        calculate_checksum(&defrag).to_string()
    }

    fn part2(&mut self, input: &str) -> String {
        let mut disk = parse(input, true);
        let defrag = defragment_type2(&mut disk);

        calculate_checksum(&defrag).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_small() {
        const EXAMPLE_INPUT: &str = "12345";

        let mut sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "60");
    }

    #[test]
    fn test_part1_larger() {
        const EXAMPLE_INPUT: &str = "2333133121414131402";

        let mut sol = Solution {};
        let answer = sol.part1(&EXAMPLE_INPUT);

        assert_eq!(answer, "1928");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = "2333133121414131402";

        let mut sol = Solution {};
        let answer = sol.part2(&EXAMPLE_INPUT);

        assert_eq!(answer, "2858");
    }
}
