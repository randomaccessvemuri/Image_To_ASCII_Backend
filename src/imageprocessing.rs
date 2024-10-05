use photon_rs::effects::adjust_contrast;
use photon_rs::monochrome::grayscale_human_corrected;
use photon_rs::native::{open_image, save_image};
use photon_rs::transform::resize;
use photon_rs::PhotonImage;


pub fn convert_image_to_greyscale_contrast_corrected(image_path: &str) -> PhotonImage {
    let mut img = open_image(image_path).unwrap();
    
    
    grayscale_human_corrected(&mut img);
    adjust_contrast(&mut img, 70.0);

    save_image(img.clone(), "output.jpg");

    return img;
}


pub fn generate_ascii_char_from_range(chars: String, b_value_in: u8) -> String {
    let length_of_chars = chars.chars().count();  // Use .chars().count() for actual character count
    let b_value = b_value_in;

    // Divide the 256 range into the number of characters in the string (256 because u8 ranges from 0 to 255)
    let range = 255 / length_of_chars as u8;

    // Calculate the index by dividing b_value by the calculated range
    let index = b_value / range;

    // Ensure the index doesn't exceed the length of the character set (account for potential rounding issues)
    let adjusted_index = if index as usize >= length_of_chars {
        length_of_chars - 1
    } else {
        index as usize
    };

    // Get the character from the string at the adjusted index
    let ascii_char = chars.chars().nth(adjusted_index).unwrap().to_string();

    return ascii_char;
}



pub fn convert_grayscale_contrast_corrected_to_ascii(img_in: &mut PhotonImage, size_x: u32, size_y: u32, variant: u8) -> String {
    
    let mut ascii = String::new();
    let mut counter = 0;
    
    let mut img = img_in.clone();
    img = resize(&img, size_x, size_y, photon_rs::transform::SamplingFilter::Gaussian);



    for i in img.get_raw_pixels().chunks(4){
        let b = i[2];
        let mut ascii_char = String::new();
        match variant {
            0 => {
                ascii_char = generate_ascii_char_from_range(" .,:;i1tfLCG08@".to_string(), b);
            },
            1 => {
                ascii_char = generate_ascii_char_from_range(" ░▒▓█".to_string(), b);

            },
            2 => {
                ascii_char = generate_ascii_char_from_range(" .`'-,~^:\"=+<*>!i|/\\L()1{}[]?%$#@".to_string(), b);
            },
            _ => {
                ascii_char = generate_ascii_char_from_range(" `'-_\".,^:;!+*~=<>i?1lI|\\/(){}[]#%&@".to_string(), b);
            }
        }
        
        

        print!("{}", ascii_char);
        ascii.push_str(&ascii_char);
        counter += 1;

        if counter % img.get_width() == 0 {
            ascii.push_str("\n");
            println!();
        }
      
    }

    
    

    return ascii;
}