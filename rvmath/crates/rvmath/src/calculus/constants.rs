//! Calculus-related mathematical constants.

/// Euler-Mascheroni constant (γ gamma)
///
/// The limiting difference between the harmonic series and the natural logarithm.
/// γ ≈ 0.5772156649015328606065120900824024310421...
///
/// Used in: asymptotic approximations, special functions (digamma, polygamma)
pub const EULER_MASCHERONI: f64 = 0.577215664901533;

/// Reciprocal of Euler-Mascheroni constant
pub const EULER_MASCHERONI_RECIP: f64 = 1.0 / EULER_MASCHERONI;

/// Catalan's constant (G)
///
/// G = β(2) = Σ((-1)^n / (2n+1)²) for n=0 to ∞
/// G ≈ 0.915965594177219015054603514932384110774...
///
/// Appears in: Dirichlet beta function evaluations, combinatorics
pub const CATALAN: f64 = 0.915965594177219;

/// Apéry's constant (ζ(3))
///
/// ζ(3) = Σ(1/n³) for n=1 to ∞
/// ζ(3) ≈ 1.202056903159594285399738161511449990765...
///
/// Appears in: Riemann zeta function evaluations, physics
pub const APERY: f64 = 1.202056903159594;

/// Feigenbaum constant delta (δ)
///
/// δ ≈ 4.6692016091029906718532038204662016172...
///
/// Rate at which period-doubling bifurcations occur in chaotic systems
pub const FEIGENBAUM_DELTA: f64 = 4.669201609102991;

/// Feigenbaum constant alpha (α)
///
/// α ≈ 2.5029078750958928485566356762090220976...
///
/// Related to scaling in chaotic period-doubling
pub const FEIGENBAUM_ALPHA: f64 = 2.502907875095893;

