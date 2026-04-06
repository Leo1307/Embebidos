use opencv::core::{Rect, Size, Vector};
use opencv::prelude::*;
use opencv::{core, highgui, imgproc, videoio, Result};
use opencv::objdetect::{CascadeClassifier, CASCADE_SCALE_IMAGE};

const WINDOW: &str = "Deteccion de rostros";

fn main() -> Result<()> {
    highgui::named_window_def(WINDOW)?;

    let xml = core::find_file_def("haarcascades/haarcascade_frontalface_alt.xml")
        .expect("No se pudo encontrar el archivo cascade");

    let mut video = videoio::VideoCapture::from_file("Prueba.mp4", videoio::CAP_FFMPEG)?;

    if !video.is_opened()? {
        panic!("No se pudo abrir el archivo de video");
    }

    let fps = video.get(videoio::CAP_PROP_FPS)?;
    let total_frames = video.get(videoio::CAP_PROP_FRAME_COUNT)?;
    println!("Video abierto: {} fps, {} frames totales", fps, total_frames);

    let mut face = CascadeClassifier::new(&xml)?;
    let mut gray = Mat::default();
    let mut reduced = Mat::default();

    loop {
        let mut frame = Mat::default();
        video.read(&mut frame)?;

        if frame.size()?.width == 0 {
            println!("Fin del video");
            break;
        }

        imgproc::cvt_color_def(&frame, &mut gray, imgproc::COLOR_BGR2GRAY)?;
        imgproc::resize(&gray, &mut reduced, Size::new(0, 0), 0.25, 0.25, imgproc::INTER_LINEAR)?;

        let mut faces = Vector::new();
        face.detect_multi_scale(
            &reduced,
            &mut faces,
            1.1,
            2,
            CASCADE_SCALE_IMAGE,
            Size::new(30, 30),
            Size::new(0, 0),
        )?;

        for face_rect in &faces {
            let scaled_face = Rect::new(
                face_rect.x * 4,
                face_rect.y * 4,
                face_rect.width * 4,
                face_rect.height * 4,
            );

            imgproc::rectangle(
                &mut frame,
                scaled_face,
                core::Scalar::new(0.0, 255.0, 0.0, 0.0),
                2,
                imgproc::LINE_8,
                0,
            )?;
        }

        let text = format!("Rostros: {} | Frame: {:.0}", faces.len(), video.get(videoio::CAP_PROP_POS_FRAMES)?);
        imgproc::put_text(
            &mut frame,
            &text,
            core::Point::new(20, 40),
            imgproc::FONT_HERSHEY_SIMPLEX,
            0.8,
            core::Scalar::new(0.0, 255.0, 0.0, 0.0),
            2,
            imgproc::LINE_8,
            false,
        )?;

        highgui::imshow(WINDOW, &frame)?;

        let key = highgui::wait_key(30)?;
        if key == 27 {
            break;
        }
    }

    highgui::destroy_all_windows()?;
    Ok(())
}
