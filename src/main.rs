use ::{clap::Parser, std::path::Path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long)]
    factor: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args {
        input,
        output,
        factor,
    } = Args::parse();

    let input = Path::new(&input);
    let output = Path::new(&output);

    upscale_image(input, output, factor).map_err(Into::into)
}

fn upscale_image(input: &Path, output: &Path, factor: usize) -> Result<(), image::ImageError> {
    let raster = image::open(input)?;

    let width = raster.width();
    let height = raster.height();

    let rgba = image::RgbaImage::from(raster);
    let rgba = xbrz::scale_rgba(&rgba, width as usize, height as usize, factor);

    let width = width * factor as u32;
    let height = height * factor as u32;

    image::save_buffer(
        output,
        &rgba,
        width,
        height,
        image::ExtendedColorType::Rgba8,
    )
}
