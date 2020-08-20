use tex_atlas;
use tex_atlas::{
    ColorType, 
    Origin, 
    BoundingBoxPixelCoords, 
    OffsetPixelCoords, 
    MultiTextureAtlas2D, 
    TextureAtlas2D
};


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
fn multi_atlas() -> MultiTextureAtlas2D {
    let width = 16;
    let height = 16;
    let texture_width = 8;
    let texture_height = 8;
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
    let names = vec![format!("red"), format!("green"), format!("blue"), format!("black")];
    let bounding_boxes = vec![
        BoundingBoxPixelCoords { top_left: OffsetPixelCoords { u: 0, v: 15 }, width: texture_width, height: texture_height },
        BoundingBoxPixelCoords { top_left: OffsetPixelCoords { u: 8, v: 15 }, width: texture_width, height: texture_height },
        BoundingBoxPixelCoords { top_left: OffsetPixelCoords { u: 0, v:  7 }, width: texture_width, height: texture_height },
        BoundingBoxPixelCoords { top_left: OffsetPixelCoords { u: 8, v:  7 }, width: texture_width, height: texture_height },
    ];
    let indices = vec![0, 1, 2, 3];
    let atlas_entries = vec![
        (indices[0], names[0].clone(), bounding_boxes[0]), (indices[1], names[1].clone(), bounding_boxes[1]), 
        (indices[2], names[2].clone(), bounding_boxes[2]), (indices[3], names[3].clone(), bounding_boxes[3]),
    ];
    let atlas_name = String::from("atlas");
    let atlas = TextureAtlas2D::new(width, height, color_type, origin, atlas_entries, atlas_name.clone(), data);
    let pages = vec![atlas];
    let page_names = vec![atlas_name];

    MultiTextureAtlas2D::new(pages, page_names)
}

/// The sample file exists.
#[test]
fn sample_file_exists() {
    let multi_atlas = tex_atlas::load_file(SAMPLE_DATA);
    assert!(multi_atlas.is_ok());
}

/// The file loader yields the correct number of atlases.
#[test]
fn load_files_parses_the_correct_number_of_atlases() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    let expected = multi_atlas();

    assert_eq!(result.page_count(), expected.page_count());
}

/// The file loader parses all atlas names correctly.
#[test]
fn load_files_parses_all_atlases_by_name() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    for page_name in result.page_names() {
        let result_page = result.by_page_name(page_name);
        let expected_page = result.by_page_name(page_name);
        assert!(result_page.is_some());
        assert!(expected_page.is_some());
    }
}

/// The file loader yields the correct width of each texture atlas.
#[test]
fn load_file_yields_correct_width() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    let expected = multi_atlas();
    for page_name in result.page_names() {
        let result_page = result.by_page_name(page_name).unwrap();
        let expected_page = expected.by_page_name(page_name).unwrap();

        assert_eq!(result_page.width, expected_page.width);
    }
}

/// The file loader yields the correct width.
#[test]
fn load_file_yields_correct_height() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    let expected = multi_atlas();
    for page_name in result.page_names() {
        let result_page = result.by_page_name(page_name).unwrap();
        let expected_page = expected.by_page_name(page_name).unwrap();

        assert_eq!(result_page.height, expected_page.height);
    }
}

/// The file loader yields the correct number of color channels.
#[test]
fn load_file_yields_correct_pixel_channel_count() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    let expected = multi_atlas();
    for page_name in result.page_names() {
        let result_page = result.by_page_name(page_name).unwrap();
        let expected_page = expected.by_page_name(page_name).unwrap();

        assert_eq!(result_page.channel_count, expected_page.channel_count);
    }
}

/// The number of pixels in the loaded image matches the number of pixels in the expected image.
#[test]
fn load_file_yields_correct_pixel_count() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    let expected = multi_atlas();
    for page_name in expected.page_names() {
        let result_page = result.by_page_name(page_name).unwrap();
        let expected_page = expected.by_page_name(page_name).unwrap();

        assert_eq!(result_page.len_pixels(), expected_page.len_pixels());
    }
}

/// The number of pixels in the image matches the width * height. That is, it satisfies
/// 
/// `number of pixels == width * height.`
#[test]
fn height_times_width_equals_pixel_count() {
    let multi_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    for page in multi_atlas.pages() {
        let height = page.height as usize;
        let width = page.width as usize;
        let pixel_count = page.len_pixels();

        assert_eq!(width * height, pixel_count);
    }
}

/// The number of bytes in the image matches the width * height * bytes per pixel. That is, it satisfies
/// 
/// `number of bytes == width * height * bytes per pixel.`
#[test]
fn height_times_width_equals_length_in_bytes() {
    let multi_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    for page in multi_atlas.pages() {
        let height = page.height;
        let width = page.width;
        let bytes_per_pixel = page.bytes_per_pixel;

        assert_eq!(width * height * bytes_per_pixel, page.len_bytes());
    }
}

