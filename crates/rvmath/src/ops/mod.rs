//! Functional-style operator modules.
//!
//! Provides free-function wrappers around common mathematical operations.
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`arithmetic`] | Basic arithmetic: add, sub, mul, div, rem, neg, abs |
//! | [`logarithm`] | Logarithmic functions: ln, log, log10, log2, ln_1p |
//! | [`trig`] | Trigonometric functions: sin, cos, tan, asin, acos, atan, atan2 |
//! | [`hyperbolic`] | Hyperbolic functions: sinh, cosh, tanh |

pub mod arithmetic;
pub mod logarithm;
pub mod trig;
pub mod hyperbolic;
