use m01_rm_list_files::{file_sequence, frame_set};
use std::fmt::format;

fn main() {
    let seq = frame_set("1-10");
    println!("Frame Set \"1-10\"    : {:?}", seq.unwrap());

    let seq2 = frame_set("1-100:8");
    println!("Frame Set \"1-100:8\" : {:?}", seq2.as_deref().unwrap());

    println!("{}", seq2.as_ref().unwrap()[0]);
    println!(
        "{}",
        seq2.as_ref().unwrap()[seq2.as_ref().unwrap().len() - 2]
    );

    let file_seq = file_sequence("/foo/bar.1-20#.exr");
    println!("File Sequence: {:?}", file_seq.as_ref().unwrap());
}
