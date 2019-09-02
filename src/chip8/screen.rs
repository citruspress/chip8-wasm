const WIDTH: usize = 64;
const HEIGHT: usize = 32;

/// The `Screen` type. Represents the chip8 screen. Each pixel is represented by
/// a bit in a bitfield.
pub struct Screen {
    pixels: [u8; WIDTH * HEIGHT / 8],
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: [0; WIDTH * HEIGHT / 8],
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [0; WIDTH * HEIGHT / 8];
    }

    pub fn draw_sprite(&mut self, x: usize, mut y: usize, data: &[u8]) {
        for line in data {
            let first_pixel_offset = x % 8;
            let index = self.index(x, y);
            let first_row = line >> first_pixel_offset as u8;
            self.pixels[index] = first_row ^ self.pixels[index];

            if first_pixel_offset > 0 {
                let second_pixel_offset = 8 - first_pixel_offset;
                let second_row = line << second_pixel_offset;
                self.pixels[index + 1] = second_row ^ self.pixels[index + 1];
            }
            y += 1;
        }
    }

    pub fn get_screen_data(&self) -> [u8; WIDTH * HEIGHT] {
        let mut pixels = [0; WIDTH * HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.get(x, y) {
                    pixels[y * WIDTH + x] = 255;
                }
            }
        }

        pixels
    }

    fn get(&self, x: usize, y: usize) -> bool {
        let offset = (8 - x % 8) - 1;
        let pixel_mask = 1 << offset;
        self.pixels[self.index(x, y)] & pixel_mask == pixel_mask
    }

    fn index(&self, x: usize, y: usize) -> usize {
        (y * WIDTH + x) / 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_is_cleared_by_default() {
        let screen = Screen::new();

        assert!(screen.pixels.iter().all(|p| p == &0));
    }

    #[test]
    fn can_get_single_pixel() {
        let screen = Screen::new();

        assert_eq!(false, screen.get(0, 0));
    }

    #[test]
    fn can_draw_sprite_data() {
        let mut screen = Screen::new();

        let sprite = vec![0xFF];
        screen.draw_sprite(0, 0, &sprite);

        for x in 0..8 {
            assert_eq!(true, screen.get(x, 0));
        }
        assert_eq!(false, screen.get(8, 0));
    }

    #[test]
    fn can_draw_multiline_sprite_data() {
        let mut screen = Screen::new();

        let sprite = vec![0xFF, 0xFF];
        screen.draw_sprite(0, 0, &sprite);

        for y in 0..2 {
            for x in 0..8 {
                assert_eq!(true, screen.get(x, y));
            }
        }
    }

    #[test]
    fn setting_a_pixel_toggles_it() {
        let mut screen = Screen::new();

        let sprite = vec![0xFF];
        let second_sprite = vec![0b11000011];
        screen.draw_sprite(0, 0, &sprite);
        screen.draw_sprite(0, 0, &second_sprite);

        assert_eq!(false, screen.get(0, 0));
        assert_eq!(false, screen.get(1, 0));
        assert_eq!(true, screen.get(2, 0));
        assert_eq!(true, screen.get(3, 0));
        assert_eq!(true, screen.get(4, 0));
        assert_eq!(true, screen.get(5, 0));
        assert_eq!(false, screen.get(6, 0));
        assert_eq!(false, screen.get(7, 0));
    }

    #[test]
    fn clear_clears_the_screen() {
        let mut screen = Screen::new();

        let sprite = vec![0xFF, 0xFF];
        screen.draw_sprite(0, 0, &sprite);
        screen.clear();

        assert!(screen.pixels.iter().all(|p| p == &0));
    }

    #[test]
    fn draw_with_half_sprite_offset() {
        let mut screen = Screen::new();

        let sprite = vec![0xFF];
        screen.draw_sprite(5, 0, &sprite);

        assert_eq!(false, screen.get(0, 0));
        for x in 5..13 {
            assert_eq!(true, screen.get(x, 0));
        }
        assert_eq!(false, screen.get(5 + 8, 0));
    }
}
