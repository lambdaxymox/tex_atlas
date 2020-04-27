use std::fs;
use std::io;
use std::path::Path;

use tex_atlas;
use tex_atlas::{BoundingBoxPixelCoords, OffsetPixelCoords, TextureAtlas2D};


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
    let result_height = test.result_atlas.height;
    let expected_height = test.expected_atlas.height;

    assert_eq!(result_height, expected_height);
}

/// Given a valid texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The width in pixels of the atlas images should match.
#[test]
fn atlas_file_written_and_then_read_should_match_widths() {
    let test = read_write_test(SAMPLE_DATA);
    let result_width = test.result_atlas.width;
    let expected_width = test.expected_atlas.width;

    assert_eq!(result_width, expected_width);
}

/// Given a valid texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The origins of the atlases should match.
#[test]
fn atlas_file_written_and_then_read_should_match_origins() {
    let test = read_write_test(SAMPLE_DATA);
    let result_origin = test.result_atlas.origin;
    let expected_origin = test.expected_atlas.origin;

    assert_eq!(result_origin, expected_origin);
}

/// Given a valid texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The color space types of the atlases should match.
#[test]
fn atlas_file_written_and_then_read_should_match_color_types() {
    let test = read_write_test(SAMPLE_DATA);
    let result_color_type = test.result_atlas.color_type;
    let expected_color_type = test.expected_atlas.color_type;

    assert_eq!(result_color_type, expected_color_type);
}

/// Given a valid texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The color channel counts of the atlases should match.
#[test]
fn atlas_file_written_and_then_read_should_match_channel_counts() {
    let test = read_write_test(SAMPLE_DATA);
    let result_channel_count = test.result_atlas.channel_count;
    let expected_channel_count = test.expected_atlas.channel_count;

    assert_eq!(result_channel_count, expected_channel_count);
}

/// Given a valid texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The number of bytes per pixel of the atlases should match.
#[test]
fn atlas_file_written_and_then_read_should_match_bytes_per_channel() {
    let test = read_write_test(SAMPLE_DATA);
    let result_bytes_per_pixel = test.result_atlas.bytes_per_pixel;
    let expected_bytes_per_pixel = test.expected_atlas.bytes_per_pixel;

    assert_eq!(result_bytes_per_pixel, expected_bytes_per_pixel);
}

/// Given a valid texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The number of textures in the atlases should match.
#[test]
fn atlas_file_written_and_then_read_should_match_texture_counts() {
    let test = read_write_test(SAMPLE_DATA);
    let result_count = test.result_atlas.texture_count();
    let expected_count = test.expected_atlas.texture_count();

    assert_eq!(result_count, expected_count);
}

/// Given a valid texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The texture names should act as a primary key for the atlas: i.e., the 
/// texture names and the associated textures should be preserved.
#[test]
fn atlas_file_written_and_then_read_should_preserve_texture_names() {
    let test = read_write_test(SAMPLE_DATA);
    let result_atlas = test.result_atlas;
    let expected_atlas = test.expected_atlas;
    for name in result_atlas.names().iter() {
        let result = result_atlas.get_name(name);
        let expected = expected_atlas.get_name(name);
        assert_eq!(result, expected);
    }
}

#[test]
fn atlas_file_written_and_then_read_should_preserve_texture_names2() {
    let test = read_write_test(SAMPLE_DATA);
    let result_atlas = test.result_atlas;
    let expected_atlas = test.expected_atlas;
    for name in expected_atlas.names().iter() {
        let result = result_atlas.get_name(name);
        let expected = expected_atlas.get_name(name);
        assert_eq!(result, expected);
    }
}

/// Given a valid texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The texture indices should act as a primary key for the atlas: i.e., the 
/// texture indices and the associated textures should be preserved.
#[test]
fn atlas_file_written_and_then_read_should_preserve_texture_indices() {
    let test = read_write_test(SAMPLE_DATA);
    let result_atlas = test.result_atlas;
    let expected_atlas = test.expected_atlas;
    for index in result_atlas.indices().iter() {
        let result = result_atlas.get_index(*index);
        let expected = expected_atlas.get_index(*index);
        assert_eq!(result, expected);
    }
}

#[test]
fn atlas_file_written_and_then_read_should_preserve_texture_indices2() {
    let test = read_write_test(SAMPLE_DATA);
    let result_atlas = test.result_atlas;
    let expected_atlas = test.expected_atlas;
    for index in expected_atlas.indices().iter() {
        let result = result_atlas.get_index(*index);
        let expected = expected_atlas.get_index(*index);
        assert_eq!(result, expected);
    }
}

/// Given a valid texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// Every texture index and texture name that match the same texture in the atlas should
/// still do so when the data is written out and read back.
#[test]
fn atlas_file_written_and_then_read_should_preserve_textures() {
    let test = read_write_test(SAMPLE_DATA);
    let result_atlas = test.result_atlas;
    let expected_atlas = test.expected_atlas;
    let indices = result_atlas.indices();
    let names = result_atlas.names();
    let result_zip = indices.iter().zip(names.iter());
    for (index, name) in result_zip.filter(|(index, name)| { result_atlas.get_index(**index) == result_atlas.get_name(name) }) {
        let expected_index = expected_atlas.get_index(*index); 
        let expected_name = expected_atlas.get_name(name);
        assert_eq!(expected_index, expected_name);
    }
}

/// Given a valid texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// Reading back the texture atlas that was written to disk should preserve the 
/// underlying image data.
#[test]
fn atlas_file_written_and_then_read_should_preserve_underlying_image_data() {
    let test = read_write_test(SAMPLE_DATA);
    let result_atlas = test.result_atlas;
    let expected_atlas = test.expected_atlas;
    
    assert_eq!(result_atlas.as_bytes(), expected_atlas.as_bytes());
}

/// Given a valid texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// Reading back the texture atlas that was written to disk should match in the 
/// length underlying image data.
#[test]
fn atlas_file_written_and_then_read_should_preserve_underlying_image_data_length() {
    let test = read_write_test(SAMPLE_DATA);
    let result_atlas = test.result_atlas;
    let expected_atlas = test.expected_atlas;
    
    assert_eq!(result_atlas.len_bytes(), expected_atlas.len_bytes());
}
