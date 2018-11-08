use std::{
    slice::{
        from_raw_parts,
        from_raw_parts_mut
    },
    io,
    io::BufReader,
    fs::{File, read_dir},
    path::Path
};
use png::{
    ColorType,
    Decoder,
    DecodingError
};

use integral_geometry::Size;
use vec2d::Vec2D;

pub struct ThemeSprite {
    pixels: Vec2D<u32>
}

impl ThemeSprite {
    #[inline]
    pub fn width(&self) -> usize {
        self.pixels.size().width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.pixels.size().height
    }

    #[inline]
    pub fn bounds(&self) -> Size {
        self.pixels.size()
    }

    #[inline]
    pub fn rows(&self) -> impl DoubleEndedIterator<Item = &[u32]> {
        self.pixels.rows()
    }

    #[inline]
    pub fn get_row(&self, index: usize) -> &[u32] {
        &self.pixels[index]
    }

    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        self.pixels[y][x]
    }
}

pub struct Theme {
    land_texture: Option<ThemeSprite>,
    border_texture: Option<ThemeSprite>
}

impl Theme {
    pub fn land_texture(&self) -> Option<&ThemeSprite> {
        self.land_texture.as_ref()
    }

    pub fn border_texture(&self) -> Option<&ThemeSprite> {
        self.border_texture.as_ref()
    }
}

#[derive(Debug)]
pub enum ThemeLoadError {
    File(io::Error),
    Decoding(DecodingError),
    Format(String)
}

impl From<io::Error> for ThemeLoadError {
    fn from(e: io::Error) -> Self {
        ThemeLoadError::File(e)
    }
}

impl From<DecodingError> for ThemeLoadError {
    fn from(e: DecodingError) -> Self {
        ThemeLoadError::Decoding(e)
    }
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            land_texture: None,
            border_texture: None,
        }
    }

    pub fn load(path: &Path) -> Result<Theme, ThemeLoadError> {
        let mut theme = Self::new();

        for entry in read_dir(path)? {
            let file = entry?;
            if file.file_name() == "LandTex.png" {
                theme.land_texture = Some(load_sprite(&file.path())?)
            } else if file.file_name() == "Border.png" {
                theme.border_texture = Some(load_sprite(&file.path())?)
            }
        }

        Ok(theme)
    }
}

fn load_sprite(path: &Path) -> Result<ThemeSprite, ThemeLoadError> {
    let decoder = Decoder::new(
        BufReader::new(File::open(path)?));
    let (info, mut reader) = decoder.read_info()?;

    if info.color_type != ColorType::RGBA {
        return Err(ThemeLoadError::Format(
            format!("Unexpected format: {:?}", info.color_type)));
    }
    let size = Size::new(info.width as usize, info.height as usize);

    let mut pixels: Vec2D<u32> = Vec2D::new(size, 0);
    reader.next_frame(slice_u32_to_u8_mut(pixels.as_mut_slice()))?;

    Ok(ThemeSprite { pixels })
}

pub fn slice_u32_to_u8(slice_u32: &[u32]) -> &[u8] {
    unsafe {
        from_raw_parts::<u8>(
            slice_u32.as_ptr() as *const u8,
            slice_u32.len() * 4
        )
    }
}

pub fn slice_u32_to_u8_mut(slice_u32: &mut [u32]) -> &mut [u8] {
    unsafe {
        from_raw_parts_mut::<u8>(
            slice_u32.as_mut_ptr() as *mut u8,
            slice_u32.len() * 4
        )
    }
}

