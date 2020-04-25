#![feature(vec_into_raw_parts)]
use image::png;
use image::{ColorType, ImageDecoder};
use serde_derive::{Deserialize, Serialize};

use std::path::Path;
use std::error::Error;
use std::fmt;
use std::mem;
use std::io;
use std::fs::File;
use std::slice;
use std::collections::hash_map::HashMap;
use std::error;



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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    CouldNotLoadImageBuffer,
    Got32BitFloatingPointImageInsteadOfByteImage,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::CouldNotLoadImageBuffer => {
                write!(f, "{}", "Could not load image buffer.")
            }
            ErrorKind::Got32BitFloatingPointImageInsteadOfByteImage => {
                write!(f, "{}", "Tried to load an image as byte vectors, got 32 bit floating point image instead.")
            }
        }
    }
}

#[derive(Debug)]
struct Repr {
    kind: ErrorKind,
    error: Box<dyn error::Error + Send + Sync>,
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.error)
    }
}

/// A `Error` is an error typing representing the results of the failure of
/// a read or write operation.
pub struct TextureAtlas2DError {
    repr: Repr,
}

impl TextureAtlas2DError {
    pub fn new(kind: ErrorKind, error: Box<dyn error::Error + Send + Sync>) -> TextureAtlas2DError {
        TextureAtlas2DError {
            repr: Repr {
                kind: kind,
                error: error,
            }
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.repr.kind
    }
}

impl fmt::Debug for TextureAtlas2DError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

impl fmt::Display for TextureAtlas2DError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.repr, f)
    }
}

impl error::Error for TextureAtlas2DError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// The `Origin` parameter determines which part of the underlying texture atlas image is considered
/// the origin of the image. That is, when trying to render the texture atlas in a graphics application,
/// this parameter tells the texture atlas parser how to format the atlas image for rendering.
/// Geometrically, there are two equivalence classes of orientations each equivalent to one of the origin
/// positions in two dimensions.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Origin {
    /// The atlas image starts in the top left corner of the image, with the x-axis pointing right,
    /// and the y-axis pointing down.
    TopLeft,
    /// The atlas image starts in the bottom right corner of the image, with the x-axis pointing right,
    /// and the y-axis pointing up.
    BottomLeft,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextureAtlas2DWarning {
    NoWarnings,
    TextureDimensionsAreNotAPowerOfTwo,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UVOffset {
    pub u: f32,
    pub v: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UVBoundingBox {
    top_left: UVOffset,
    width: f32,
    height: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PixelOffset {
    pub u: usize,
    pub v: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PixelBoundingBox {
    top_left: PixelOffset,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug)]
struct TextureImage2D<T> {
    width: usize,
    height: usize,
    depth: usize,
    data: Vec<T>,
}

impl<T> TextureImage2D<T> {
    fn from_rgba_data(width: usize, height: usize, data: Vec<RGBA>) -> TextureImage2D<RGBA> {
        TextureImage2D {
            width: width,
            height: height,
            depth: 4,
            data: data,
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl TextureImage2D<RGBA> {
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        &self.data[0].r
    }

    fn as_bytes(&self) -> &[u8] {
        let ptr: *const u8 = &self.data[0].r;
        let len_bytes = self.width * self.height * self.depth;
        let bytes = unsafe { 
            slice::from_raw_parts(ptr, len_bytes)
        };

        bytes
    }
    
    fn len_bytes(&self) -> usize {
        mem::size_of::<RGBA>() * self.data.len()
    }
}

#[derive(Clone, Debug)]
pub struct TextureAtlas2D<T> {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    origin: Origin,
    names: Vec<String>,
    uv_offsets: Vec<UVBoundingBox>,
    pixel_offsets: Vec<PixelBoundingBox>,
    data: TextureImage2D<T>,
}

impl<T> TextureAtlas2D<T> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn image(&self) -> &TextureImage2D<T> {
        &self.data
    }
}

impl TextureAtlas2D<RGBA> {
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_bytes()
    }

    #[inline]
    pub fn texture_count(&self) -> usize {
        self.names.len()
    }

    pub fn from_rgba_data(width: usize, height: usize, origin: Origin, data: Vec<RGBA>) -> TextureAtlas2D<RGBA> {
        let image_data = TextureImage2D::<RGBA>::from_rgba_data(width, height, data);

        TextureAtlas2D {
            width: width,
            height: height,
            depth: 4,
            origin: origin, 
            names: vec![],
            uv_offsets: vec![],
            pixel_offsets: vec![],
            data: image_data,
        }
    }

    pub fn coordinate_charts(&self) -> HashMap<&str, PixelBoundingBox> {
        let mut charts = HashMap::new();
        for i in 0..self.texture_count() {
            let name = self.names[i].as_str();
            let bounding_box = self.pixel_offsets[i];
            charts.insert(name, bounding_box);
        }

        charts
    }
}

#[derive(Clone, Debug)]
pub struct TextureAtlas2DResult<T> {
    pub atlas: TextureAtlas2D<T>,
    pub warnings: TextureAtlas2DWarning,
}

impl<T> TextureAtlas2DResult<T> {
    pub fn has_no_warnings(&self) -> bool {
        self.warnings == TextureAtlas2DWarning::NoWarnings
    }
}

fn load_image_from_reader<R: io::Read>(reader: R) -> Result<TextureImage2D<RGBA>, TextureAtlas2DError> {
    let png_reader = png::PngDecoder::new(reader).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Box::new(e))
    })?;
    let (width, height) = png_reader.dimensions();
    let (width, height) = (width as usize, height as usize);
    let depth = png_reader.color_type().channel_count() as usize;
    let mut image_data: Vec<u8> = vec![0; width * height * depth];
    png_reader.read_image(&mut image_data).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Box::new(e))
    })?;

    let width_in_bytes = 4 * width;
    let half_height = height / 2;
    for row in 0..half_height {
        for col in 0..width_in_bytes {
            let temp = image_data[row * width_in_bytes + col];
            image_data[row * width_in_bytes + col] = image_data[((height - row - 1) * width_in_bytes) + col];
            image_data[((height - row - 1) * width_in_bytes) + col] = temp;
        }
    }

    let tex_image_data = unsafe { 
        let (old_ptr, old_length, old_capacity) = image_data.into_raw_parts();
        let ptr = mem::transmute::<*mut u8, *mut RGBA>(old_ptr);
        let length = old_length / 4;
        let capacity = old_capacity / 4;
        Vec::from_raw_parts(ptr, length, capacity)
    };
    let tex_image = TextureImage2D::<RGBA>::from_rgba_data(width, height, tex_image_data);

    Ok(tex_image)
}

