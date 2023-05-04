
# Final project report: [FINAL-REPORT.md](FINAL-REPORT.md)


This repository contains code and documentation for Rust 101 project - Sebastian Petrik (Plasmoxy#7521), Igor Durica (duricaigor#0164), Andrii Rybak (Kiligram#6228).


# Project proposal

![out](https://user-images.githubusercontent.com/31391787/228638774-5de3e49c-0058-4f22-b1c6-9b8f4413c94f.jpg)

We would like to implement an GUI/server application using various technologies, focusing mainly on image manipulation including cropping, inverting, detecting faces, taking images from camera and other interesting features that'll come to our minds.

## We hope to explore/learn:
- Rust's modular system - separate different functionalities into standalone modules giving the ability to reuse them as libraries and allowing splitting the work between members
- Integration of different libraries/technologies together
  - integrating our functionalties into a web server application / into a GUI application
  - integration with a deep learning library for face detection in the photos
- Custom image manipulation implementation (for example, inverting pixel colors to produce a negative) - possibly with Rayon, or using an existing image manipulation crate
- Concurrency and parallelism in Rust for our features (using multiple cpu threads for image processing, asynchronous server endpoints implementation)

## We hope to implement these features:
- Build a GUI inferface for our functionalities - either with native Rust fraemwork or Webview-based framework - https://tauri.app/
- Build a REST API for our functionalities using a web server framework
- Selected image manipulation functionalities (for example, crop an image, invert vertically/horizontally, set aspect ratio, invert colors)
- Identifying face on photo and cropping the person (using deep learning model OR just using a Rust crate implementation)
- Taking photo with camera (for now without live preview, simply take photo)

## Possible dependencies:
- Tauri - web frontend / rust backend - framework for GUI
- Rocket or Axum https://github.com/tokio-rs/axum - backend framework
- Tokio - for concurrency with rocket.rs
- Rayon - for (possible) parallelisation of our image procesing functionality
- Tensorflow - loading and inference of a face recognition DL model, or possibly even Tract (learn to leverage vast ecosystem of universal ONNX DL model inference)
- Nokhwa - taking a photo with a camera

