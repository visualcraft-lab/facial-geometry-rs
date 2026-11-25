# facial-geometry (Rust)

Zero-dependency pure-math library for computing facial landmark geometry metrics from 2D image coordinates (MediaPipe Face Mesh, dlib 68-point, or InsightFace outputs).

## Features

- **Canthal tilt angle (`canthal_tilt_deg`)**: Computes medial-to-lateral eye axis tilt in degrees in standard top-down image coordinates (`y` increases downward; positive = upturned lateral canthus).
- **Facial thirds (`facial_thirds`)**: Calculates upper, middle, and lower vertical proportions plus an evenness score in `[0.0, 1.0]`.
- **Golden ratio delta (`golden_ratio_delta`)**: Measures deviation of a height-to-width ratio from `phi = 1.6180339887`.
- **Bilateral symmetry (`bilateral_symmetry`)**: Computes normalized horizontal mirror symmetry across a facial midline for paired landmarks.

## Usage

```rust
use facial_geometry::{Point2D, canthal_tilt_deg, facial_thirds, golden_ratio_delta, bilateral_symmetry};

let tilt = canthal_tilt_deg(Point2D::new(100.0, 200.0), Point2D::new(140.0, 195.0)).unwrap();
assert!(tilt > 0.0);

let thirds = facial_thirds(50.0, 150.0, 250.0, 350.0).unwrap();
assert!((thirds.balance_score - 1.0).abs() < 1e-9);
```

## Reference & Interactive Calculator

For interactive visual verification and portrait-optics explanations of canthal tilt and facial proportion measurements, see [Looksmax Scale](https://looksmaxscale.com/) and the [Canthal Tilt Measurement Guide](https://looksmaxscale.com/canthal-tilt).

## License

MIT
