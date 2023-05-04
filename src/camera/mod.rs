use std::sync::{Arc, Mutex};

use image::{ImageBuffer, Rgb};
use nokhwa::{
    pixel_format::{RgbFormat},
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};

pub struct CameraService {
    camera: Camera,
}

impl CameraService {
    // Initialize nokhwa by aasking permissions on Mac
    // Chatgpt coded this, all hail openai for teaching me mpsc arcmutex for solving
    // A FRKING CAMERA INITIALIZATION
    // ....
    // .....
    // -> AND IT DOESNT WORK ON M1 MAC apegjapoejgpaoejfaipejfaif
    // ...
    // ...
    pub async fn init() -> anyhow::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();
        let shared_tx = Arc::new(Mutex::new(tx)); // Share the sender across multiple threads
        let shared_tx_clone = shared_tx.clone(); // Clone the shared sender
        nokhwa::nokhwa_initialize(move |x| {
            println!("Nokhwa Initalized: {x}");
            shared_tx_clone.lock().unwrap().send(()).unwrap(); // Use the shared sender inside the closure
        });
        rx.recv().unwrap();
        Ok(())
    }

    pub fn new() -> Self {
        let index = CameraIndex::Index(0);
        let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

        Self {
            camera: Camera::new(index, format).unwrap(),
        }
    }

    pub fn capture(&mut self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let frame = self.camera.frame().unwrap();
        println!("Captured Single Frame of {}", frame.buffer().len());

        let decoded = frame.decode_image::<RgbFormat>().unwrap();
        println!("Decoded Frame of {}", decoded.len());

        decoded
    }
}
