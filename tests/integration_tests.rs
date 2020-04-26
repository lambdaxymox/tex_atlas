use tex_atlas;
use tex_atlas::{ColorType, Origin, PixelBoundingBox, PixelOffset, TextureAtlas2D};


const SAMPLE_DATA: &str = "assets/sample.atlas";


fn to_byte_vec(vec: Vec<u32>) -> Vec<u8> {
    let mut acc = vec![];
    for item in vec.iter() {
        let byte_0 = ((item & 0xFF000000) >> 24) as u8;
        let byte_1 = ((item & 0x00FF0000) >> 16) as u8;
        let byte_2 = ((item & 0x0000FF00) >>  8) as u8;
        let byte_3 = ((item & 0x000000FF) >>  0) as u8;
        acc.push(byte_0);
        acc.push(byte_1);
        acc.push(byte_2);
        acc.push(byte_3);
    }

    acc
}

/// The integration test data expected from the sample image.
/// NOTE: 
/// 0xFF0000FF = { R: 255, G:   0, B:   0, A: 255 }
/// 0x00FF00FF = { R:   0, G: 255, B:   0, A: 255 }
/// 0x0000FFFF = { R:   0, G:   0, B: 255, A: 255 }
/// 0x000000FF = { R:   0, G:   0, B    0, A: 255 }
fn atlas() -> TextureAtlas2D {
    let width = 16;
    let height = 16;
    let color_type = ColorType::Rgba8;
    let origin = Origin::BottomLeft;
    let data: Vec<u8> = to_byte_vec(vec![
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
    ]);
    let names = vec![format!("sample")];
    let top_left = PixelOffset { u: 0, v: 15 };
    let pixel_offsets = vec![PixelBoundingBox { top_left: top_left, width: width, height: height }];
    
    TextureAtlas2D::new(width, height, color_type, origin, names, pixel_offsets, data)
}

/// The file loader yields the correct width.
#[test]
fn load_file_yields_correct_width() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let expected = atlas();

    assert_eq!(result.width, expected.width);
}

/// The file loader yields the correct width.
#[test]
fn load_file_yields_correct_height() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let expected = atlas();

    assert_eq!(result.height, expected.height);
}

/// The file loader yields the correct number of color channels.
#[test]
fn load_file_yields_correct_pixel_channel_count() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let expected = atlas();

    assert_eq!(result.channel_count, expected.channel_count);
}

/// The number of pixels in the loaded image matches the number
/// of pixels in the expected image.
#[test]
fn load_file_yields_correct_pixel_count() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let expected = atlas();

    assert_eq!(result.len_pixels(), expected.len_pixels());
}

/// The number of pixels in the image matches the width * height. That is, it satisfies
/// 
/// `number of pixels == width * height.`
#[test]
fn height_times_width_equals_pixel_count() {
    let atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let height = atlas.height as usize;
    let width = atlas.width as usize;
    let pixel_count = atlas.len_pixels();

    assert_eq!(width * height, pixel_count);
}

/// The file loader yields the correct data block.
#[test]
fn load_file_yields_correct_data_block() {
    let result_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let expected_atlas = atlas();
    let result = result_atlas.as_bytes();
    let expected = expected_atlas.as_bytes();

    assert_eq!(result, expected);
}

/// The file loader yields the expected texture image.
#[test]
fn load_file_yields_correct_texture_atlas_data() {
    let result_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().atlas;
    let expected_atlas = atlas();
    let result = result_atlas.as_bytes();
    let expected = expected_atlas.as_bytes();

    assert_eq!(result, expected);
}
