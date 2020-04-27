use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

use tex_atlas;
use tex_atlas::{ColorType, Origin, BoundingBoxPixelCoords, OffsetPixelCoords, TextureAtlas2D};


const SAMPLE_DATA: &str = "assets/sample.atlas";
const EXAMPLE_DATA: &str = "assets/example.atlas";


/// Loading an atlas file that does not exist should fail.
#[test]
fn loading_a_nonexistent_atlas_file_should_fail() {
    let path = Path::new("DoesNotExist.atlas");
    assert!(!path.exists());

    let maybe_atlas = tex_atlas::load_file(path);
    assert!(maybe_atlas.is_err());
}

/// Given a valid atlas file, we should be able to write it to storage.
#[test]
fn atlas_file_should_write_to_storage_successfully() {
    let atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let path = Path::new("test.atlas");
    let result = tex_atlas::write_to_file(path, &atlas);
    fs::remove_file(path).unwrap();

    assert!(result.is_ok());
}


struct ReadWriteTest {
    expected_atlas: tex_atlas::TextureAtlas2D,
    result_atlas: tex_atlas::TextureAtlas2D,
}

impl ReadWriteTest {
    fn new(
        expected_atlas: tex_atlas::TextureAtlas2D,
        result_atlas: tex_atlas::TextureAtlas2D) -> ReadWriteTest {

        ReadWriteTest {
            expected_atlas: expected_atlas,
            result_atlas: result_atlas,
        }
    }
}

fn read_write_test<P: AsRef<Path>>(expected_path: P) -> ReadWriteTest {
    let expected_atlas = tex_atlas::load_file(&expected_path).unwrap().atlas;
    let buffer = vec![];
    let mut cursor = io::Cursor::new(buffer);
    tex_atlas::to_writer(&mut cursor, &expected_atlas).unwrap();
    let result_atlas = tex_atlas::from_reader(&mut cursor).unwrap().atlas;

    ReadWriteTest::new(expected_atlas, result_atlas)
}

/// Given a valid texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The height in pixels of the atlas images should match.
#[test]
fn atlas_file_written_and_then_read_should_match_heights() {
    let test = read_write_test(SAMPLE_DATA);

    assert_eq!(test.result_atlas.height, test.expected_atlas.height);
}
