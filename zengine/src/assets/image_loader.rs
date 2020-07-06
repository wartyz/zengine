extern crate image;

use image::DynamicImage;
use self::image::GenericImageView;

pub struct ImageAsset {
    pub width: u32,
    pub height: u32,

    pub data: Vec<u8>,
}

pub fn load(image_name: &str) -> ImageAsset {
    // current_exe() Devuelve Result<PathBuf> la ruta de archivos del ejecutable.
    match std::env::current_exe() {
        Ok(mut absolute_path) => {
            // Se mueve un fichero o carpeta atras
            absolute_path.pop();

            // Añade path
            absolute_path.push("assets/images/");
            absolute_path.push(image_name);

            match image::open(absolute_path) {
                Ok(img) => {
                    let (width, height) = img.dimensions();

                    let img = match img {
                        DynamicImage::ImageRgba8(img) => img,
                        img => img.to_rgba(),
                    };

                    return ImageAsset {
                        width: width,
                        height: height,
                        data: img.into_raw(),
                    };
                }
                Err(e) => panic!("No se puede cargar la imagen {}: {}", image_name, e)
            }
        }
        Err(e) => panic!("Error en el path del ejecutable {}", e)
    }
}