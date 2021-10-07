use speedy2d::{Graphics2D, error::{BacktraceError, ErrorMessage}, image::{ImageFileFormat, ImageHandle, ImageSmoothingMode}};

pub fn get_image_handle(graphics: &mut Graphics2D, path: &'static str) -> Result<ImageHandle, BacktraceError<ErrorMessage>> {
    graphics.create_image_from_file_path(Some(ImageFileFormat::PNG), ImageSmoothingMode::NearestNeighbor, path)
}