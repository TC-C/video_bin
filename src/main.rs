use image::{open, save_buffer, ColorType, ImageResult};
use std::{
    fs::File,
    io::{stdin, stdout, Result, Write},
    thread::spawn,
};

//Only fits NON Prime images. This is a statistical anomaly for larger files and has not been considered yet
fn main() {
    let response = get_input_string("Would you like to encode a file or decode an image? >>> ")
        .unwrap()
        .to_lowercase();
    let response = response.as_str();
    match response {
        "encode" => {
            let bin_file_name =
                get_input_string("Enter the name of the file to encode >>> ").unwrap();
            let bin = std::fs::read(bin_file_name).unwrap();
            let len = bin.len();
            let img_dimension_thread = spawn(move || get_lcm(len / 3));
            let img_file_name =
                get_input_string("Enter the file name of the image to save to >>> ").unwrap();
            let img_dimension = img_dimension_thread.join().unwrap();
            encode_buf(&bin, img_file_name, img_dimension).unwrap();
        }
        "decode" => {
            let img_file_name =
                get_input_string("Enter the name of the file to decode >>> ").unwrap();
            let bin_file_name =
                get_input_string("Enter the name for the output file >>> ").unwrap();
            decode_buffer(&img_file_name, &bin_file_name).unwrap();
        }
        _ => {}
    }
}

fn encode_buf(bin: &[u8], img_file_name: String, img_dimension: (u32, u32)) -> ImageResult<()> {
    save_buffer(
        img_file_name,
        bin,
        img_dimension.0,
        img_dimension.1,
        ColorType::Rgb8,
    )
}

fn decode_buffer(img_file_name: &str, output_file_name: &str) -> Result<()> {
    let img = open(img_file_name).unwrap().to_rgb8();
    let bytes = img.as_raw();
    File::create(output_file_name)?.write_all(bytes)
}

fn get_lcm(n: usize) -> (u32, u32) {
    let factors: Vec<usize> = (1..=n).filter(|k| n % k == 0).collect();
    let fac_1 = n / factors
        .get(factors.len() / 2 - (factors.len() & 1 ^ 1))
        .unwrap();
    let fac_2 = n / factors.get(factors.len() / 2).unwrap();
    (fac_1 as u32, fac_2 as u32)
}

fn get_input_string(message: &str) -> Result<String> {
    print!("{}", message);
    stdout().flush()?;
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)?;
    Ok(input_string.trim_end_matches(&['\r', '\n'][..]).to_owned())
}
