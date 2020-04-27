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
    CouldNotOpenTextureAtlas,
    CouldNotLoadCoordinateCharts,
    CouldNotLoadAtlasImageBuffer,
    Got32BitFloatingPointImageInsteadOfByteImage,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorKind::UnrecognizedColorType => {
                write!(f, "{}", "The image buffer has an unrecognized color format.")
            }
            ErrorKind::CouldNotLoadAtlasImageBuffer => {
                write!(f, "{}", "Could not load image buffer.")
            }
            ErrorKind::CouldNotLoadCoordinateCharts => {
                write!(f, "{}", "The atlas coordinate charts are invalid.")
            }
            ErrorKind::CouldNotOpenTextureAtlas => {
                write!(f, "{}", "The texture atlas file could not be opened or parsed.")
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
    /// The texture image dimensions are not a power of two. Texture image dimensions that
    /// are a power of two are easier to index into for graphics hardware.
    TextureDimensionsAreNotAPowerOfTwo,
}

/// The position of the top left corner of the bounding box in texture coordinates
/// of the unit square [0,1] x [0,1].
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OffsetTexCoords {
    /// The horizontal coordinate.
    pub u: f32,
    /// The vertical coordinate.
    pub v: f32,
}

impl OffsetTexCoords {
    fn new(u: f32, v: f32) -> OffsetTexCoords {
        OffsetTexCoords {
            u: u,
            v: v,
        }
    }
}

/// The parameters describing the position and dimensions of the bounding box
/// in texture corrdinates in the unit square [0,1] x [0,1].  
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoundingBoxTexCoords {
    /// The position of the top left corner of the bounding box.
    top_left: OffsetTexCoords,
    /// The width of the bounding box.
    width: f32,
    // The height of the bounding box.
    height: f32,
}

impl BoundingBoxTexCoords {
    fn new(top_left: OffsetTexCoords, width: f32, height: f32) -> BoundingBoxTexCoords {
        BoundingBoxTexCoords {
            top_left: top_left,
            width: width,
            height: height,
        }
    }
}

/// The position of the top left corner of the bounding box in
/// terms of the raw pixel position in the underlying image array.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffsetPixelCoords {
    /// The horizontal coordinate.
    pub u: usize,
    /// The vertical coordinate.
    pub v: usize,
}

impl OffsetPixelCoords {
    fn new(u: usize, v: usize) -> OffsetPixelCoords {
        OffsetPixelCoords {
            u: u,
            v: v,
        }
    }
}

/// The parameter that describe the position and dimensions of the bounding box
/// in terms of the location inside the underlying storage of the pixels.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundingBoxPixelCoords {
    /// The position of the top left corner of the bounding box.
    pub top_left: OffsetPixelCoords,
    /// The width in pixels of the bounding box.
    pub width: usize,
    /// The height in pixels of the bounding box.
    pub height: usize,
}

impl BoundingBoxPixelCoords {
    fn new(top_left: OffsetPixelCoords, width: usize, height: usize) -> BoundingBoxPixelCoords {
        BoundingBoxPixelCoords {
            top_left: top_left,
            width: width,
            height: height,
        }
    }
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

/// An atlas entry contains all the information about where a 
/// texture is located in the atlas image, and what the name of the
/// texture is.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct AtlasEntry {
    /// The name of the texture.
    name: String,
    /// The bounding box for the texture denominated in units of the unit square 
    /// [0,1] x [0,1].
    bounding_box_tex: BoundingBoxTexCoords,
    /// The bounding box for the texture denominated in units of pixels.
    bounding_box_pix: BoundingBoxPixelCoords,
}

impl AtlasEntry {
    fn new(name: String, 
        bounding_box_tex: BoundingBoxTexCoords, 
        bounding_box_pix: BoundingBoxPixelCoords) -> AtlasEntry {
        
        AtlasEntry {
            name: name,
            bounding_box_tex: bounding_box_tex,
            bounding_box_pix: bounding_box_pix,
        }
    }
}

/// A struct for organizing the serialization and deserialization of
/// a texture in the texture atlas.
#[derive(Serialize, Deserialize)]
struct TextureAtlas2DSerializationEntry {
    name: String,
    bounding_box: BoundingBoxPixelCoords,
}

