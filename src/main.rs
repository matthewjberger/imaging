use anyhow::Result;
use skia_safe::{
    AlphaType, Color, ColorSpace, ColorType, Data, EncodedImageFormat, ISize, ImageInfo, Point,
    Surface,
};
use std::{fs::File, io::Write};

fn main() -> Result<()> {
    let image = image::open("images/dice.png")?;
    let (width, height) = (image.width(), image.height());

    let mut surface = Surface::new_raster_n32_premul((width as _, height as _)).unwrap();
    surface.canvas().clear(Color::WHITE);

    let image_info = ImageInfo::new(
        ISize::new(width as _, height as _),
        ColorType::RGBA8888,
        AlphaType::Opaque,
        ColorSpace::new_srgb_linear().with_linear_gamma(),
    );

    let pixels = image.as_bytes();
    let skia_image = skia_safe::image::Image::from_raster_data(
        &image_info,
        Data::new_copy(&pixels),
        width as usize * 4,
    )
    .unwrap();

    surface
        .canvas()
        .draw_image(&skia_image, Point::new(0.0, 0.0), None);

    let data = surface
        .image_snapshot()
        .encode_to_data(EncodedImageFormat::PNG)
        .unwrap();

    let mut file = File::create("test.png").unwrap();
    file.write_all(data.as_bytes()).unwrap();

    Ok(())
}
