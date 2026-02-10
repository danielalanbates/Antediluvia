//! Procedural terrain generation using Simplex Noise.
//! 
//! Seed: "Genesis 6:14"
//! Scale: 1:70 (30,000 km² playable area)
//! Shape: Pangea Ultima (C-shaped supercontinent)

use noise::{NoiseFn, Simplex};

/// The Pangea terrain generator.
pub struct PangeaGenerator {
    noise: Simplex,
    scale: f64,
}

impl PangeaGenerator {
    /// Create a new Pangea generator with the deterministic seed.
    pub fn new() -> Self {
        // Hash "Genesis 6:14" to a u32 seed
        let seed_str = "Genesis 6:14";
        let seed = Self::hash_seed(seed_str);
        
        Self {
            noise: Simplex::new(seed),
            scale: 0.005, // Frequency for continent-scale features
        }
    }

    /// Hash a string seed to a u32.
    fn hash_seed(s: &str) -> u32 {
        let mut hash: u32 = 5381;
        for byte in s.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
        }
        hash
    }

    /// Get the height at a given (x, z) coordinate.
    /// 
    /// Returns height in meters. Negative values are ocean.
    pub fn get_height(&self, x: f64, z: f64) -> f32 {
        // Distance from the center (0,0) = Eden
        let dist = (x * x + z * z).sqrt();

        // Base continent noise
        let base_noise = self.noise.get([x * self.scale, z * self.scale]);

        // The "C-Shape" Mask: Radial gradient forcing continent shape
        let height = if dist < 500.0 {
            // The Garden Plateau (Inaccessible)
            2000.0
        } else if dist < 2000.0 {
            // Havilah (Starting zone) - Lush, safe
            20.0 + (base_noise as f32 * 10.0)
        } else if dist < 5000.0 {
            // The Great Moat (Ocean barrier)
            -50.0
        } else if dist < 30000.0 {
            // The Outer Continent (Pangea)
            // Apply erosion-like variation
            let angle = z.atan2(x);
            let radial_factor = ((dist - 5000.0) / 25000.0).clamp(0.0, 1.0);
            
            // C-shape mask: Reduce height on the "open" side
            let c_shape_mask = (angle.sin() * 0.5 + 0.5) * radial_factor;
            
            (base_noise as f32 * 200.0) * c_shape_mask as f32
        } else {
            // Beyond the world edge
            -100.0
        };

        height
    }

    /// Generate a heightmap for a region.
    /// 
    /// Returns a Vec of heights for a grid of (width x height) points.
    pub fn generate_heightmap(&self, center_x: f64, center_z: f64, width: usize, height: usize, step: f64) -> Vec<f32> {
        let mut heightmap = Vec::with_capacity(width * height);
        
        let start_x = center_x - (width as f64 * step) / 2.0;
        let start_z = center_z - (height as f64 * step) / 2.0;

        for z_idx in 0..height {
            for x_idx in 0..width {
                let x = start_x + (x_idx as f64 * step);
                let z = start_z + (z_idx as f64 * step);
                let h = self.get_height(x, z);
                heightmap.push(h);
            }
        }

        heightmap
    }
}

impl Default for PangeaGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eden_is_high() {
        let gen = PangeaGenerator::new();
        let height = gen.get_height(0.0, 0.0);
        assert!(height > 1000.0, "Eden should be a high plateau");
    }

    #[test]
    fn test_havilah_is_safe() {
        let gen = PangeaGenerator::new();
        let height = gen.get_height(1000.0, 0.0);
        assert!(height > 0.0 && height < 100.0, "Havilah should be low-altitude land");
    }

    #[test]
    fn test_ocean_is_negative() {
        let gen = PangeaGenerator::new();
        let height = gen.get_height(3000.0, 0.0);
        assert!(height < 0.0, "The moat should be ocean");
    }
}
