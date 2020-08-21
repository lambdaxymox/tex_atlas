use image::png;
use image::{ImageDecoder};
use serde_derive::{Deserialize, Serialize};
use zip::ZipArchive;

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
    /// Pixel is 8-bit luminance.
    L8,
    /// Pixel is 8-bit luminance with an alpha channel.
    La8,
    /// Pixel contains 8-bit R, G, and B channels.
    Rgb8,
    /// Pixel is an 8-bit RGB pixel with an 8-bit alpha channel.
    Rgba8,
    /// Pixel is 16-bit luminance.
    L16,
    /// Pixel is 16-bit luminance with an alpha channel.
    La16,
    /// Pixel is 16-bit RGB.
    Rgb16,
    /// Pixel is 16-bit RGBA.
    Rgba16,
    /// Pixel contains 8-bit B, G, and R channels.
    Bgr8,
    /// Pixel is 8-bit BGR with an 8-bit alpha channel.
    Bgra8,

}

impl ColorType {
    #[inline]
    pub fn bytes_per_pixel(self) -> usize {
        match self {
            ColorType::L8 => 1,
            ColorType::L16 => 2,
            ColorType::La8 => 2,
            ColorType::Rgb8 => 3,
            ColorType::Bgr8 => 3,
            ColorType::Rgba8 => 4,
            ColorType::Bgra8 => 4,
            ColorType::La16 => 4,
            ColorType::Rgb16 => 6,
            ColorType::Rgba16 => 8,
        }
    }

    #[inline]
    pub fn channel_count(self) -> usize {
        match self {
            ColorType::L8 => 1,
            ColorType::L16 => 1,
            ColorType::La8 => 2,
            ColorType::Rgb8 => 3,
            ColorType::Bgr8 => 3,
            ColorType::Rgba8 => 4,
            ColorType::Bgra8 => 4,
            ColorType::La16 => 2,
            ColorType::Rgb16 => 3,
            ColorType::Rgba16 => 4,
        }
    }

    #[inline]
    pub fn bits_per_pixel(self) -> usize {
        8 * self.bytes_per_pixel()
    }

    #[inline]
    pub fn has_alpha_channel(self) -> bool {
        match self {
            ColorType::L8 => false,
            ColorType::L16 => false,
            ColorType::La8 => true,
            ColorType::Rgb8 => false,
            ColorType::Bgr8 => false,
            ColorType::Rgba8 => true,
            ColorType::Bgra8 => true,
            ColorType::La16 => true,
            ColorType::Rgb16 => false,
            ColorType::Rgba16 => true,
        }
    }
}

/// The kind of error generated by the encoding or decoding process for texture atlases.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    /// The atlas image has an unsupported color space.
    UnrecognizedColorType,
    /// The texture atlas could not be opened
    CouldNotOpenTextureAtlas,
    /// The coordinate chart for the texture atlas is corrupted.
    CouldNotLoadCoordinateCharts,
    /// The image buffer for the texture atlas is corrupted.
    CouldNotLoadAtlasImageBuffer,
    /// The underlying image representation uses 32 bit floats instead of bytes vectors for the pixels.
    Got32BitFloatingPointImageInsteadOfByteImage,
    /// The image buffer for the texture atlas is missing.
    MissingImageBuffer,
    /// The coordinate charts for the atlas are missing.
    MissingCoordinateCharts,
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
            ErrorKind::MissingImageBuffer => {
                write!(f, "{}", "Texture atlas is missing image buffer.")
            }
            ErrorKind::MissingCoordinateCharts => {
                write!(f, "{}", "Texture atlas is missing coordinate data.")
            }
        }
    }
}

/// The internal representation of a texture atlas error.
#[derive(Debug)]
struct Repr {
    kind: ErrorKind,
    name: String,
    error: Option<Box<dyn error::Error + Send + Sync>>,
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture Atlas `{}` generated error `{}`: {:?}", self.name, self.kind, self.error)
    }
}

/// An error type that represents the possible failures during the reading,
/// writing, parsing, and encoding of a texture atlas.
pub struct TextureAtlas2DError {
    repr: Repr,
}

