mod imageprocessing;

fn main() {
    let mut img = imageprocessing::convert_image_to_greyscale_contrast_corrected("/Users/tanmayvemuri/Downloads/Slayer_wordmark.svg.png");
    imageprocessing::convert_grayscale_contrast_corrected_to_ascii(&mut img, 32, 16, 4);
}
