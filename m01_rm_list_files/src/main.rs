use m01_rm_list_files::frame_set;

fn main() {
    let seq = frame_set("1-10");
    println!("{:?}", seq);

    let seq2 = frame_set("1-100:8");
    println!("{:?}", seq2);
}