impl TextureAtlas2DError {
    pub fn new(kind: ErrorKind, name: String, error: Option<Box<dyn error::Error + Send + Sync + 'static>>) -> TextureAtlas2DError {
        TextureAtlas2DError {
            repr: Repr {
                kind: kind,
                name: name,
                error: error,
            }
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.repr.kind
    }

    pub fn atlas_name(&self) -> &str {
        &self.repr.name
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

/// The `Origin` parameter determines which part of the underlying texture atlas 
/// image is considered the origin of the image. That is, when trying to render the 
/// texture atlas in a graphics application, this parameter tells the texture 
/// atlas parser how to format the atlas image for rendering. Geometrically, there are two 
/// equivalence classes of orientations possible for the atlas. Each origin
/// represents one of the equivalence classes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Origin {
    /// The atlas image starts in the top left corner of the image, with the x-axis 
    /// pointing right, and the y-axis pointing down.
    TopLeft,
    /// The atlas image starts in the bottom right corner of the image, with the x-axis 
    /// pointing right, and the y-axis pointing up.
    BottomLeft,
}

/// Possible warnings generated when parsing a texture image atlas. These include things 
/// that are not strictly errors, but could degrade the performance of the texture atlas 
/// in graphics applications. The default case is that no warnings occurred.
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
    #[inline]
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
    pub top_left: OffsetTexCoords,
    /// The width of the bounding box.
    pub width: f32,
    // The height of the bounding box.
    pub height: f32,
}

impl BoundingBoxTexCoords {
    #[inline]
    fn new(top_left: OffsetTexCoords, width: f32, height: f32) -> BoundingBoxTexCoords {
        BoundingBoxTexCoords {
            top_left: top_left,
            width: width,
            height: height,
        }
    }
}

/// The corners of a bounding box in the texture atlas. This is an equivalent 
/// representation to the usual representation in terms of the top left 
/// corner, the width, and the height of the bounding box.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BoundingBoxCornersTexCoords {
    /// The top left corner of th bounding box.
    pub top_left: OffsetTexCoords,
    /// The top right corner of the bounding box.
    pub top_right: OffsetTexCoords,
    /// The bottom left corner of the bounding box.
    pub bottom_left: OffsetTexCoords,
    /// The bottom right corner of the bounding box.
    pub bottom_right: OffsetTexCoords,
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
    #[inline]
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

/// The corners of a bounding box in the texture atlas. This is an equivalent 
/// representation to the usual representation in terms of the top left 
/// corner, the width, and the height of the bounding box.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BoundingBoxCornersPixelCoords {
    /// The top left corner of the bounding box.
    pub top_left: OffsetPixelCoords,
    /// The top right corner of the bounding box.
    pub top_right: OffsetPixelCoords,
    /// The bottom left corner of the bounding box.
    pub bottom_left: OffsetPixelCoords,
    /// The bottom right corner of the bounding box.
    pub bottom_right: OffsetPixelCoords,
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

    #[inline]
    fn len_pixels(&self) -> usize {
        self.width * self.height
    }

    #[inline]
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
    /// The bounding box for the texture in units of the unit square [0,1] x [0,1].
    bounding_box_tex: BoundingBoxTexCoords,
    /// The bounding box for the texture in units of pixels.
    bounding_box_pix: BoundingBoxPixelCoords,
}

