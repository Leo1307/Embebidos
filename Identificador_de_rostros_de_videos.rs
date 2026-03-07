use opencv::core::{Rect, Size, Vector};
use opencv::prelude::*;
use opencv::{core, highgui, imgproc, videoio, Result};

opencv::opencv_branch_5! {
    use opencv::xobjdetect::{CascadeClassifier, CASCADE_SCALE_IMAGE};
}

opencv::not_opencv_branch_5! {
    use opencv::objdetect::{CascadeClassifier, CASCADE_SCALE_IMAGE};
}

const WINDOW: &str = "Deteccion de rostros";
const WINDOW_GRAY: &str = "Escala de grises";
const WINDOW_REDUCED: &str = "Imagen reducida";

fn main() -> Result<()> {
    highgui::named_window_def(WINDOW)?;
    highgui::named_window_def(WINDOW_GRAY)?;
    highgui::named_window_def(WINDOW_REDUCED)?;

    let xml = core::find_file_def("haarcascades/haarcascade_frontalface_alt.xml")
        .expect("No se pudo encontrar el archivo cascade. Asegúrate de tener los archivos de OpenCV");

    let mut cam = videoio::VideoCapture::from_file("Prueba.mp4", videoio::CAP_ANY)?;

    if !cam.is_opened()? {
        panic!("No se pudo abrir el video");
    }

    let mut face = CascadeClassifier::new(&xml)?;
    let mut gray = Mat::default();
    let mut reduced = Mat::default();

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;

        if frame.size()?.width == 0 {
            break;
        }

        // Convertir a escala de grises
        imgproc::cvt_color_def(&frame, &mut gray, imgproc::COLOR_BGR2GRAY)?;
        highgui::imshow(WINDOW_GRAY, &gray)?;

        // Reducir imagen para detección más rápida
        imgproc::resize(&gray, &mut reduced, Size::new(0, 0), 0.25, 0.25, imgproc::INTER_LINEAR)?;
        highgui::imshow(WINDOW_REDUCED, &reduced)?;

        // Detectar rostros
        let mut faces = Vector::new();
        face.detect_multi_scale(
            &reduced,
            &mut faces,
            1.1,  // scale factor
            2,    // min neighbors
            CASCADE_SCALE_IMAGE,
            Size::new(30, 30),  // min size
            Size::new(0, 0),     // max size
        )?;

        println!("Rostros detectados: {}", faces.len());

        // Dibujar rectángulos en el frame original
        for face_rect in &faces {
            let scaled_face = Rect::new(
                face_rect.x * 4,
                face_rect.y * 4,
                face_rect.width * 4,
                face_rect.height * 4
            );

            imgproc::rectangle(
                &mut frame,
                scaled_face,
                core::Scalar::new(0.0, 255.0, 0.0, 0.0), // Verde
                2,
                imgproc::LINE_8,
                0,
            )?;
        }

        // Mostrar contador en pantalla
        let text = format!("Rostros: {}", faces.len());
        imgproc::put_text(
            &mut frame,
            &text,
            core::Point::new(20, 40),
            imgproc::FONT_HERSHEY_SIMPLEX,
            1.0,
            core::Scalar::new(0.0, 255.0, 0.0, 0.0), // Verde
            2,
            imgproc::LINE_8,
            false,
        )?;

        highgui::imshow(WINDOW, &frame)?;

        // Salir con ESC (código 27) o cualquier tecla
        let key = highgui::wait_key(30)?;
        if key == 27 { // ESC key
            break;
        }
    }

    // Limpiar recursos
    highgui::destroy_all_windows()?;
    Ok(())
}
