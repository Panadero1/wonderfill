use serde::{Deserialize, Serialize, de::Visitor};
use speedy2d::{
    error::{BacktraceError, ErrorMessage},
    image::{ImageDataType, ImageFileFormat, ImageHandle, ImageSmoothingMode},
    Graphics2D,
};

pub fn get_image_handle(
    graphics: &mut Graphics2D,
    path: &str,
) -> Result<ImageHandle, BacktraceError<ErrorMessage>> {
    graphics.create_image_from_file_path(
        Some(ImageFileFormat::PNG),
        ImageSmoothingMode::NearestNeighbor,
        path,
    )
}

pub enum ImgState {
    Pending,
    Image(ImageHandle),
}
impl ImgState {
    fn init(&mut self, img: ImageHandle) {
        *self = ImgState::Image(img);
    }
}

#[derive(Debug)]
pub struct Img {
    pub state: Option<ImageHandle>,
    path: String,
}

impl Serialize for Img {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.path)
    }
}

impl<'de> Deserialize<'de> for Img {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_str(ImgVisitor)
    }
}

impl Img {
    pub fn new(path: String) -> Img {
        Img {
            state: None,
            path,
        }
    }
    pub fn init(&mut self, graphics: &mut Graphics2D) {
        self.state = Some(get_image_handle(graphics, &self.path).unwrap());
    }
}

struct ImgVisitor;

impl<'de> Visitor<'de> for ImgVisitor {
    type Value = Img;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expecting file path")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
            E: serde::de::Error, {
        Ok(Img::new(String::from(v)))
    }

    
}

#[derive(Debug)]
pub enum ImgError {
    ImgNotInitialized,
}