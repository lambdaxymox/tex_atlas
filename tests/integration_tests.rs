use tex_atlas;
use tex_atlas::{RGBA, TextureAtlas2D};


const SAMPLE_DATA: &str = "assets/sample.png";

/// The integration test data expected from the sample image.
/// NOTE: 
/// 0xFF0000FF = { R: 255, G:   0, B:   0, A: 255 }
/// 0x00FF00FF = { R:   0, G: 255, B:   0, A: 255 }
/// 0x0000FFFF = { R:   0, G:   0, B: 255, A: 255 }
/// 0x000000FF = { R:   0, G:   0, B    0, A: 255 }
fn atlas() -> TextureAtlas2D<RGBA> {
    let width = 16;
    let height = 16;
    let depth = 4;
    let data: Vec<RGBA> = vec![
        0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF,
        0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF,
        0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF,
        0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF,
        0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF,
        0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF,
        0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF,
        0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x0000FFFF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF,
        0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF,
        0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF,
        0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF,
        0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF,
        0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF,
        0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF,
        0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF,
        0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0xFF0000FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF, 0x00FF00FF,
    ].iter().map(|c| RGBA::from(*c)).collect();

    TextureAtlas2D::from_rgba_data(width, height, data)
}

/// The file loader yields the correct width.
#[test]
fn test_load_file_yields_correct_width() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let expected = atlas();

    assert_eq!(result.width, expected.width);
}

/// The file loader yields the correct width.
#[test]
fn test_load_file_yields_correct_height() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let expected = atlas();

    assert_eq!(result.height, expected.height);
}

/// The number of pixels in the loaded image matches the number
/// of pixels in the expected image.
#[test]
fn test_load_file_yields_correct_pixel_count() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let expected = atlas();

    assert_eq!(result.pixel_count(), expected.pixel_count());
}

/// The number of pixels in the image matches the width * height. That is, it satisfies
/// 
/// `number of pixels == width * height.`
#[test]
fn test_height_times_width_equals_pixel_count() {
    let atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let height = atlas.height as usize;
    let width = atlas.width as usize;
    let pixel_count = atlas.pixel_count();

    assert_eq!(width * height, pixel_count);
}

/// The file loader yields the correct data block.
#[test]
fn test_load_file_yields_correct_data_block() {
    let result_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let expected_atlas = atlas();
    let result = result_atlas.image();
    let expected = expected_atlas.image();

    assert_eq!(result, expected);
}

/// The file loader yields the expected texture image.
#[test]
fn test_load_file_yields_correct_texture_image() {
    let result_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let expected_atlas = atlas();
    let result = result_atlas.image();
    let expected = expected_atlas.image();

    assert_eq!(result, expected);
}