impl TextureAtlas2DSerializationEntry {
    fn new(name: String, bounding_box: BoundingBoxPixelCoords) -> TextureAtlas2DSerializationEntry {
        TextureAtlas2DSerializationEntry {
            name: name,
            bounding_box: bounding_box,
        }
    } 
}

/// A struct for organizing the serialization and deserialization of a 
/// texture atlas.
#[derive(Serialize, Deserialize)]
struct TextureAtlas2DSerialization {
    origin: Origin,
    coordinate_charts: HashMap<usize, TextureAtlas2DSerializationEntry>,
}

impl TextureAtlas2DSerialization {
    fn new(origin: Origin, coordinate_charts: HashMap<usize, TextureAtlas2DSerializationEntry>) -> TextureAtlas2DSerialization {
        TextureAtlas2DSerialization {
            origin: origin,
            coordinate_charts: coordinate_charts,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextureAtlas2D {
    pub width: usize,
    pub height: usize,
    pub channel_count: usize,
    pub bytes_per_pixel: usize,
    pub color_type: ColorType,
    origin: Origin,
    names: HashMap<String, usize>,
    bounding_boxes: HashMap<usize, AtlasEntry>,
    data: TextureImage2D,
}

impl TextureAtlas2D {
    /// Construct a new texture atlas.
    pub fn new(
        width: usize, height: usize, color_type: ColorType, origin: Origin, 
        entries: Vec<(usize, String, BoundingBoxPixelCoords)>, data: Vec<u8>) -> TextureAtlas2D {
        
        let image_data = TextureImage2D::new(width, height, color_type, data);
        let mut bounding_boxes = HashMap::new();
        for (i, name_i, bounding_box_pix_i) in entries.iter() {
            let top_left_i = bounding_box_pix_i.top_left;
            let u = top_left_i.u as f32 / width as f32;
            let v = top_left_i.v as f32 / height as f32;
            let offset_tex_i = OffsetTexCoords::new(u, v);
            let width_tex_i = bounding_box_pix_i.width as f32 / width as f32;
            let height_tex_i = bounding_box_pix_i.height as f32 / height as f32;
            let bounding_box_tex_i = BoundingBoxTexCoords::new(offset_tex_i, width_tex_i, height_tex_i);
            let atlas_entry = AtlasEntry::new(name_i.clone(), bounding_box_tex_i, *bounding_box_pix_i);
            bounding_boxes.insert(*i, atlas_entry);
        }

        let mut tex_names = HashMap::new();
        for i in 0..bounding_boxes.len() {
            let tex_name = bounding_boxes[&i].name.clone();
            tex_names.insert(tex_name, i);
        }
        
        TextureAtlas2D {
            width: width,
            height: height,
            channel_count: image_data.channel_count,
            bytes_per_pixel: image_data.bytes_per_pixel,
            color_type: color_type,
            origin: origin,
            names: tex_names,
            bounding_boxes: bounding_boxes,
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

    /// Get the set of all texture names for the textures inside the 
    /// texture atlas.
    pub fn names(&self) -> Vec<&str> {
        self.names.keys().map(|s| s.as_str()).collect()
    }

    /// Get the bounding box in units of pixels for a texture by name.
    pub fn get_name(&self, name: &str) -> Option<BoundingBoxPixelCoords> {
        match self.names.get(name) {
            Some(index) => Some(self.bounding_boxes[index].bounding_box_pix),
            None => None,
        }
    }

    /// Get the bounding box in units of the unit square for a texture by name.
    pub fn get_name_uv(&self, name: &str) -> Option<BoundingBoxTexCoords> {
        match self.names.get(name) {
            Some(index) => Some(self.bounding_boxes[index].bounding_box_tex),
            None => None,
        }
    }

    /// Get the bounding box in units of pixels for a texture by index.
    pub fn get_index(&self, index: usize) -> Option<BoundingBoxPixelCoords> {
        if index < self.bounding_boxes.len() {
            Some(self.bounding_boxes[&index].bounding_box_pix)
        } else {
            None
        }
    }

    /// Get the bounding box in units of the unit square for a texture by index.
    pub fn get_index_uv(&self, index: usize) -> Option<BoundingBoxTexCoords> {
        if index < self.bounding_boxes.len() {
            Some(self.bounding_boxes[&index].bounding_box_tex)
        } else {
            None
        }
    }

    /// Get the collection of all bounding boxes for the textures inside the 
    /// texture atlas.
    fn coordinate_charts(&self) -> TextureAtlas2DSerialization {
        let mut coordinate_charts = HashMap::new();
        for name in self.names.keys() {
            let name_str = name.clone();
            let index = self.names[name.as_str()];
            let bounding_box = self.bounding_boxes[&index].bounding_box_pix;
            let entry = TextureAtlas2DSerializationEntry::new(name_str, bounding_box);
            coordinate_charts.insert(index, entry);
        }

        TextureAtlas2DSerialization::new(self.origin, coordinate_charts)
    }

    fn image(&self) -> &TextureImage2D {
        &self.data
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

/// Orient the texture atlas image depending on the 
/// position of the origin.
fn orient_image(image: &mut [u8], origin: Origin, height: usize, width_in_bytes: usize) {
    if origin == Origin::BottomLeft {
        let half_height = height / 2;
        for row in 0..half_height {
            for col in 0..width_in_bytes {
                let temp = image[row * width_in_bytes + col];
                image[row * width_in_bytes + col] = image[((height - row - 1) * width_in_bytes) + col];
                image[((height - row - 1) * width_in_bytes) + col] = temp;
            }
        }
    }
}

/// Load an atlas image file from a reader.
fn load_image_from_reader<R: io::Read>(reader: R) -> Result<TextureImage2D, TextureAtlas2DError> {
    let png_reader = png::PngDecoder::new(reader).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadAtlasImageBuffer;
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
        let kind = ErrorKind::CouldNotLoadAtlasImageBuffer;
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


/// Load a texture atlas from any endpoint that can be read from. This include files
/// and buffers in memory.
pub fn from_reader<R: io::Read + io::Seek>(reader: R) -> Result<TextureAtlas2DResult, TextureAtlas2DError> {
    let mut zip_reader = zip::ZipArchive::new(reader).map_err(|e| {
        let kind = ErrorKind::CouldNotOpenTextureAtlas;
        TextureAtlas2DError::new(kind, Some(Box::new(e)))
    })?;
    let coordinate_charts_file = zip_reader.by_name("coordinate_charts.json").map_err(|e| {
        let kind = ErrorKind::CouldNotLoadCoordinateCharts;
        TextureAtlas2DError::new(kind, Some(Box::new(e)))
    })?;
    let atlas_chart_data: TextureAtlas2DSerialization = serde_json::from_reader(coordinate_charts_file).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadCoordinateCharts;
        TextureAtlas2DError::new(kind, Some(Box::new(e)))
    })?;
    let image_file = zip_reader.by_name("atlas.png").map_err(|e| {
        let kind = ErrorKind::CouldNotLoadAtlasImageBuffer;
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

    let coordinate_charts = atlas_chart_data.coordinate_charts;
    let mut atlas_entries: Vec<(usize, String, BoundingBoxPixelCoords)> = vec![];
    for (i, chart_i) in coordinate_charts.iter() {
        atlas_entries.push((*i, chart_i.name.clone(), chart_i.bounding_box));
    }

    let color_type = tex_image.color_type;
    let origin = atlas_chart_data.origin;
    let atlas = TextureAtlas2D::new(width, height, color_type, origin, atlas_entries, tex_image.data);

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
    // before writing it out. PNG images index start from the top left corner of
    // the image.
    let mut image = atlas.image().clone();
    let origin = atlas.origin;
    let height = atlas.height;
    let width_in_bytes = 4 * atlas.width;
    orient_image(&mut image.data, origin, height, width_in_bytes);

    // Write out the atlas image.
    zip_file.start_file("atlas.png", options)?;
    let png_writer = png::PNGEncoder::new(&mut zip_file);
    png_writer.encode(
        image.as_bytes(), atlas.width as u32, atlas.height as u32, image::ColorType::Rgba8
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, Box::new(e)))?;

    zip_file.finish()?;

    Ok(())
}

/// Load a texture atlas from a reader or buffer.
pub fn load_from_memory(buffer: &[u8]) -> Result<TextureAtlas2DResult, TextureAtlas2DError> {
    let reader = io::Cursor::new(buffer);
    from_reader(reader)   
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