/// The file loader yields the correct data block.
#[test]
fn load_file_yields_correct_data_block() {
    let result = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    let expected = multi_atlas();
    for page_name in result.page_names() {
        let result_page = result.by_page_name(page_name).unwrap();
        let expected_page = expected.by_page_name(page_name).unwrap();
        let result_bytes = result_page.as_bytes();
        let expected_bytes = expected_page.as_bytes();

        assert_eq!(result_bytes, expected_bytes);
    }
}

/// Every texture name in every atlas that we parse from the multi atlas files should exist
/// in the multi texture atlas.
#[test]
fn each_texture_in_the_multi_atlas_exists() {
    let multi_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    for page in multi_atlas.pages() {
        for texture_name in page.texture_names() {
            assert!(page.by_texture_name(texture_name).is_some(), "{} : {}", page.atlas_name(), texture_name);
        }
    }
}

/// Textures that are not present in the atlas should not be found by the query
/// methods.
#[test]
fn each_texture_absent_from_the_atlas_does_not_exist() {
    let multi_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    let texture_name = "DOES NOT EXIST";
    for page in multi_atlas.pages() {
        assert!(page.by_texture_name(texture_name).is_none());
    }
}

/// Every texture has a corresponding bounding box.
#[test]
fn every_texture_corresponding_bounding_box() {
    let multi_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    for page in multi_atlas.pages() {
        for i in 0..page.texture_count() {
            assert!(page.by_index(i).is_some());
        }
    }
}

/// Every bounding box in units of pixels should have a corresponding
/// bounding box in the unit square in texture coordinates.
#[test]
fn every_pixel_bounding_box_has_a_corresponding_uv_bounding_box() {
    let multi_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    for page in multi_atlas.pages() {
        for i in 0..page.texture_count() {
            assert!(page.by_index_uv(i).is_some());
        }
    }
}

/// The texture atlas decoder correctly parses the names and pixel bounding boxes 
/// of the textures in the atlas.
#[test]
fn resulting_texture_atlas_entries_match_expected_atlas_entries_by_name() {
    let result_multi_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    let expected_multi_atlas = multi_atlas();
    for page in expected_multi_atlas.pages() {
        let page_name = page.atlas_name();
        let result_page = result_multi_atlas.by_page_name(page_name).unwrap();
        for texture_name in page.texture_names() {
            let expected = page.by_texture_name(texture_name);
            let result = result_page.by_texture_name(texture_name);
            assert_eq!(result, expected);
        }
    }
}

/// The texture atlas decoder correctly parses the names and pixel bounding boxes 
/// of the textures in the atlas.
#[test]
fn resulting_texture_atlas_entries_match_expected_atlas_entries_by_index() {
    let result_multi_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    let expected_multi_atlas = multi_atlas();
    for page_name in expected_multi_atlas.page_names() {
        let expected_page = expected_multi_atlas.by_page_name(page_name).unwrap();
        let result_page = result_multi_atlas.by_page_name(page_name).unwrap();
        for i in 0..result_page.texture_count() {
            let expected = expected_page.by_index(i);
            let result = result_page.by_index(i);
            assert_eq!(result, expected);
        }
    }
}

/// The multi texture atlas decoder correctly parses the names and texture coordinate
///  bounding boxes of the textures in each texture atlas.
#[test]
fn resulting_texture_atlas_entries_match_expected_atlas_entries_by_name_tex() {
    let result_multi_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    let expected_multi_atlas = multi_atlas();
    for page_name in expected_multi_atlas.page_names() {
        let expected_page = expected_multi_atlas.by_page_name(page_name).unwrap();
        let result_page = result_multi_atlas.by_page_name(page_name).unwrap();
        for texture_name in expected_page.texture_names() {
            let expected = expected_page.by_texture_name_uv(texture_name);
            let result = result_page.by_texture_name_uv(texture_name);
            assert_eq!(result, expected);
        }
    }
}

/// The multi texture atlas decoder correctly parses the names and texture coordinate 
/// bounding boxes of the textures in each texture atlas.
#[test]
fn resulting_texture_atlas_entries_match_expected_atlas_entries_by_index_tex() {
    let result_multi_atlas = tex_atlas::load_file(SAMPLE_DATA).unwrap().multi_atlas;
    let expected_multi_atlas = multi_atlas();
    for page_name in expected_multi_atlas.page_names() {
        let expected_page = expected_multi_atlas.by_page_name(page_name).unwrap();
        let result_page = result_multi_atlas.by_page_name(page_name).unwrap();        
        for i in 0..expected_page.texture_count() {
            let expected = expected_page.by_index_uv(i);
            let result = result_page.by_index_uv(i);
            assert_eq!(result, expected);
        }
    }
}
