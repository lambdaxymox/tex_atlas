use image::png;
use image::{ImageDecoder};
use serde_derive::{Deserialize, Serialize};

use std::path::Path;
use std::error::Error;
use std::fmt;
use std::io;
use std::fs::File;
use std::collections::hash_map::HashMap;
use std::error;


/// The color space represented by the underlying image data.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ColorType {
    Rgba8,
}

impl ColorType {
    pub fn bytes_per_pixel(self) -> usize {
        match self {
            ColorType::Rgba8 => 4,
        }
    }

    pub fn channel_count(self) -> usize {
        match self {
            ColorType::Rgba8 => 4,
        }
    }

    pub fn bits_per_pixel(self) -> usize {
        match self {
            ColorType::Rgba8 => 32,
        }
    }
}

/// The kind of error generated by the encoding or decoding process for texture atlases.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    UnrecognizedColorType,
    CouldNotLoadImageBuffer,
    Got32BitFloatingPointImageInsteadOfByteImage,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::UnrecognizedColorType => {
                write!(f, "{}", "The image buffer has an unrecognized color format.")
            }
            ErrorKind::CouldNotLoadImageBuffer => {
                write!(f, "{}", "Could not load image buffer.")
            }
            ErrorKind::Got32BitFloatingPointImageInsteadOfByteImage => {
                write!(f, "{}", "Tried to load an image as byte vectors, got 32 bit floating point image instead.")
            }
        }
    }
}

/// The internal representation of a texture atlas error.
#[derive(Debug)]
struct Repr {
    kind: ErrorKind,
    error: Option<Box<dyn error::Error + Send + Sync>>,
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.kind, self.error)
    }
}

/// An error type that represents the possible failures during the reading,
/// writing, parsing, and encoding of a texture atlas.
pub struct TextureAtlas2DError {
    repr: Repr,
}

impl TextureAtlas2DError {
    pub fn new(kind: ErrorKind, error: Option<Box<dyn error::Error + Send + Sync + 'static>>) -> TextureAtlas2DError {
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
/// Geometrically, there are two equivalence classes of orientations possible for the atlas. Each origin
/// represents one of the equivalence classes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Origin {
    /// The atlas image starts in the top left corner of the image, with the x-axis pointing right,
    /// and the y-axis pointing down.
    TopLeft,
    /// The atlas image starts in the bottom right corner of the image, with the x-axis pointing right,
    /// and the y-axis pointing up.
    BottomLeft,
}

/// Possible warnings generated when parsing a texture image atlas. These include things that are not strictly 
/// errors, but could degrade the performance of the texture atlas in graphics applications. The default case is
/// that no warnings occurred.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextureAtlas2DWarning {
    /// No warnings occurred.
    NoWarnings,
    /// The texture image dimensins are not a power of two. Texture image dimensions that
    /// are a power of two are easier to index into for graphics hardware.
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
    pub top_left: PixelOffset,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug)]
struct TextureImage2D {
    width: usize,
    height: usize,
    channel_count: usize,
    bytes_per_pixel: usize,
    color_type: ColorType,
    data: Vec<u8>,
}

impl AsRef<[u8]> for TextureImage2D {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl TextureImage2D {
    #[inline]
    fn new(width: usize, height: usize, color_type: ColorType, data: Vec<u8>) -> TextureImage2D {
        TextureImage2D {
            width: width,
            height: height,
            channel_count: color_type.channel_count(),
            bytes_per_pixel: color_type.bytes_per_pixel(),
            color_type: color_type,
            data: data,
        }
    }

    fn len_pixels(&self) -> usize {
        self.width * self.height
    }