impl AtlasEntry {
    /// Construct a new atlas entry.
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

/// A data structure representing all the data for a texture atlas.
#[derive(Clone, Debug)]
pub struct TextureAtlas2D {
    /// The width of the texture atlas in pixels.
    pub width: usize,
    /// The height of the texture atlas in pixel.
    pub height: usize,
    /// The number of channels per pixel.
    pub channel_count: usize,
    /// The number of bytes per pixel.
    pub bytes_per_pixel: usize,
    /// The color space of the atlas image.
    pub color_type: ColorType,
    /// The origin in the atlas image for orienting the texture.
    origin: Origin,
    /// The table of texture names.
    texture_names: HashMap<String, usize>,
    /// The bounding boxes for each texture in the texture atlas.
    bounding_boxes: HashMap<usize, AtlasEntry>,
    /// The name of the texture atlas.
    atlas_name: String,
    /// The underlying texture image.
    data: TextureImage2D,
}

impl TextureAtlas2D {
    /// Construct a new texture atlas.
    pub fn new(
        width: usize, height: usize, color_type: ColorType, origin: Origin, 
        entries: Vec<(usize, String, BoundingBoxPixelCoords)>, atlas_name: String, data: Vec<u8>) -> TextureAtlas2D {
        
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

        let mut texture_names = HashMap::new();
        for i in 0..bounding_boxes.len() {
            let texture_name = bounding_boxes[&i].name.clone();
            texture_names.insert(texture_name, i);
        }
        
        TextureAtlas2D {
            width: width,
            height: height,
            channel_count: image_data.channel_count,
            bytes_per_pixel: image_data.bytes_per_pixel,
            color_type: color_type,
            origin: origin,
            texture_names: texture_names,
            bounding_boxes: bounding_boxes,
            atlas_name: atlas_name,
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
        self.texture_names.len()
    }

    /// Get the position of the origin in the texture atlas.
    #[inline]
    pub fn origin(&self) -> Origin {
        self.origin
    }

    /// Get the name of the texture atlas.
    #[inline]
    pub fn atlas_name(&self) -> &str {
        &self.atlas_name
    }

    /// Get the set of all texture names for the textures inside the 
    /// texture atlas.
    pub fn texture_names(&self) -> Vec<&str> {
        self.texture_names.keys().map(|s| s.as_str()).collect()
    }

    /// Get the set of all texture indices for the textures inside
    /// the texture atlas.
    pub fn indices(&self) -> Vec<usize> {
        self.bounding_boxes.keys().map(|i| *i).collect()
    }

    /// Get the bounding box in units of pixels for a texture by name.
    pub fn by_texture_name(&self, name: &str) -> Option<BoundingBoxPixelCoords> {
        match self.texture_names.get(name) {
            Some(index) => Some(self.bounding_boxes[index].bounding_box_pix),
            None => None,
        }
    }

    /// Get the bounding box in units of the unit square for a texture by name.
    pub fn by_texture_name_uv(&self, name: &str) -> Option<BoundingBoxTexCoords> {
        match self.texture_names.get(name) {
            Some(index) => Some(self.bounding_boxes[index].bounding_box_tex),
            None => None,
        }
    }

    /// Get the bounding box in units of pixels for a texture by index.
    pub fn by_index(&self, index: usize) -> Option<BoundingBoxPixelCoords> {
        if index < self.bounding_boxes.len() {
            Some(self.bounding_boxes[&index].bounding_box_pix)
        } else {
            None
        }
    }

    /// Get the bounding box in units of the unit square for a texture by index.
    pub fn by_index_uv(&self, index: usize) -> Option<BoundingBoxTexCoords> {
        if index < self.bounding_boxes.len() {
            Some(self.bounding_boxes[&index].bounding_box_tex)
        } else {
            None
        }
    }

    /// Get the bounding box in units of pixels for a given texture by index.
    pub fn by_index_corners(&self, index: usize) -> Option<BoundingBoxCornersPixelCoords> {
        self.by_index(index).map(|bounding_box| {
            let width = bounding_box.width;
            let height = bounding_box.height;
            let top_left = bounding_box.top_left;
            let bottom_left = OffsetPixelCoords::new(top_left.u, top_left.v - height);
            let top_right = OffsetPixelCoords::new(top_left.u + width, top_left.v);
            let bottom_right = OffsetPixelCoords::new(top_left.u + width, top_left.v - height);
    
            BoundingBoxCornersPixelCoords {
                top_left: top_left,
                top_right: top_right,
                bottom_left: bottom_left,
                bottom_right: bottom_right,
            }
        })
    }

    /// Get the bounding box in units of the unit square for a given texture by index.
    pub fn by_index_corners_uv(&self, index: usize) -> Option<BoundingBoxCornersTexCoords> {
        self.by_index(index).map(|bounding_box| {
            let atlas_width = self.width;
            let atlas_height = self.height;
            let width = bounding_box.width;
            let height = bounding_box.height;
            let top_left = bounding_box.top_left;
            let bottom_left = OffsetTexCoords::new(
                (top_left.u as f32) / (atlas_width as f32), ((top_left.v - height) as f32) / (atlas_height as f32)
            );
            let top_right = OffsetTexCoords::new(
                ((top_left.u + width) as f32) / (atlas_width as f32), (top_left.v as f32) / (atlas_height as f32)
            );
            let bottom_right = OffsetTexCoords::new(
                ((top_left.u + width) as f32) / (atlas_width as f32), ((top_left.v - height) as f32) / (atlas_height as f32)
            );
            let top_left = OffsetTexCoords::new(
                top_left.u as f32 / atlas_width as f32, top_left.v as f32 / atlas_height as f32
            );
    
            BoundingBoxCornersTexCoords {
                top_left: top_left,
                top_right: top_right,
                bottom_left: bottom_left,
                bottom_right: bottom_right,
            }
        })
    }

    /// Get the bounding box in units of pixels for a given texture by name.
    pub fn by_texture_name_corners(&self, name: &str) -> Option<BoundingBoxCornersPixelCoords> {
        self.by_texture_name(name).map(|bounding_box| {
            let width = bounding_box.width;
            let height = bounding_box.height;
            let top_left = bounding_box.top_left;
            let bottom_left = OffsetPixelCoords::new(top_left.u, top_left.v - height);
            let top_right = OffsetPixelCoords::new(top_left.u + width, top_left.v);
            let bottom_right = OffsetPixelCoords::new(top_left.u + width, top_left.v - height);
    
            BoundingBoxCornersPixelCoords {
                top_left: top_left,
                top_right: top_right,
                bottom_left: bottom_left,
                bottom_right: bottom_right,
            }
        })
    }

    /// Get the bounding box in units of the unit square for a given texture by name.
    pub fn by_texture_name_corners_uv(&self, name: &str) -> Option<BoundingBoxCornersTexCoords> {
        self.by_texture_name(name).map(|bounding_box| {
            let atlas_width = self.width;
            let atlas_height = self.height;
            let width = bounding_box.width;
            let height = bounding_box.height;
            let top_left = bounding_box.top_left;
            let bottom_left = OffsetTexCoords::new(
                (top_left.u as f32) / (atlas_width as f32), ((top_left.v - height) as f32) / (atlas_height as f32)
            );
            let top_right = OffsetTexCoords::new(
                ((top_left.u + width) as f32) / (atlas_width as f32), (top_left.v as f32) / (atlas_height as f32)
            );
            let bottom_right = OffsetTexCoords::new(
                ((top_left.u + width) as f32) / (atlas_width as f32), ((top_left.v - height) as f32) / (atlas_height as f32)
            );
            let top_left = OffsetTexCoords::new(
                top_left.u as f32 / atlas_width as f32, top_left.v as f32 / atlas_height as f32
            );
    
            BoundingBoxCornersTexCoords {
                top_left: top_left,
                top_right: top_right,
                bottom_left: bottom_left,
                bottom_right: bottom_right,
            }
        })
    }

    /// Get the collection of all bounding boxes for the textures inside the 
    /// texture atlas.
    fn coordinate_charts(&self) -> TextureAtlas2DSerialization {
        let mut coordinate_charts = HashMap::new();
        for name in self.texture_names.keys() {
            let name_str = name.clone();
            let index = self.texture_names[name.as_str()];
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

/// A data structure storing a collection of texture atlases. In a multi-texture atlas we denote
/// each atlas as a page.
#[derive(Clone, Debug)]
pub struct MultiTextureAtlas2D {
    /// The set of texture atlases.
    pages: Vec<TextureAtlas2D>,
    /// The names of each texture atlas in the multi-texture atlas.
    page_names: HashMap<String, usize>,
}

impl MultiTextureAtlas2D {
    /// Construct a new multi-texture atlas. 
    pub fn new(pages: Vec<TextureAtlas2D>, names: Vec<String>) -> MultiTextureAtlas2D {
        let mut page_names = HashMap::new();
        for i in 0..names.len() {
            page_names.insert(names[i].clone(), i);
        }

        MultiTextureAtlas2D {
            pages: pages,
            page_names: page_names,
        }
    }

    /// Get all the pages in the multi-texture atlas.
    #[inline]
    pub fn pages(&self) -> &[TextureAtlas2D] {
        &self.pages
    }

    /// Get a texture atlas by its name.
    pub fn by_page_name(&self, name: &str) -> Option<&TextureAtlas2D> {
        if let Some(index) = self.page_names.get(name) {
            return Some(&self.pages[*index]);
        }

        None
    }

    /// Get a texture atlas by its index.
    pub fn by_page_index(&self, index: usize) -> Option<&TextureAtlas2D> {
        if index <= self.pages.len() {
            Some(&self.pages[index])
        } else {
            None
        }
    }

    /// Get the number of pages (atlases) in the multi texture atlas.
    #[inline]
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Get the names of the texture atlases in the multi texture atlas.
    #[inline]
    pub fn page_names(&self) -> impl Iterator<Item = &str> {
        self.page_names.keys().map(|s| s.as_str())
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
    /// Check that no warnings were generated during the loading of a texture atlas.
    pub fn no_warnings_generated(&self) -> bool {
        self.warnings == TextureAtlas2DWarning::NoWarnings
    }
}

pub struct MultiTextureAtlas2DResult {
    pub multi_atlas: MultiTextureAtlas2D,
    pub warnings: Vec<TextureAtlas2DWarning>,
}

/// Orient the texture atlas image depending on the position of the origin.
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
        TextureAtlas2DError::new(kind, String::from(""), Some(Box::new(e)))
    })?;
    let (width, height) = png_reader.dimensions();
    let (width, height) = (width as usize, height as usize);
    let color_type = match png_reader.color_type() {
        image::ColorType::Rgba8 => ColorType::Rgba8,
        _ => {
            let kind = ErrorKind::UnrecognizedColorType;
            return Err(TextureAtlas2DError::new(kind, String::from(""), None));
        }
    };
    let bytes_per_pixel = png_reader.color_type().bytes_per_pixel() as usize;
    let mut image_data: Vec<u8> = vec![0; width * height * bytes_per_pixel];
    png_reader.read_image(&mut image_data).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadAtlasImageBuffer;
        TextureAtlas2DError::new(kind, String::from(""), Some(Box::new(e)))
    })?;

