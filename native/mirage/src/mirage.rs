use image::{DynamicImage, FilterType, GenericImageView, ImageFormat};
use rustler::resource::ResourceArc;
use rustler::{
    types::{atom::Atom, Binary, OwnedBinary},
    Encoder, Env, Term,
};
use rustler_codegen::NifStruct as Struct;
use std::error::Error;
use std::io::Write as _;

use crate::atoms::{error, gif, jpg, ok, png, unsupported_image_format};

#[derive(Struct)]
#[module = "Mirage"]
struct Mirage<'a> {
    bytes: Binary<'a>,
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
            let mut output = Vec::new();
            let mut binary = OwnedBinary::new(image.raw_pixels().len()).unwrap();
            let format = match image::guess_format(&bytes.as_slice()) {
                Ok(format) => format,
                Err(_) => return Ok((error(), unsupported_image_format()).encode(env)),
            };

            match image.write_to(&mut output, format) {
                Ok(_) => {
                    match binary.as_mut_slice().write_all(&output) {
                        Ok(()) => (),
                        Err(err) => println!("{:?}", err.description()),
                    };
                    let extension = extension(format);
                    let bytes = binary.release(env);
                    let byte_size = bytes.len();
                    let width = image.width();
                    let height = image.height();
                    let resource = ResourceArc::new(Image { image, format });

                    let mirage = Mirage {
                        bytes,
                        byte_size,
                        extension,
                        height,
                        width,
                        resource,
                    };

                    Ok((ok(), mirage).encode(env))
                }
                Err(err) => return Ok((error(), err.description()).encode(env)),
            }
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
                bytes,
                byte_size,
                extension,
                height,
                width,
                resource,
            };

            Ok((ok(), mirage).encode(env))
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

pub fn load<'a>(env: Env<'a>) -> bool {
    rustler::resource_struct_init!(Image, env);
    true
}
