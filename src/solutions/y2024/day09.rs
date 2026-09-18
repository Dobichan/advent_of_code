use std::{fmt::Display, iter::repeat_n};

use crate::AoCSolution;

#[derive(Debug, Default)]
pub struct Solution {}

#[derive(Clone, Debug)]
pub struct FileInfo {
    id: usize,
    start: usize,
    length: usize,
}

#[derive(Clone, Debug)]
pub struct FreeInfo {
    start: usize,
    length: usize,
}

#[derive(Clone, Debug)]
pub enum Block {
    File { f: FileInfo },
    Free { f: FreeInfo },
}

fn _print_disk(disk: &Vec<Block>) {
    for f in disk {
        match f {
            Block::File { f } => {
                for _i in 0..f.length {
                    print!("{}", f.id);
                }
            }

            Block::Free { f } => {
                for _i in 0..f.length {
                    print!(".");
                }
            }
        }
    }
    println!();
}

fn defragment(disk: &[Block]) -> Vec<usize> {
    let total = disk
        .iter()
        .map(|b| match b {
            Block::File { f } => f.length,
            Block::Free { f } => f.length,
        })
        .sum();
    let mut ret = Vec::with_capacity(total);

    let mut start: usize = 0;
    let mut end = disk.len();
    let mut tail_id = 0;
    let mut tail_left = 0;

    while start < end {
        match &disk[start] {
            Block::File { f } => {
                ret.extend(repeat_n(f.id, f.length));
            }
            Block::Free { f } => {
                let mut gap = f.length;
                while gap > 0 {
                    if tail_left == 0 {
                        end -= 1;
                        if end <= start {
                            break;
                        }
                        if let Block::File { f: tail } = &disk[end] {
                            tail_id = tail.id;
                            tail_left = tail.length;
                        }
                        continue;
                    }
                    let n = gap.min(tail_left);
                    ret.extend(repeat_n(tail_id, n));
                    gap -= n;
                    tail_left -= n;
                }
            }
        }
        start += 1;
    }
    ret.extend(repeat_n(tail_id, tail_left));
    ret
}

fn defragment_type2(disk: &[Block]) -> Vec<FileInfo> {
    let mut files = Vec::new();
    let mut gaps = Vec::new();

    for block in disk {
        match block {
            Block::File { f } => files.push(f.clone()),
            Block::Free { f } if f.length > 0 => gaps.push(f.clone()),
            Block::Free { .. } => {}
        }
    }

    for file in files.iter_mut().rev() {
        let Some(gap) = gaps
            .iter_mut()
            .take_while(|g| g.start < file.start)
            .find(|g| g.length >= file.length)
        else {
            continue;
        };
        file.start = gap.start;
        gap.start += file.length;
        gap.length -= file.length
    }
    files
}

fn calculate_checksum(disk: &[usize]) -> u64 {
    let mut ret = 0;
    for (index, &block) in disk.iter().enumerate() {
        ret += index as u64 * block as u64;
    }

    ret
}

fn checksum_files(files: &[FileInfo]) -> usize {
    files
        .iter()
        .map(|f| f.id * (f.start..f.start + f.length).sum::<usize>())
        .sum()
}

impl AoCSolution for Solution {
    type Parsed = Vec<Block>;

    fn parse(&self, input: &str) -> Self::Parsed {
        let mut start = 0;
        let mut disk = Vec::new();

        for (id, file_and_space) in input.trim().as_bytes().chunks(2).enumerate() {
            let file_len = (file_and_space[0] - b'0') as usize;
            disk.push(Block::File {
                f: FileInfo {
                    id,
                    start,
                    length: file_len,
                },
            });
            start += file_len;

            let free_len = file_and_space.get(1).map_or(0, |d| (d - b'0') as usize);
            disk.push(Block::Free {
                f: FreeInfo {
                    start,
                    length: free_len,
                },
            });
            start += free_len;
        }

        // _print_disk(&disk);
        disk
    }

    fn part1(&self, data: &Self::Parsed) -> impl Display {
        calculate_checksum(&defragment(data))
    }

    fn part2(&self, data: &Self::Parsed) -> impl Display {
        checksum_files(&defragment_type2(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_small() {
        const EXAMPLE_INPUT: &str = "12345";

        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "60");
    }

    #[test]
    fn test_part1_larger() {
        const EXAMPLE_INPUT: &str = "2333133121414131402";

        assert_eq!(Solution::default().solve1(EXAMPLE_INPUT), "1928");
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT: &str = "2333133121414131402";

        assert_eq!(Solution::default().solve2(EXAMPLE_INPUT), "2858");
    }
}