    let width_in_bytes = bytes_per_pixel * width;
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

fn atlas_from_reader<R: io::Read + io::Seek>(zip_reader: &mut ZipArchive<R>, page_name: &str) -> Result<TextureAtlas2DResult, TextureAtlas2DError> {
    let coordinate_charts_name = format!("{}.json", page_name);
    let coordinate_charts_file = zip_reader.by_name(&coordinate_charts_name).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadCoordinateCharts;
        TextureAtlas2DError::new(kind, String::from(page_name), Some(Box::new(e)))
    })?;
    let atlas_chart_data: TextureAtlas2DSerialization = serde_json::from_reader(coordinate_charts_file).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadCoordinateCharts;
        TextureAtlas2DError::new(kind, String::from(page_name), Some(Box::new(e)))
    })?;
    let image_file_name = format!("{}.png", page_name);
    let image_file = zip_reader.by_name(&image_file_name).map_err(|e| {
        let kind = ErrorKind::CouldNotLoadAtlasImageBuffer;
        TextureAtlas2DError::new(kind, String::from(page_name), Some(Box::new(e)))
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
    let atlas_name = String::from(page_name);
    let atlas = TextureAtlas2D::new(width, height, color_type, origin, atlas_entries, atlas_name, tex_image.data);

    Ok(TextureAtlas2DResult {
        atlas: atlas,
        warnings: warnings,
    })
}

fn extract_atlas_names<R: io::Read + io::Seek>(zip_reader: &ZipArchive<R>) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut atlas_names = vec![];
    let mut atlases_missing_coordinates = vec![];
    let mut atlases_missing_images = vec![];
    let mut file_names = zip_reader
        .file_names()
        .filter(|file_name| file_name.ends_with(".json") || file_name.ends_with(".png"))
        .collect::<Vec<&str>>();
    file_names.sort();

    let mut i = 0;
    while i < file_names.len() {
        if file_names[i].ends_with(".json") && file_names[i + 1].ends_with(".png") {
            // The atlas contains both the coordinate chart file and the atlas image file.
            let length = file_names[i].len() - 5;
            let atlas_name = String::from(&file_names[i][..length]);
            atlas_names.push(atlas_name);
            i += 2;
        } else if file_names[i].ends_with(".json") && !file_names[i + i].ends_with(".png") {
            // The atlas has the coordinate chart file but lacks the atlas image file.
            let length = file_names[i].len() - 5;
            let atlas_name = String::from(&file_names[i][..length]);
            atlases_missing_images.push(atlas_name);
            i += 1;
        } else if !file_names[i].ends_with(".json") && file_names[i + i].ends_with(".png") {
            // The atlas lacks the coordinate chart but has the atlas image file.
            let length = file_names[i].len() - 4;
            let atlas_name = String::from(&file_names[i][..length]);
            atlases_missing_coordinates.push(atlas_name);
            i += 1;
        } else {
            // We should not get here. The code before the loop that filtered and sorted the file list 
            // should have taken care of this.
            i += 1;
        }
    }

    (atlas_names, atlases_missing_coordinates, atlases_missing_images)
}

