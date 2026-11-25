//! Pure-math facial landmark geometry calculations in 2D image coordinates.
//!
//! Coordinates follow standard top-down image space (`x` increases right, `y` increases down).
//! For interactive visual verification of these formulas, see
//! [Looksmax Scale](https://looksmaxscale.com/) and the
//! [Canthal Tilt Guide](https://looksmaxscale.com/canthal-tilt).

/// Golden ratio constant `phi = (1 + sqrt(5)) / 2`.
pub const PHI: f64 = 1.618_033_988_749_895;

/// A 2D landmark point in image pixel or normalized coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Vertical facial thirds ratios and balance score.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FacialThirds {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
    /// `1.0` represents an exact `1/3 : 1/3 : 1/3` split; decreases linearly with total L1 deviation.
    pub balance_score: f64,
}

/// Computes the canthal tilt angle in degrees from the medial (inner) canthus
/// to the lateral (outer) canthus in top-down image coordinates (`y` down).
///
/// Positive values indicate an upward (positive) lateral tilt; negative values
/// indicate a downward tilt. Works for both left and right eyes by using `|dx|`.
pub fn canthal_tilt_deg(medial: Point2D, lateral: Point2D) -> Option<f64> {
    let dx = (lateral.x - medial.x).abs();
    if dx <= f64::EPSILON {
        return None;
    }
    let dy_up = medial.y - lateral.y;
    Some(dy_up.atan2(dx).to_degrees())
}

/// Computes upper, middle, and lower facial thirds from top-down `y` coordinates:
/// `trichion_y < glabella_y < subnasale_y < menton_y`.
pub fn facial_thirds(
    trichion_y: f64,
    glabella_y: f64,
    subnasale_y: f64,
    menton_y: f64,
) -> Option<FacialThirds> {
    if !(trichion_y < glabella_y && glabella_y < subnasale_y && subnasale_y < menton_y) {
        return None;
    }
    let total = menton_y - trichion_y;
    let upper = (glabella_y - trichion_y) / total;
    let middle = (subnasale_y - glabella_y) / total;
    let lower = (menton_y - subnasale_y) / total;
    let target = 1.0 / 3.0;
    let l1_dev = (upper - target).abs() + (middle - target).abs() + (lower - target).abs();
    let balance_score = (1.0 - (l1_dev / (4.0 / 3.0))).clamp(0.0, 1.0);
    Some(FacialThirds {
        upper,
        middle,
        lower,
        balance_score,
    })
}

/// Returns `(ratio, delta)` where `ratio = height / width` and `delta = ratio - PHI`.
pub fn golden_ratio_delta(face_height: f64, face_width: f64) -> Option<(f64, f64)> {
    if face_height <= 0.0 || face_width <= 0.0 {
        return None;
    }
    let ratio = face_height / face_width;
    Some((ratio, ratio - PHI))
}

/// Computes normalized bilateral mirror symmetry in `[0.0, 1.0]` for corresponding
/// `(left, right)` landmark pairs around a vertical `midline_x`, normalized by `face_width`.
pub fn bilateral_symmetry(
    pairs: &[(Point2D, Point2D)],
    midline_x: f64,
    face_width: f64,
) -> Option<f64> {
    if pairs.is_empty() || face_width <= 0.0 {
        return None;
    }
    let sum_err: f64 = pairs
        .iter()
        .map(|(l, r)| {
            let mirrored_lx = 2.0 * midline_x - l.x;
            let dx = mirrored_lx - r.x;
            let dy = l.y - r.y;
            (dx * dx + dy * dy).sqrt()
        })
        .sum();
    let mean_err = sum_err / (pairs.len() as f64);
    Some((1.0 - (mean_err / face_width)).clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_canthal_tilt_both_eyes() {
        let right = canthal_tilt_deg(Point2D::new(100.0, 200.0), Point2D::new(140.0, 196.0)).unwrap();
        let left = canthal_tilt_deg(Point2D::new(100.0, 200.0), Point2D::new(60.0, 196.0)).unwrap();
        assert!((right - left).abs() < 1e-9);
        assert!((right - 5.7105931375).abs() < 1e-6);
    }

    #[test]
    fn negative_canthal_tilt() {
        let tilt = canthal_tilt_deg(Point2D::new(100.0, 200.0), Point2D::new(140.0, 204.0)).unwrap();
        assert!(tilt < 0.0);
    }

    #[test]
    fn exact_equal_facial_thirds() {
        let t = facial_thirds(10.0, 110.0, 210.0, 310.0).unwrap();
        assert!((t.upper - 1.0 / 3.0).abs() < 1e-9);
        assert!((t.balance_score - 1.0).abs() < 1e-9);
    }

    #[test]
    fn golden_ratio_exact() {
        let (ratio, delta) = golden_ratio_delta(161.8033988749895, 100.0).unwrap();
        assert!((ratio - PHI).abs() < 1e-9);
        assert!(delta.abs() < 1e-9);
    }

    #[test]
    fn perfect_symmetry_returns_one() {
        let pairs = [
            (Point2D::new(80.0, 100.0), Point2D::new(120.0, 100.0)),
            (Point2D::new(70.0, 150.0), Point2D::new(130.0, 150.0)),
        ];
        let s = bilateral_symmetry(&pairs, 100.0, 120.0).unwrap();
        assert!((s - 1.0).abs() < 1e-9);
    }
}
