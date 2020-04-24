#![feature(vec_into_raw_parts)]
use stb_image::image;
use stb_image::image::LoadResult;
use std::path::Path;
use std::error::Error;
use std::fmt;
use std::mem;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r: r, g: g, b: b, a: a }
    }
}

impl Default for Rgba {
    #[inline]
    fn default() -> Rgba {
        Rgba::new(0, 0, 0, 255)
    }
}

impl From<u32> for Rgba {
    #[inline]
    fn from(val: u32) -> Rgba {
        Rgba {
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
    pub data: Vec<Rgba>,
}

impl TexImage2D {
    pub fn new(width: u32, height: u32) -> TexImage2D {
        TexImage2D {
            width: width,
            height: height,
            depth: 4,
            data: vec![Rgba::default(); (width * height) as usize],
        }
    }

    pub fn from_rgba_data(width: u32, height: u32, data: Vec<Rgba>) -> TexImage2D {
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
            data.push(Rgba::new(chunk[0], chunk[1], chunk[2], chunk[3]));
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
pub enum TexImage2DError {
    CouldNotLoadImageBuffer,
    Got32BitFloatingPointImageInsteadOfByteImage,
}

impl fmt::Display for TexImage2DError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TexImage2DError::CouldNotLoadImageBuffer => {
                write!(f, "{}", "Could not load image buffer.")
            }
            TexImage2DError::Got32BitFloatingPointImageInsteadOfByteImage => {
                write!(f, "{}", "Tried to load an image as byte vectors, got 32 bit floating point image instead.")
            }
        }
    }
}

impl Error for TexImage2DError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TexImage2DWarning {
    NoWarnings,
    TextureDimensionsAreNotAPowerOfTwo,
}

#[derive(Clone, Debug)]
pub struct TexImage2DResult {
    pub image: TexImage2D,
    pub warnings: TexImage2DWarning,
}

impl TexImage2DResult {
    pub fn has_no_warnings(&self) -> bool {
        self.warnings == TexImage2DWarning::NoWarnings
    }
}

/// Load a PNG texture image from a reader or buffer.
pub fn load_from_memory(buffer: &[u8]) -> Result<TexImage2DResult, TexImage2DError> {
    let force_channels = 4;
    let mut image_data = match image::load_from_memory_with_depth(buffer, force_channels, false) {
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
        let ptr = mem::transmute::<*mut u8, *mut Rgba>(old_ptr);
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
        let ptr = mem::transmute::<*mut u8, *mut Rgba>(old_ptr);
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

#[cfg(test)]
mod tests {
    use super::Rgba;


    #[test]
    fn test_u32_to_rgba_conversion() {
        let val = 0x12345678;
        let result = super::Rgba::from(val);
        let expected = Rgba::new(0x12, 0x34, 0x56, 0x78);

        assert_eq!(result, expected);
    }
}
