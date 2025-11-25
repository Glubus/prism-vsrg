#[derive(Clone)]
pub struct PixelSystem {
    pub pixel_size: f32, // Basé sur la hauteur (2.0 / height)
    pub window_width: u32,
    pub window_height: u32,
    pub aspect_ratio: f32,
}

impl PixelSystem {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        let pixel_size = 2.0 / window_height as f32;
        let aspect_ratio = window_width as f32 / window_height as f32;
        Self {
            pixel_size,
            window_width,
            window_height,
            aspect_ratio,
        }
    }

    /// Convertit des pixels en taille normalisée Y (Hauteur)
    pub fn y_pixels_to_normalized(&self, pixels: f32) -> f32 {
        pixels * self.pixel_size
    }

    /// Convertit des pixels en taille normalisée X (Largeur)
    /// Applique la correction d'aspect ratio pour garder les carrés carrés.
    pub fn x_pixels_to_normalized(&self, pixels: f32) -> f32 {
        (pixels * self.pixel_size) / self.aspect_ratio
    }

    /// Helper générique (ancien comportement, souvent utilisé pour Y)
    pub fn pixels_to_normalized(&self, pixels: f32) -> f32 {
        self.y_pixels_to_normalized(pixels)
    }

    pub fn update_size(&mut self, width: u32, height: u32, forced_ratio: Option<f32>) {
        self.window_width = width;
        self.window_height = height;
        self.pixel_size = 2.0 / height as f32;

        // Si un ratio est forcé (via les settings), on l'utilise, sinon on calcule le ratio réel
        self.aspect_ratio = forced_ratio.unwrap_or(width as f32 / height as f32);
    }
}
