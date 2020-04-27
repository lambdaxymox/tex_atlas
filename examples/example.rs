use tex_atlas;

const EXAMPLE: &str = "assets/example.atlas";


fn main() {
    let atlas = tex_atlas::load_file(EXAMPLE);
    assert!(atlas.is_ok());
}
