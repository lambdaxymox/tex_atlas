# Texture Atlas Library
This software repository is a library for working with texture atlases. It is a library for working with 
the `TexureAtlas2D` image format (defined by this library) for loading two-dimensional texture atlases 
in computer graphics and game programming applications.

# Usage
To use the `tex_atlas` library, add the following line to your `Cargo.toml` file.
```toml
[dependencies]
tex_atlas = { git = "https://github.com/lambdaxymox/tex_atlas" }
```
Then include the library at the top of the `main.rs` or `lib.rs` file for your project.
```rust
use tex_atlas; 
```
After that, you can load texture atlas files (with the `*.atlas` file extension) using the library. 

# Specification
See the specification document for details on the structure of the file format. Put briefly, each atlas file
is a Zip archive consisting of a JSON file describing where each texture is located in the atlas, as well as 
a png file containing the atlas image itself.
