use std::{collections::HashMap, rc::Rc};

use serde::{de::Visitor, Deserialize, Serialize};
use speedy2d::{
    error::{BacktraceError, ErrorMessage},
    image::{ImageFileFormat, ImageHandle, ImageSmoothingMode},
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

#[derive(Debug)]
pub struct ImgManager {
    imgs: HashMap<String, Rc<ImageHandle>>,
}
impl ImgManager {
    pub fn new() -> ImgManager {
        ImgManager {
            imgs: HashMap::new(),
        }
    }
    pub fn get_img(&mut self, path: &String, graphics: &mut Graphics2D) -> Rc<ImageHandle> {
        if let Some(val) = self.imgs.get(path) {
            return Rc::clone(&val);
        } else {
            println!("{}", path);
            let result = Rc::new(get_image_handle(graphics, path).unwrap());
            self.imgs.insert(path.clone(), Rc::clone(&result));
            return result;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Img {
    pub state: Option<Rc<ImageHandle>>,
    path: String,
}

impl Serialize for Img {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.path)
    }
}

impl<'de> Deserialize<'de> for Img {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ImgVisitor)
    }
}

impl Img {
    pub fn new(path: String) -> Img {
        Img { state: None, path }
    }
    pub fn init(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager) {
        self.state = Some(manager.get_img(&self.path, graphics));
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
        E: serde::de::Error,
    {
        Ok(Img::new(String::from(v)))
    }
}

#[derive(Debug)]
pub enum ImgError {
    ImgNotInitialized,
}