/// Load a multi texture atlas from a readable endpoint. This primarily includes files and buffers in memory.
pub fn from_reader<R: io::Read + io::Seek>(reader: R) -> Result<MultiTextureAtlas2DResult, TextureAtlas2DError> {
    let mut zip_reader = zip::ZipArchive::new(reader).map_err(|e| {
        let kind = ErrorKind::CouldNotOpenTextureAtlas;
        TextureAtlas2DError::new(kind, String::from(""), Some(Box::new(e)))
    })?;
    let (
        mut atlas_names, 
        atlases_missing_coordinates, 
        atlases_missing_images) = extract_atlas_names(&zip_reader);

    if !atlases_missing_coordinates.is_empty() {
        let kind = ErrorKind::MissingCoordinateCharts;
        let name = atlases_missing_coordinates[0].clone();
        return Err(TextureAtlas2DError::new(kind, name, None));
    }
    if !atlases_missing_images.is_empty() {
        let kind = ErrorKind::MissingImageBuffer;
        let name = atlases_missing_images[0].clone();
        return Err(TextureAtlas2DError::new(kind, name, None));
    }

    let mut pages = vec![];
    let mut page_names = vec![];
    let mut warnings = vec![];
    for atlas_name in atlas_names.drain(..) {
        let result = atlas_from_reader(&mut zip_reader, &atlas_name);
        match result {
            Ok(atlas_result) => {
                pages.push(atlas_result.atlas);
                warnings.push(atlas_result.warnings);
                page_names.push(atlas_name);
            }
            Err(e) => return Err(e)
        }
    }

    let multi_atlas = MultiTextureAtlas2D::new(pages, page_names);
    Ok(MultiTextureAtlas2DResult {
        multi_atlas: multi_atlas,
        warnings: warnings,
    })
}

