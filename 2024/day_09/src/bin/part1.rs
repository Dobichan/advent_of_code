#[derive(Clone, Debug)]
struct FileInfo {
    id: u32,
    start: usize,
    len: usize,
}

#[derive(Clone, Debug)]
struct FreeInfo {
    start: usize,
    len: usize,
}

#[derive(Clone, Debug)]
enum Block {
    Free { f: FreeInfo },
    File { f: FileInfo },
}

fn print_disk(disk: &Vec<Block>) {
    for f in disk {
        match f {
            Block::File { f } => print!("{:?}", f.id),
            Block::Free { f: _ } => print!("."),
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
                },
            };

            // println!("{index} - File: {:?}", file);
            for _ in 0..len {
                ret.push(file.clone());
            }
            file_counter += 1;
        } else {
            let free = Block::Free {
                f: FreeInfo {
                    start: block_no,
                    len,
                },
            };

            // println!("{index} - Free: {:?}", free);
            for _ in 0..len {
                ret.push(free.clone());
            }
        }
        block_no += len;
    }
    // print_disk(&ret);

    ret
}

fn defragment_disk(disk: &mut Vec<Block>) -> Vec<Block> {
    let mut ret: Vec<Block> = vec![];
    let mut start: usize = 0;
    let mut end = disk.len() - 1;

    while start <= end {
        let block: &Block = &disk[start];
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
    // print_disk(&ret);
    ret
}

fn calculate_checksum(disk: Vec<Block>) -> u64 {
    let mut ret: u64 = 0;

    for (index, b) in disk.iter().enumerate() {
        ret += index as u64
            * match b {
                Block::File { f } => f.id as u64,
                Block::Free { f: _ } => panic!("Should not have free blocks in a defrag disk"),
            }
    }
    ret
}

fn part1(input: &str) -> u64 {
    let mut disk = parse(input);

    let defrag = defragment_disk(&mut disk);

    calculate_checksum(defrag)
}

fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_small() {
        let dummy = "12345";

        assert_eq!(part1(dummy), 60);
    }

    #[test]
    fn test_part1_larger() {
        let dummy = "2333133121414131402";

        assert_eq!(part1(dummy), 1928);
    }
}
