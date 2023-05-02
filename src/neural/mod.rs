use std::sync::{Arc, Mutex};
use std::time::Instant;

use image::RgbImage;

use crate::core::Bbox;

use self::nn::{InferModel, UltrafaceModel, UltrafaceVariant};

mod nn;

#[derive(Clone)]
pub struct NeuralInferrer {
    model: Arc<Mutex<UltrafaceModel>>,
}

impl NeuralInferrer {
    pub async fn new() -> anyhow::Result<Self> {
        let model = UltrafaceModel::new(UltrafaceVariant::W640H480, 0.5, 0.5).await.expect("Initialize model");
        let mutex = Mutex::new(model);

        Ok(Self { model: Arc::new(mutex) })
    }

    // Run Ultraface onnx neural model inference on a rgb image, return vec of bounding boxes and confidences of
    // detected faces, showing only faces with >95% confidence.
    pub fn infer_face(&self, image: &RgbImage) -> Vec<(Bbox, f32)> {
        let start = Instant::now();
        let model = self.model.lock().unwrap();
        let bboxes_and_confidences = model.run(image.clone()).unwrap();

        // accepty only > 95% confidence
        let filtered: Vec<(Bbox, f32)> = bboxes_and_confidences.into_iter().filter(|(_, confidence)| *confidence > 0.95).collect();

        println!(
            "Inferred faces in {:?}, found {:?} faces with >95% confidence.",
            start.elapsed(),
            filtered.len(),
        );
        return filtered;
    }
}
