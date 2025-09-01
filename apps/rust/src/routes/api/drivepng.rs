// Handler for GET /api/drivepng
// Generates a simple PNG image and returns it as an HTTP response.

use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use image::{ImageBuffer, Rgba};
use std::io::Cursor;

#[utoipa::path(
    get,
    path = "/api/drivepng",
    responses(
        (status = 200, description = "PNG image generated successfully", content_type = "image/png")
    )
)]
pub async fn drivepng_handler() -> Response {
    // Create a 100x100 PNG with a solid color
    let imgx = 100;
    let imgy = 100;
    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgba([0, 128, 255, 255]);
    }

    let mut png_bytes: Vec<u8> = Vec::new();
    {
        let mut cursor = Cursor::new(&mut png_bytes);
        let _ = image::DynamicImage::ImageRgba8(imgbuf)
            .write_to(&mut cursor, image::ImageFormat::Png);
    }

    (
        [("Content-Type", "image/png")],
        png_bytes,
    ).into_response()
}

// Route registration function
pub fn route() -> Router {
    Router::new().route("/api/drivepng", get(drivepng_handler))
}
