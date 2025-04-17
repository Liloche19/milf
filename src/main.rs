use png::Decoder;
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input.png>", args[0]);
        std::process::exit(1);
    }
    let image = File::open(&args[1]).unwrap();
    let decoder = Decoder::new(image);
    let mut res = decoder.read_info().unwrap();
    let mut buf = vec![0; res.output_buffer_size()];
    let info = res.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    dbg!(info);
}
