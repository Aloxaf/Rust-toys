use image::{GrayImage, Luma};

#[inline]
fn luma2ascii(c: Luma<u8>) -> char {
    let ascii_char = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
    let index = (((ascii_char.len() - 1) * c.data[0] as usize) as f32 / 255.0) as usize;
    ascii_char.chars().nth(index).unwrap()
}

pub fn image2ascii(img: &GrayImage) -> Vec<String> {
    let (width, _) = img.dimensions();
    let chars = img.pixels().map(|&c| luma2ascii(c)).collect::<Vec<_>>();
    chars
        .chunks(width as usize)
        .map(|c| c.iter().collect())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{image2ascii, luma2ascii};
    use image::{GrayImage, Luma};

    #[test]
    fn test_luma2ascii() {
        assert_eq!(luma2ascii(Luma { data: [255] }), ' ');
        assert_eq!(luma2ascii(Luma { data: [0] }), '$');
    }

    #[test]
    fn test_image2ascii() {
        let pixels = vec![0, 255, 0, 255];
        let asciis = vec!["$ ", "$ "];
        let image = GrayImage::from_raw(2, 2, pixels).unwrap();
        assert_eq!(image2ascii(&image), asciis);
    }
}
