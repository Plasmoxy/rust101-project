This report serves as a summary for the final Rust 101 project written by - Sebastian Petrik (Plasmoxy#7521), Igor Durica (duricaigor#0164), Andrii Rybak (Kiligram#6228).

# Introduction

![out](https://user-images.githubusercontent.com/31391787/228638774-5de3e49c-0058-4f22-b1c6-9b8f4413c94f.jpg)

For our final project, we decided to build a simple API offering image manipulation functionalities together with face recognition using a pretrained deep learning model.
Each feature has a dedicated endpoint which requires the users to upload a single image in the body of their HTTP POST request and subsequently returns either an altered version, or
JSON data in the response.
We also have the foundation for facial recognition of footage taken directly from a user's web camera, however we had difficulties with testing and finalizing the web camera functionality.

# Requirements

Following are the requirements we expected to fulfill during the implementation.

## Functional Requirements

- The API should allow users to upload image and receive either altered versions with faces highlighted
- The API should allow users to upload image and receive distorted version
- The API should allow users to upload image and receive version with inverted colors
- The API should allow users to upload image and receive version with black edges cut off
- The API should allow users to upload image, specify the number of degrees via route parameters and receive version rotated by that many degrees
- The API should allow users to upload image, specify the x and y offsets and the width and height and via query parameters and receive cropped version
- The API should provide web camera funcionality on the server-side, additionaly with face recognition

## Non-Functional Requirements

- The API should be able to process images and footage quickly and efficiently to provide real-time results
- The API should be able to handle multiple requests simultaneously without any delays or timeouts
- The API shall be easy to use and intuitive for all users
- The API should have proper error handling 
- The API should return a HTTP 400 error code if an image cannot be parsed from the request body
- The API should return a HTTP 400 error code if required parameters are missing 

# Design diagram

# Design Choices

## Module system
To logically separate different sections of our projects, we decided to use a module structure where each module is contained in its own folder:
- camera - camera functionality providing async-initializable CameraService
- core - providing core types and functionality used in other modules
- images - image manipulation functionality
- neural - face detection AI model inference functionality
- web -  web server and endpoints implementation

## Neural inference
To implement face recognition, we wanted to explore using common pretrained AI model formats for inference in Rust. We attempted to use TensorFlow's protocol buffer formats (SavedModel), however that required
installation of TensorFlow on the host machine. Instead, we then explored the ONNX model format using tract-onnx Rust library. Various free ONNX models are available, we chose the UltraFace 640x640 model.

## Web and Axum

We chose Axum because of its simplicity as well as the abundance and quality of documentation and educational content which helps further smooth out the new developer experience.
Additionally, it is also built using and fully compatible with the Tokio framework which we were already using in our application by the time we decided to expose our individual features via an API.

## Image processing and multithreading 
We chose `image::RgbImage` struct as our primary image format used in project. We explored it's internals and by gaining access to the individual pixels of the buffer, we were able
to provide the image manipulation functionality.

We attempted to employ rayon parallel iterator for image manipulation, however this has proven more difficult and we instead used threads directly.
For faster execution, we are spawning 4 threads for cutting off the picture's black edges.

## Tokio runtime
We used Tokio async runtime for our app and used it for initialization and as a server runtime. In the neural module, the required models are automatically downloaded if not present, thus
the initialization of module is implemented within an async function and needs to be awaited in the initialization before the server starts. We also attempted to get some initialization functionality
into the camera module.

## Camera
We attempted to implement camera functionality which would take pictures and provide them in the RgbImage format. We used nokhwa library and implemented it, including async initialization of the nokhwa library.
The library provided only callback-based initialization, so we attempted to use MPSC channel to convert this into an async function. The code has compiled, but there were difficulties in testing
it due to poor interoperation of AVFoundation camera backend on M1 Macs in Rust. Because of the difficulties, the camera functionality wasn't finished and other parts took priority.

# Dependencies

- anyhow - default convenient error handling
- axum - backend framework
- tokio - async runtime
- image - used for parsing JPEG/PNG into a raw pixel buffer
- imageproc - used for drawing bounding boxes on images from the neural network 
- ndarray - used for neural network input representations
- nokhwa - camera functionality
- rand - random number generation for distorting images
- reqwest - used for downloading the neural model
- serde - serialization traits derivation
- smallvec - used in neural network
- tract-onnx - ONNX neural inference framework for Rust

# Evaluation

## General implementation
We were able to fulfill most of our project requirements. While the funcionality works, there is a lot of space for improvements including the camera module, better parallelisation and multithreading.

## AI model interoperation
While we were able to succesfuly implement face recognition using an pretrained AI model in Rust. The resulting endpoint however takes some time for bigger photos. We suspect the problem may be with input
representation preparation before feeding it into the model, thus more testing is needed.

## Camera
The main problem with camera was bad library support on M1 architecure - specifically the interoperation with Apple AVFoundation backend on M1 was broken. No library for Rust is currently available which has the issue resolved and the most popular library has this as an open unresolved issue. This made us wondering how ready is Rust ecosystem when it comes to some high level functionalities like this - what if our project really needs this but it just isn't implemented?
Fixing this issue would require much more expertise and it would be probably easier to call a Python camera code from Rust.

## Concurrency
We used concurrency in our project, however during camera initialization implementation, it was interesting trying to convert callback-based funcitonality into async functionality. The MPSC channel and other variants (we had trouble with oneshot though because of moved values) have shown to be an interesting and useful mechanism for this problem.

## Axum

Using Axum was a pleasant experience for the most part.
It's a fairly straightforward web framework with not too steep of a learning curve
However it did feel awkward at times, especially in regards to how request bodies are parsed.
When using Multipart Requests to upload images, passing other information inside of the request body as text resulted in fairly convoluted code.
To my knowledge, there is no way to access specific fields of the request by their key and Axum does not provide support for typed requests out of the box.
I was therefore left with no choice but to iterate over the individual fields in whichever order they came in.
I am fully aware that this might 100% be due to user error or a general lack of knowledge of the framework.
Regardless, I was able to find convenient workarounds through the use of query and route parameters which both work very well. 

