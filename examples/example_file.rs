use tex_atlas;
use std::fs::File;

const EXAMPLE: &str = "assets/example.atlas";


fn main() {
    let file = File::open(EXAMPLE).unwrap();
    let atlas = tex_atlas::from_reader(file);
    assert!(atlas.is_ok());
}