/// Load a PNG texture image from a reader or buffer.
fn load_image_from_memory(buffer: &[u8]) -> Result<TextureImage2D<RGBA>, TextureAtlas2DError> {
    let reader = io::Cursor::new(buffer);
    load_image_from_reader(reader)   
}

/// Load a PNG texture image from a file name.
fn load_image_from_file<P: AsRef<Path>>(file_path: P) -> Result<TextureImage2D<RGBA>, TextureAtlas2DError> {
    let reader = File::open(file_path).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Box::new(e))
    })?;
    load_image_from_reader(reader)
}


pub fn from_reader<R: io::Read + io::Seek>(reader: R) -> Result<TextureAtlas2DResult<RGBA>, TextureAtlas2DError> {
    let mut zip_reader = zip::ZipArchive::new(reader).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Box::new(e))
    })?;
    let coordinate_charts_file = zip_reader.by_name("coordinate_charts.json").map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Box::new(e))
    })?;
    let coordinate_charts: HashMap<String, PixelBoundingBox> = serde_json::from_reader(coordinate_charts_file).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Box::new(e))
    })?;
    let image_file = zip_reader.by_name("atlas.png").map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Box::new(e))
    })?;
    let tex_image = load_image_from_reader(image_file)?;
    
    // Check that the image size is a power of two.
    let width = tex_image.width;
    let height = tex_image.height;
    let warnings = if (width & (width - 1)) != 0 || (height & (height - 1)) != 0 {
        TextureAtlas2DWarning::TextureDimensionsAreNotAPowerOfTwo
    } else {
        TextureAtlas2DWarning::NoWarnings
    };

    let names: Vec<String> = coordinate_charts.keys().map(|s| s.clone()).collect();
    let mut pixel_offsets: Vec<PixelBoundingBox> = vec![];
    for i in 0..names.len() {
        pixel_offsets.push(coordinate_charts[&names[i]]);
    }

    let atlas = TextureAtlas2D {
        width: tex_image.width,
        height: tex_image.height,
        depth: tex_image.depth,
        origin: Origin::BottomLeft,
        names: names,
        uv_offsets: vec![],
        pixel_offsets: pixel_offsets,
        data: tex_image,
    };

    Ok(TextureAtlas2DResult {
        atlas: atlas,
        warnings: warnings,
    })
}

pub fn to_writer<W: io::Write + io::Seek>(writer: W, atlas: &TextureAtlas2D<RGBA>) -> io::Result<()> {
    let mut zip_file = zip::ZipWriter::new(writer);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    // Write out the coordinate charts.
    zip_file.start_file("coordinate_charts.json", options)?;
    serde_json::to_writer_pretty(&mut zip_file, &atlas.coordinate_charts())?;

    // if the origin is the bottom left of the image, we need to flip the image back over
    // before writing it out.
    let mut image = atlas.image().clone();
    if atlas.origin == Origin::BottomLeft {
        let height = atlas.height;
        let width_in_bytes = 4 * atlas.width;
        let half_height = atlas.height / 2;
        for row in 0..half_height {
            for col in 0..width_in_bytes {
                let temp = image.data[row * width_in_bytes + col];
                image.data[row * width_in_bytes + col] = image.data[((height - row - 1) * width_in_bytes) + col];
                image.data[((height - row - 1) * width_in_bytes) + col] = temp;
            }
        }
    }

    // Write out the atlas image.
    zip_file.start_file("atlas.png", options)?;
    let png_writer = png::PNGEncoder::new(&mut zip_file);
    png_writer.encode(
        image.as_bytes(), atlas.width as u32, atlas.height as u32, ColorType::Rgba8
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, Box::new(e)))?;

    zip_file.finish()?;

    Ok(())
}

/// Load a texture atlas directly from a file.
pub fn load<P: AsRef<Path>>(path: P) -> Result<TextureAtlas2DResult<RGBA>, TextureAtlas2DError> {
    let reader = File::open(&path).unwrap();
    from_reader(reader)
}

pub fn write_to_file<P: AsRef<Path>>(path: P, atlas: &TextureAtlas2D<RGBA>) -> io::Result<()> {
    // Set up the image zip archive.
    let mut file_path = path.as_ref().to_path_buf();
    file_path.set_extension("atlas");
    let file = File::create(&file_path)?;

    // Write out the atlas contents.
    to_writer(file, atlas)
}


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
