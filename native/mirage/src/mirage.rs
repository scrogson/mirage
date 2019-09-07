use image::{DynamicImage, FilterType, GenericImageView, ImageFormat};
use rustler::{Atom, Binary, Encoder, Env, NifStruct, OwnedBinary, ResourceArc, Term};
use std::error::Error;
use std::io::Write as _;

use crate::atoms::{error, gif, jpg, ok, png, unsupported_image_format};

#[derive(NifStruct)]
#[module = "Mirage"]
struct Mirage {
    byte_size: usize,
    extension: Atom,
    height: u32,
    width: u32,
    resource: ResourceArc<Image>,
}

struct Image {
    image: DynamicImage,
    format: ImageFormat,
}

/// from_bytes(path: String) -> Result<Mirage>
pub fn from_bytes<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, rustler::Error> {
    let bytes: Binary = args[0].decode()?;

    match image::load_from_memory(bytes.as_slice()) {
        Ok(image) => {
            if let Ok(format) = image::guess_format(&bytes.as_slice()) {
                let mirage = Mirage {
                    byte_size: bytes.len(),
                    extension: extension(format),
                    width: image.width(),
                    height: image.height(),
                    resource: ResourceArc::new(Image { image, format }),
                };

                return Ok((ok(), mirage).encode(env));
            }
            return Err(rustler::Error::Atom("unsupported_image_format"));
        }
        Err(err) => Ok((error(), err.description()).encode(env)),
    }
}

/// resize(resource: ResourceArc<Image>, width: u32, height: u32) -> Result<Vec<u8>>
pub fn resize<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, rustler::Error> {
    let resource: ResourceArc<Image> = args[0].decode()?;
    let width: u32 = args[1].decode()?;
    let height: u32 = args[2].decode()?;
    let resized = resource
        .image
        .resize_to_fill(width, height, FilterType::Triangle);
    let mut output = Vec::new();
    let mut binary = OwnedBinary::new(resized.raw_pixels().len()).unwrap();

    match resized.write_to(&mut output, resource.format) {
        Ok(_) => {
            binary
                .as_mut_slice()
                .write_all(&output)
                .map_err(|_| rustler::Error::Atom("io_error"))?;
            let extension = extension(resource.format);
            let bytes = binary.release(env);
            let byte_size = bytes.as_slice().len();

            let mirage = Mirage {
                byte_size,
                extension,
                height,
                width,
                resource,
            };

            Ok((ok(), bytes, mirage).encode(env))
        }
        Err(err) => return Ok((error(), err.description()).encode(env)),
    }
}

fn extension(format: ImageFormat) -> Atom {
    match format {
        ImageFormat::PNG => png(),
        ImageFormat::JPEG => jpg(),
        ImageFormat::GIF => gif(),
        _ => unsupported_image_format(),
    }
}

pub fn load<'a>(env: Env, _info: Term<'a>) -> bool {
    rustler::resource_struct_init!(Image, env);
    true
}
