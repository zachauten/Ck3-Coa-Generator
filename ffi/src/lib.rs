use coa_converter_lib;
use image::io::Reader as ImageReader;

#[no_mangle]
pub extern "C" fn add(a: isize, b: isize) -> isize {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn add() {
        let result = super::add(1, 2);
        assert_eq!(result, 3);
    }
}
