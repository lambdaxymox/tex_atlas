use std::fs;
use std::io;
use std::path::Path;

use tex_atlas;


const SAMPLE_DATA: &str = "assets/example.atlas";


/// Loading an atlas file that does not exist should fail.
#[test]
fn loading_a_nonexistent_atlas_file_should_fail() {
    let path = Path::new("DoesNotExist.atlas");
    assert!(!path.exists());

    let maybe_multi_atlas = tex_atlas::load_file(path);
    assert!(maybe_multi_atlas.is_err());
}

/// Given a valid atlas file, we should be able to write it to storage.
#[test]
fn atlas_file_should_write_to_storage_successfully() {
    let multi_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    let path = Path::new("test.atlas");
    let result = tex_atlas::write_to_file(path, &multi_atlas);
    fs::remove_file(path).unwrap();

    assert!(result.is_ok());
}


struct ReadWriteTest {
    expected_multi_atlas: tex_atlas::MultiTextureAtlas2D,
    result_multi_atlas: tex_atlas::MultiTextureAtlas2D,
}

impl ReadWriteTest {
    fn new(
        expected_multi_atlas: tex_atlas::MultiTextureAtlas2D,
        result_multi_atlas: tex_atlas::MultiTextureAtlas2D) -> ReadWriteTest {

        ReadWriteTest {
            expected_multi_atlas: expected_multi_atlas,
            result_multi_atlas: result_multi_atlas,
        }
    }
}

fn read_write_test<P: AsRef<Path>>(expected_path: P) -> ReadWriteTest {
    let expected_multi_atlas = tex_atlas::load_file(&expected_path).unwrap().multi_atlas;
    let buffer = vec![];
    let mut cursor = io::Cursor::new(buffer);
    tex_atlas::to_writer(&mut cursor, &expected_multi_atlas).unwrap();
    let result_multi_atlas = tex_atlas::from_reader(&mut cursor, "").unwrap().multi_atlas;

    ReadWriteTest::new(expected_multi_atlas, result_multi_atlas)
}

/// Given a valid multi texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact multi texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The height in pixels of the atlas images should match.
#[test]
fn atlas_file_written_and_then_read_should_match_heights() {
    let test = read_write_test(SAMPLE_DATA);
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        let result_height = result_atlas.height;
        let expected_height = expected_atlas.height;

        assert_eq!(result_height, expected_height);
    }
}

/// Given a valid multi texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// given a multi texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The width in pixels of the atlas images should match.
#[test]
fn atlas_file_written_and_then_read_should_match_widths() {
    let test = read_write_test(SAMPLE_DATA);
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        let result_width = result_atlas.width;
        let expected_width = expected_atlas.width;

        assert_eq!(result_width, expected_width);
    }
}

/// Given a valid multi texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The origins of the atlases should match.
#[test]
fn atlas_file_written_and_then_read_should_match_origins() {
    let test = read_write_test(SAMPLE_DATA);
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        let result_origin = result_atlas.origin();
        let expected_origin = expected_atlas.origin();

        assert_eq!(result_origin, expected_origin);
    }
}

/// Given a valid multi texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The color space types of the atlases should match.
#[test]
fn atlas_file_written_and_then_read_should_match_color_types() {
    let test = read_write_test(SAMPLE_DATA);
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        let result_color_type = result_atlas.color_type;
        let expected_color_type = expected_atlas.color_type;

        assert_eq!(result_color_type, expected_color_type);
    }
}

/// Given a valid multi texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The color channel counts of the atlases should match.
#[test]
fn atlas_file_written_and_then_read_should_match_channel_counts() {
    let test = read_write_test(SAMPLE_DATA);
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        let result_channel_count = result_atlas.channel_count;
        let expected_channel_count = expected_atlas.channel_count;

        assert_eq!(result_channel_count, expected_channel_count);
    }
}

