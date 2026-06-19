//! Common constants beyond [`std::f64::consts`].
//!
//! Categories:
//! - **Geometric ratios**: [`PHI`], [`INV_PHI`], [`SILVER_RATIO`], [`PLASTIC_RATIO`], [`SQRT_3`], [`SQRT_5`]
//! - **Analysis & number theory**: [`EULER_MASCHERONI`], [`CATALAN`], [`APERY`], [`KHINCHIN`], [`GLAISHER_KINKELIN`], [`MEISSEL_MERTENS`], [`GELFOND`]
//! - **Feigenbaum constants**: [`FEIGENBAUM_DELTA`], [`FEIGENBAUM_ALPHA`]
//! - **Conversion factors**: [`DEG_TO_RAD`], [`RAD_TO_DEG`]
//! - **Luma coefficients (Rec. 709)**: [`LUMA_R`], [`LUMA_G`], [`LUMA_B`]

// --- Geometric and Algebraic Ratios ---

/// Golden Ratio (Phi) - (1 + sqrt(5)) / 2
pub const PHI: f64 = 1.618033988749895;

/// Inverse of the Golden Ratio (1 / Phi) - Phi - 1
pub const INV_PHI: f64 = 0.618033988749895;

/// Silver Ratio (Delta_S) - 1 + sqrt(2)
pub const SILVER_RATIO: f64 = 2.414213562373095;

/// Plastic Ratio (Rho) - The unique real solution to x^3 = x + 1
pub const PLASTIC_RATIO: f64 = 1.324717957244746;

/// Square root of 3
pub const SQRT_3: f64 = 1.732050807568877;

/// Square root of 5
pub const SQRT_5: f64 = 2.23606797749979;

// --- Analysis and Number Theory Constants ---

/// Euler-Mascheroni constant (Gamma)
///
/// The limiting difference between the harmonic series and the natural logarithm.
pub const EULER_MASCHERONI: f64 = 0.577215664901533;

/// Catalan's constant (G)
///
/// Often appears in estimates of combinatorial structures and definite integrals.
pub const CATALAN: f64 = 0.915965594177219;

/// Apéry's constant - zeta(3)
pub const APERY: f64 = 1.202056903159594;

/// Khinchin's constant (K)
///
/// The geometric mean of the elements of the continued fraction expansion of almost all real numbers.
pub const KHINCHIN: f64 = 2.685452001065306;

/// Glaisher-Kinkelin constant (A)
pub const GLAISHER_KINKELIN: f64 = 1.282427129100623;

/// Meissel–Mertens constant (M)
///
/// Related to the sum of the reciprocals of prime numbers.
pub const MEISSEL_MERTENS: f64 = 0.261497212847643;

/// Gelfond's constant (e^pi)
pub const GELFOND: f64 = 23.1406926327793;

/// Feigenbaum constant (delta) - bifurcation velocity
pub const FEIGENBAUM_DELTA: f64 = 4.669201609102991;

/// Feigenbaum constant (alpha) - width of the tines
pub const FEIGENBAUM_ALPHA: f64 = 2.502907875095893;

// --- Graphics and Technical Utilities ---

/// Superellipse / Lamé curve constant (approximation for "squircle")
pub const SQUIRCLE_POW: f64 = 4.0;

/// Multiplier to convert Degrees to Radians (PI / 180)
pub const DEG_TO_RAD: f64 = 0.0174532925199433;

/// Multiplier to convert Radians to Degrees (180 / PI)
pub const RAD_TO_DEG: f64 = 57.2957795130823;

/// Luma coefficients for Grayscale conversion (Rec. 709 standard) - Red
pub const LUMA_R: f64 = 0.2126;
/// Luma coefficients for Grayscale conversion (Rec. 709 standard) - Green
pub const LUMA_G: f64 = 0.7152;
/// Luma coefficients for Grayscale conversion (Rec. 709 standard) - Blue
pub const LUMA_B: f64 = 0.0722;
