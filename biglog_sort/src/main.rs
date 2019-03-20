use biglog_sort::*;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use extsort::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Line(u32, String);

impl Sortable<Line> for Line {
    #[inline]
    fn encode(item: Line, write: &mut Write) {
        write.write_u32::<LittleEndian>(item.0).unwrap();
        write.write(item.1.as_bytes()).unwrap();
        write.write(&[b'\n']).unwrap();
    }

    #[inline]
    fn decode(read: &mut Read) -> Option<Line> {
        let idx = read.read_u32::<LittleEndian>().ok()?;
        let mut bytes = read.bytes();
        let s = String::from_utf8(
            bytes
                .by_ref()
                .map(Result::unwrap)
                .take_while(|b| *b != b'\n')
                .collect(),
        )
        .unwrap();
        Some(Line(idx, s))
    }
}

fn main() {
    let path = "sample_100M.txt";

    let keyword_map = make_keyword_map(path);
    let (line_cnt, line_map) = make_line_map(path, &keyword_map);
    let line_order = make_line_order(line_cnt, &line_map);

    std::mem::drop(keyword_map);
    std::mem::drop(line_map);

    // 进行外排序
    let file = BufReader::new(File::open(path).expect("无法打开文件"));
    let mut sorter = ExternalSorter::new();
    sorter.set_max_size(1 * 1024 * 1024 * 1024 / 128); // 按每行 128 字节算, 1G 内存每次最多载入多少行
    let sorted_iter = sorter
        .sort_by(
            file.lines()
                .enumerate()
                .map(|(idx, s)| Line(idx as u32, s.unwrap())),
            |a, b| line_order[a.0 as usize].cmp(&line_order[b.0 as usize]),
        )
        .unwrap();

    // 输出
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for line in sorted_iter {
        handle.write(line.1.as_bytes()).unwrap();
        handle.write(&[b'\n']).unwrap();
    }
}