/// Given a valid multi texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The number of bytes per pixel of the atlases should match.
#[test]
fn atlas_file_written_and_then_read_should_match_bytes_per_channel() {
    let test = read_write_test(SAMPLE_DATA);
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        let result_bytes_per_pixel = result_atlas.bytes_per_pixel;
        let expected_bytes_per_pixel = expected_atlas.bytes_per_pixel;

        assert_eq!(result_bytes_per_pixel, expected_bytes_per_pixel);
    }
}

/// Given a valid multi texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// The number of textures in the atlases should match.
#[test]
fn atlas_file_written_and_then_read_should_match_texture_counts() {
    let test = read_write_test(SAMPLE_DATA);
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        let result_count = result_atlas.texture_count();
        let expected_count = expected_atlas.texture_count();

        assert_eq!(result_count, expected_count);
    }
}

/// Given a valid multi texture atlas, if we read it, write it to a new file, and read
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
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        for texture_name in expected_atlas.texture_names() {
            let result = result_atlas.by_texture_name(texture_name);
            let expected = expected_atlas.by_texture_name(texture_name);
            assert_eq!(result, expected);
        }
    }
}


/// Given a valid multi texture atlas, if we read it, write it to a new file, and read
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
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        for index in result_atlas.indices().iter() {
            let result = result_atlas.by_index(*index);
            let expected = expected_atlas.by_index(*index);
            assert_eq!(result, expected);
        }
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
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        let indices = result_atlas.indices();
        let names = result_atlas.texture_names();
        let result_zip = indices.iter().zip(names);
        for (index, name) in result_zip.filter(|(index, name)| { 
                result_atlas.by_index(**index) == result_atlas.by_texture_name(name) 
        }) {
            let expected_index = expected_atlas.by_index(*index); 
            let expected_name = expected_atlas.by_texture_name(name);
            assert_eq!(expected_index, expected_name);
        }
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
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
    
        assert_eq!(result_atlas.as_bytes(), expected_atlas.as_bytes());
    }
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
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
    
        assert_eq!(result_atlas.len_bytes(), expected_atlas.len_bytes());
    }
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
fn atlas_file_written_and_then_read_should_preserve_texture_names_uv() {
    let test = read_write_test(SAMPLE_DATA);
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        for texture_name in result_atlas.texture_names() {
            let result = result_atlas.by_texture_name_uv(texture_name);
            let expected = expected_atlas.by_texture_name_uv(texture_name);
            assert_eq!(result, expected);
        }
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
fn atlas_file_written_and_then_read_should_preserve_texture_indices_uv() {
    let test = read_write_test(SAMPLE_DATA);
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        for index in result_atlas.indices().iter() {
            let result = result_atlas.by_index_uv(*index);
            let expected = expected_atlas.by_index_uv(*index);
            assert_eq!(result, expected);
        }
    }
}

/// Given a valid multi texture atlas, if we read it, write it to a new file, and read
/// the new file back, we should get the same exact texture atlas back. That is, 
/// give a texture atlas, reading and writing should satisfy the relation
/// ```
/// read(write(read(file1), file2)), file2) == read(file1).
/// ```
/// Every texture index and texture name that match the same texture in the atlas should
/// still do so when the data is written out and read back.
#[test]
fn atlas_file_written_and_then_read_should_preserve_textures_uv() {
    let test = read_write_test(SAMPLE_DATA);
    for page_name in test.expected_multi_atlas.page_names() {
        let result_atlas = test.result_multi_atlas.by_page_name(page_name).unwrap();
        let expected_atlas = test.expected_multi_atlas.by_page_name(page_name).unwrap();
        let indices = result_atlas.indices();
        let names = result_atlas.texture_names();
        let result_zip = indices.iter().zip(names);
        for (index, name) in result_zip.filter(|(index, name)| { 
            result_atlas.by_index_uv(**index) == result_atlas.by_texture_name_uv(name) 
        }) {
            let expected_index = expected_atlas.by_index_uv(*index); 
            let expected_name = expected_atlas.by_texture_name_uv(name);
            assert_eq!(expected_index, expected_name);
        }
    }
}