/// Write a multi texture atlas out to any writable endpoint. This 
/// includes files and buffers in memory.
pub fn to_writer<W: io::Write + io::Seek>(writer: W, multi_atlas: &MultiTextureAtlas2D) -> io::Result<()> {
    let mut zip_file = zip::ZipWriter::new(writer);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for atlas in multi_atlas.pages() {
        // Write out the coordinate charts.
        zip_file.start_file(format!("{}.json", &atlas.atlas_name), options)?;
        serde_json::to_writer_pretty(&mut zip_file, &atlas.coordinate_charts())?;

        // If the origin is the bottom left of the image, we need to flip the image back over
        // before writing it out. PNG images index start from the top left corner of
        // the image.
        let mut image = atlas.image().clone();
        let bytes_per_pixel = atlas.color_type.bytes_per_pixel();
        let width_in_bytes =  bytes_per_pixel * atlas.width;
        orient_image(&mut image.data, atlas.origin, atlas.height, width_in_bytes);

        // Write out the atlas image.
        zip_file.start_file(format!("{}.png", &atlas.atlas_name), options)?;
        let png_writer = png::PNGEncoder::new(&mut zip_file);
        let height = atlas.height as u32;
        let width = atlas.width as u32;
        let color = image::ColorType::Rgba8;
        png_writer.encode(image.as_bytes(), width, height, color).map_err(
            |e| io::Error::new(io::ErrorKind::Other, Box::new(e))
        )?;
    }

    zip_file.finish()?;

    Ok(())
}

/// Load a texture atlas from a reader or buffer.
pub fn load_from_memory(buffer: &[u8]) -> Result<MultiTextureAtlas2DResult, TextureAtlas2DError> {
    let reader = io::Cursor::new(buffer);
    from_reader(reader)   
}

/// Load a texture atlas directly from a file.
pub fn load_file<P: AsRef<Path>>(path: P) -> Result<MultiTextureAtlas2DResult, TextureAtlas2DError> {
    let reader = File::open(&path).map_err(|e|{
        let kind = ErrorKind::CouldNotOpenTextureAtlas;
        TextureAtlas2DError::new(kind, String::from(""), Some(Box::new(e)))
    })?;
    from_reader(reader)
}

/// Write a texture atlas direct to a file.
pub fn write_to_file<P: AsRef<Path>>(path: P, multi_atlas: &MultiTextureAtlas2D) -> io::Result<()> {
    // Set up the image zip archive.
    let mut file_path = path.as_ref().to_path_buf();
    file_path.set_extension("atlas");
    let file = File::create(&file_path)?;

    // Write out the atlas contents.
    to_writer(file, multi_atlas)
}
