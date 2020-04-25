#![feature(vec_into_raw_parts)]
use stb_image::image;
use stb_image::image::LoadResult;
use std::path::Path;
use std::error::Error;
use std::fmt;
use std::mem;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
        RGBA { r: r, g: g, b: b, a: a }
    }
}

impl Default for RGBA {
    #[inline]
    fn default() -> RGBA {
        RGBA::new(0, 0, 0, 255)
    }
}

impl From<u32> for RGBA {
    #[inline]
    fn from(val: u32) -> RGBA {
        RGBA {
            r: ((val & 0xFF000000) >> 24) as u8,
            g: ((val & 0x00FF0000) >> 16) as u8,
            b: ((val & 0x0000FF00) >> 8) as u8,
            a: ((val & 0x000000FF) >> 0) as u8,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TexImage2D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub data: Vec<RGBA>,
}

impl TexImage2D {
    pub fn new(width: u32, height: u32) -> TexImage2D {
        TexImage2D {
            width: width,
            height: height,
            depth: 4,
            data: vec![RGBA::default(); (width * height) as usize],
        }
    }

    pub fn from_rgba_data(width: u32, height: u32, data: Vec<RGBA>) -> TexImage2D {
        TexImage2D {
            width: width,
            height: height,
            depth: 4,
            data: data,
        }
    }

    pub fn pixel_count(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        &self.data[0].r
    }
}

impl<'a> From<&'a image::Image<u8>> for TexImage2D {
    fn from(image: &'a image::Image<u8>) -> TexImage2D {
        let mut data = vec![];
        for chunk in image.data.chunks(4) {
            data.push(RGBA::new(chunk[0], chunk[1], chunk[2], chunk[3]));
        }

        TexImage2D {
            width: image.width as u32,
            height: image.height as u32,
            depth: image.depth as u32,
            data: data,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextureAtlas2DError {
    CouldNotLoadImageBuffer,
    Got32BitFloatingPointImageInsteadOfByteImage,
}

impl fmt::Display for TextureAtlas2DError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TextureAtlas2DError::CouldNotLoadImageBuffer => {
                write!(f, "{}", "Could not load image buffer.")
            }
            TextureAtlas2DError::Got32BitFloatingPointImageInsteadOfByteImage => {
                write!(f, "{}", "Tried to load an image as byte vectors, got 32 bit floating point image instead.")
            }
        }
    }
}

impl Error for TextureAtlas2DError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextureAtlas2DWarning {
    NoWarnings,
    TextureDimensionsAreNotAPowerOfTwo,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UVOffset {
    pub u: f32,
    pub v: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UVBoundingBox {
    top_left: UVOffset,
    width: f32,
    height: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PixelOffset {
    pub u: usize,
    pub v: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PixelBoundingBox {
    top_left: PixelOffset,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug)]
pub struct TextureAtlas2D<T> {
    width: usize,
    height: usize,
    depth: usize,
    names: Vec<String>,
    uv_offsets: Vec<UVOffset>,
    pixel_offsets: Vec<PixelOffset>,
    data: Vec<T>,
}

#[derive(Clone, Debug)]
pub struct TextureAtlas2DResult {
    pub atlas: TextureAtlas2D<RGBA>,
    pub warnings: TextureAtlas2DWarning,
}

impl TextureAtlas2DResult {
    pub fn has_no_warnings(&self) -> bool {
        self.warnings == TextureAtlas2DWarning::NoWarnings
    }
}

/// Load a PNG texture image from a reader or buffer.
pub fn load_from_memory(buffer: &[u8]) -> Result<TextureAtlas2DResult, TextureAtlas2DError> {
    let force_channels = 4;
    let mut image_data = match image::load_from_memory_with_depth(buffer, force_channels, false) {
        LoadResult::ImageU8(image_data) => image_data,
        LoadResult::Error(_) => {
            return Err(TextureAtlas2DError::CouldNotLoadImageBuffer);
        }
        LoadResult::ImageF32(_) => {
            return Err(TextureAtlas2DError::Got32BitFloatingPointImageInsteadOfByteImage);
        }
    };

    let width = image_data.width;
    let height = image_data.height;
    let depth = image_data.depth;

    // Check that the image size is a power of two.
    let warnings = if (width & (width - 1)) != 0 || (height & (height - 1)) != 0 {
        TextureAtlas2DWarning::TextureDimensionsAreNotAPowerOfTwo
    } else {
        TextureAtlas2DWarning::NoWarnings
    };

    let width_in_bytes = 4 *width;
    let half_height = height / 2;
    for row in 0..half_height {
        for col in 0..width_in_bytes {
            let temp = image_data.data[row * width_in_bytes + col];
            image_data.data[row * width_in_bytes + col] = image_data.data[((height - row - 1) * width_in_bytes) + col];
            image_data.data[((height - row - 1) * width_in_bytes) + col] = temp;
        }
    }

    let tex_image_data = unsafe { 
        let (old_ptr, old_length, old_capacity) = image_data.data.into_raw_parts();
        let ptr = mem::transmute::<*mut u8, *mut RGBA>(old_ptr);
        let length = old_length / 4;
        let capacity = old_capacity / 4;
        Vec::from_raw_parts(ptr, length, capacity)
    };
    let atlas = TextureAtlas2D {
        width: width,
        height: height,
        depth: depth,
        names: vec![],
        uv_offsets: vec![],
        pixel_offsets: vec![],
        data: tex_image_data,
    };

    Ok(TextureAtlas2DResult {
        atlas: atlas,
        warnings: warnings,
    })
}

/*
/// Load a PNG texture image from a file name.
pub fn load_file<P: AsRef<Path>>(file_path: P) -> Result<TexImage2DResult, TexImage2DError> {
    let force_channels = 4;
    let mut image_data = match image::load_with_depth(&file_path, force_channels, false) {
        LoadResult::ImageU8(image_data) => image_data,
        LoadResult::Error(_) => {
            return Err(TexImage2DError::CouldNotLoadImageBuffer);
        }
        LoadResult::ImageF32(_) => {
            return Err(TexImage2DError::Got32BitFloatingPointImageInsteadOfByteImage);
        }
    };

    let width = image_data.width;
    let height = image_data.height;

    // Check that the image size is a power of two.
    let warnings = if (width & (width - 1)) != 0 || (height & (height - 1)) != 0 {
        TexImage2DWarning::TextureDimensionsAreNotAPowerOfTwo
    } else {
        TexImage2DWarning::NoWarnings
    };

    let width_in_bytes = 4 * width;
    let half_height = height / 2;
    for row in 0..half_height {
        for col in 0..width_in_bytes {
            let temp = image_data.data[row * width_in_bytes + col];
            image_data.data[row * width_in_bytes + col] = image_data.data[((height - row - 1) * width_in_bytes) + col];
            image_data.data[((height - row - 1) * width_in_bytes) + col] = temp;
        }
    }

    let tex_image_data = unsafe { 
        let (old_ptr, old_length, old_capacity) = image_data.data.into_raw_parts();
        let ptr = mem::transmute::<*mut u8, *mut RGBA>(old_ptr);
        let length = old_length / 4;
        let capacity = old_capacity / 4;
        Vec::from_raw_parts(ptr, length, capacity)
    };
    let tex_image = TexImage2D::from_rgba_data(width as u32, height as u32, tex_image_data);
    let result = TexImage2DResult {
        image: tex_image,
        warnings: warnings,
    };

    Ok(result)
}
*/


#[cfg(test)]
mod tests {
    use super::RGBA;


    #[test]
    fn test_u32_to_rgba_conversion() {
        let val = 0x12345678;
        let result = super::RGBA::from(val);
        let expected = RGBA::new(0x12, 0x34, 0x56, 0x78);

        assert_eq!(result, expected);
    }
}
