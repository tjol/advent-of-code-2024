use std::collections::BTreeMap;

pub fn day09part1(input: &str) -> usize {
    let mut disk = parse_disk_map(input);

    let mut defragmented = vec![];

    for i in 0..disk.len() {
        if i >= disk.len() {
            break;
        }
        let fragment = disk[i];
        match fragment.content {
            Fragment::File(_) => {
                defragmented.push(fragment);
            }
            Fragment::Empty => {
                let mut len = fragment.len;
                while disk.len() > i + 2 && len > 0 {
                    let final_fragment = *disk.last().unwrap();
                    if final_fragment.content == Fragment::Empty {
                        disk.pop();
                    } else if final_fragment.len <= len {
                        len -= final_fragment.len;
                        defragmented.push(final_fragment);
                        disk.pop();
                    } else {
                        let mut new_fragment = final_fragment;
                        new_fragment.len = len;
                        disk.last_mut().unwrap().len -= len;
                        len = 0;
                        defragmented.push(new_fragment);
                    }
                }
            }
        }
    }

    checksum(&defragmented)
}

pub fn day09part2(input: &str) -> usize {
    let mut disk = parse_disk_map(input);

    let lengths: BTreeMap<FileId, usize> = disk
        .iter()
        .filter_map(|seg| {
            if let Fragment::File(file_id) = seg.content {
                Some((file_id, seg.len))
            } else {
                None
            }
        })
        .collect();
    let max_id = *lengths.keys().max().unwrap();

    let mut file_id = max_id;

    while file_id.0 > 0 {
        let len = *lengths.get(&file_id).unwrap();

        let mut dest = None;
        for (i, seg) in disk.iter().enumerate() {
            match seg.content {
                Fragment::File(this_id) => {
                    if this_id == file_id {
                        break; // don't move right
                    }
                }
                Fragment::Empty => {
                    if seg.len >= len {
                        dest = Some(i);
                        break;
                    }
                }
            }
        }

        if let Some(i) = dest {
            let mut new_disk = vec![];
            new_disk.extend_from_slice(&disk[0..i]);
            new_disk.push(DiskSegment {
                content: Fragment::File(file_id),
                len,
            });
            if disk[i].len > len {
                new_disk.push(DiskSegment {
                    content: Fragment::Empty,
                    len: disk[i].len - len,
                });
            }
            for seg in &disk[i + 1..] {
                match seg.content {
                    Fragment::File(this_id) if this_id != file_id => {
                        new_disk.push(*seg);
                    }
                    _ => {
                        if new_disk.last().unwrap().content == Fragment::Empty {
                            new_disk.last_mut().unwrap().len += seg.len;
                        } else {
                            new_disk.push(DiskSegment {
                                content: Fragment::Empty,
                                len: seg.len,
                            });
                        }
                    }
                }
            }
            disk = new_disk;
        }

        file_id = FileId(file_id.0 - 1);
    }

    checksum(&disk)
}

#[derive(Debug, Clone, Copy)]
struct DiskSegment {
    pub content: Fragment,
    pub len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fragment {
    Empty,
    File(FileId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FileId(usize);

fn parse_disk_map(disk_map: &str) -> Vec<DiskSegment> {
    let mut disk = vec![];

    let mut is_file = true;
    let mut next_id = 0;

    for len_c in disk_map.trim().chars() {
        let len = (len_c as u32 - '0' as u32) as usize;

        if is_file {
            disk.push(DiskSegment {
                content: Fragment::File(FileId(next_id)),
                len,
            });
            is_file = false;
            next_id += 1;
        } else {
            disk.push(DiskSegment {
                content: Fragment::Empty,
                len,
            });
            is_file = true;
        }
    }

    disk
}

fn checksum(disk: &[DiskSegment]) -> usize {
    let mut pos = 0;
    let mut acc = 0;

    for fragment in disk {
        if let Fragment::File(file_id) = fragment.content {
            for _block in 0..fragment.len {
                acc += file_id.0 * pos;
                pos += 1;
            }
        } else {
            pos += fragment.len;
        }
    }

    acc
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn part1test() {
        assert_eq!(day09part1(TEST_INPUT), 1928);
    }

    #[test]
    fn part2test() {
        assert_eq!(day09part2(TEST_INPUT), 2858);
    }
}