    fn len_bytes(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct TextureAtlas2D {
    pub width: usize,
    pub height: usize,
    pub channel_count: usize,
    pub bytes_per_pixel: usize,
    pub color_type: ColorType,
    pub origin: Origin,
    names: Vec<String>,
    uv_offsets: Vec<UVBoundingBox>,
    pixel_offsets: Vec<PixelBoundingBox>,
    data: TextureImage2D,
}

impl TextureAtlas2D {
    /// Construct a texture atlas.
    pub fn new(
        width: usize, height: usize, color_type: ColorType, origin: Origin, 
        names: Vec<String>, pixel_offsets: Vec<PixelBoundingBox>, data: Vec<u8>) -> TextureAtlas2D {
        
        let image_data = TextureImage2D::new(width, height, color_type, data);

        TextureAtlas2D {
            width: width,
            height: height,
            channel_count: image_data.channel_count,
            bytes_per_pixel: image_data.bytes_per_pixel,
            color_type: color_type,
            origin: origin,
            names: names,
            uv_offsets: vec![],
            pixel_offsets: pixel_offsets,
            data: image_data,
        }
    }

    /// Get the length of texture atlas image in units of the number of pixels.
    #[inline]
    pub fn len_pixels(&self) -> usize {
        self.data.len_pixels()
    }

    /// Get the length of the texture atlas image in units of bytes.
    #[inline]
    pub fn len_bytes(&self) -> usize {
        self.data.len_bytes()
    }

    fn image(&self) -> &TextureImage2D {
        &self.data
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    /// Get a view into the texture atlas image as a byte slice
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.data.as_bytes()
    }

    /// Get the number of textures in the texture atlas.
    #[inline]
    pub fn texture_count(&self) -> usize {
        self.names.len()
    }

    /// Get the collection of all bounding boxes for the textures inside the 
    /// texture atlas.
    pub fn coordinate_charts(&self) -> HashMap<&str, PixelBoundingBox> {
        let mut charts = HashMap::new();
        for i in 0..self.texture_count() {
            let name = self.names[i].as_str();
            let bounding_box = self.pixel_offsets[i];
            charts.insert(name, bounding_box);
        }

        charts
    }

    /// Get the set of all texture names for the textures inside the 
    /// texture atlas.
    pub fn names(&self) -> Vec<&str> {
        self.names.iter().map(|s| s.as_str()).collect()
    }

    pub fn get_by_name(name: &str) -> Option<PixelBoundingBox> {
        None
    }

    pub fn get_by_name_uv(name: &str) -> Option<UVBoundingBox> {
        None
    }

    pub fn get_by_index(&self, index: usize) -> Option<PixelBoundingBox> {
        if index > self.pixel_offsets.len() {
            Some(self.pixel_offsets[index])
        } else {
            None
        }
    }

    pub fn get_by_index_uv(&self, index: usize) -> Option<UVBoundingBox> {
        if index > self.pixel_offsets.len() {
            Some(self.uv_offsets[index])
        } else {
            None
        }
    }
}

/// This type bundles together a texture atlas and any possible warnings generated
/// from encoding or decoding a texture atlas. Warnings are properties that are not
/// errors but can degrade the performance of working with the texture atlas.
#[derive(Clone, Debug)]
pub struct TextureAtlas2DResult {
    /// The texture atlas we decoded.
    pub atlas: TextureAtlas2D,
    /// Any warnings generated in the decoding process.
    pub warnings: TextureAtlas2DWarning,
}

impl TextureAtlas2DResult {
    pub fn has_no_warnings(&self) -> bool {
        self.warnings == TextureAtlas2DWarning::NoWarnings
    }
}

/// Load an atlas image file from a reader.
fn load_image_from_reader<R: io::Read>(reader: R) -> Result<TextureImage2D, TextureAtlas2DError> {
    let png_reader = png::PngDecoder::new(reader).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Some(Box::new(e)))
    })?;
    let (width, height) = png_reader.dimensions();
    let (width, height) = (width as usize, height as usize);
    let color_type = match png_reader.color_type() {
        image::ColorType::Rgba8 => ColorType::Rgba8,
        _ => {
            let kind = ErrorKind::UnrecognizedColorType;
            return Err(TextureAtlas2DError::new(kind, None));
        }
    };
    let bytes_per_pixel = png_reader.color_type().bytes_per_pixel() as usize;
    let mut image_data: Vec<u8> = vec![0; width * height * bytes_per_pixel];
    png_reader.read_image(&mut image_data).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Some(Box::new(e)))
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

    let tex_image = TextureImage2D::new(width, height, color_type, image_data);

    Ok(tex_image)
}

/// Load a PNG texture image from a reader or buffer.
fn load_image_from_memory(buffer: &[u8]) -> Result<TextureImage2D, TextureAtlas2DError> {
    let reader = io::Cursor::new(buffer);
    load_image_from_reader(reader)   
}

/// Load a PNG texture image from a file name.
fn load_image_from_file<P: AsRef<Path>>(file_path: P) -> Result<TextureImage2D, TextureAtlas2DError> {
    let reader = File::open(file_path).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Some(Box::new(e)))
    })?;
    load_image_from_reader(reader)
}

/// Load a texture atlas from any endpoint that can be read from. This include files
/// and buffers in memory.
pub fn from_reader<R: io::Read + io::Seek>(reader: R) -> Result<TextureAtlas2DResult, TextureAtlas2DError> {
    let mut zip_reader = zip::ZipArchive::new(reader).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Some(Box::new(e)))
    })?;
    let coordinate_charts_file = zip_reader.by_name("coordinate_charts.json").map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Some(Box::new(e)))
    })?;
    let coordinate_charts: HashMap<String, PixelBoundingBox> = serde_json::from_reader(coordinate_charts_file).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Some(Box::new(e)))
    })?;
    let image_file = zip_reader.by_name("atlas.png").map_err(|e| {
        let kind = ErrorKind::CouldNotLoadImageBuffer;
        TextureAtlas2DError::new(kind, Some(Box::new(e)))
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
        channel_count: tex_image.channel_count,
        bytes_per_pixel: tex_image.bytes_per_pixel,
        color_type: tex_image.color_type,
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

/// Write a texture atlas out to any writable endpoint. This includes files
/// and buffers in memory.
pub fn to_writer<W: io::Write + io::Seek>(writer: W, atlas: &TextureAtlas2D) -> io::Result<()> {
    let mut zip_file = zip::ZipWriter::new(writer);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    // Write out the coordinate charts.
    zip_file.start_file("coordinate_charts.json", options)?;
    serde_json::to_writer_pretty(&mut zip_file, &atlas.coordinate_charts())?;

    // if the origin is the bottom left of the image, we need to flip the image back over
    // before writing it out. PNG images index starting from the top left corner of
    // the image.
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
        image.as_bytes(), atlas.width as u32, atlas.height as u32, image::ColorType::Rgba8
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, Box::new(e)))?;

    zip_file.finish()?;

    Ok(())
}

/// Load a texture atlas directly from a file.
pub fn load_file<P: AsRef<Path>>(path: P) -> Result<TextureAtlas2DResult, TextureAtlas2DError> {
    let reader = File::open(&path).unwrap();
    from_reader(reader)
}

/// Write a texture atlas direct to a file.
pub fn write_to_file<P: AsRef<Path>>(path: P, atlas: &TextureAtlas2D) -> io::Result<()> {
    // Set up the image zip archive.
    let mut file_path = path.as_ref().to_path_buf();
    file_path.set_extension("atlas");
    let file = File::create(&file_path)?;

    // Write out the atlas contents.
    to_writer(file, atlas)
}
