use rvtest::spec::describe;
use rvmath::geometry::{
    cone_surface, cone_surface_lateral, cone_volume, cylinder_surface, cylinder_volume,
    ellipse_area, ellipse_perimeter, polygon_area, polygon_perimeter, sphere_surface,
    sphere_volume, torus_surface, torus_volume, triangle_area, triangle_area_heron,
};
use rvmath::geometry_constants::{GOLDEN_RATIO, PI, PI_2, PI_4, SQRT_2, SQRT_3, SQRT_5, TAU};

const EPS: f64 = 1e-10;

fn abs_f64(x: impl Into<f64>) -> f64 { x.into().abs() }

#[test]
fn geometry_tests() {
    describe("3D Shapes")
        .it("sphere volume", || {
            let vol = sphere_volume(3.0_f64);
            let expected = (4.0 / 3.0) * std::f64::consts::PI * 27.0;
            assert!(abs_f64(vol - expected) < EPS);
        })
        .it("sphere surface", || {
            let area = sphere_surface(3.0_f64);
            let expected = 4.0 * std::f64::consts::PI * 9.0;
            assert!(abs_f64(area - expected) < EPS);
        })
        .it("cylinder volume", || {
            let vol = cylinder_volume(2.0_f64, 5.0_f64);
            let expected = std::f64::consts::PI * 4.0 * 5.0;
            assert!(abs_f64(vol - expected) < EPS);
        })
        .it("cylinder surface", || {
            let area = cylinder_surface(2.0_f64, 5.0_f64);
            let expected = 2.0 * std::f64::consts::PI * 2.0 * (2.0 + 5.0);
            assert!(abs_f64(area - expected) < EPS);
        })
        .it("cone volume", || {
            let vol = cone_volume(2.0_f64, 5.0_f64);
            let expected = (1.0 / 3.0) * std::f64::consts::PI * 4.0 * 5.0;
            assert!(abs_f64(vol - expected) < EPS);
        })
        .it("torus volume", || {
            let vol = torus_volume(4.0_f64, 1.0_f64);
            let expected = 2.0 * std::f64::consts::PI * std::f64::consts::PI * 4.0 * 1.0;
            assert!(abs_f64(vol - expected) < EPS);
        })
        .it("torus surface", || {
            let area = torus_surface(4.0_f64, 1.0_f64);
            let expected = 4.0 * std::f64::consts::PI * std::f64::consts::PI * 4.0 * 1.0;
            assert!(abs_f64(area - expected) < EPS);
        })
        .tag("3d")
        .run();

    describe("Cone Surface")
        .it("lateral surface normal", || {
            let area = cone_surface_lateral(3.0_f64, 4.0_f64);
            let expected = std::f64::consts::PI * 3.0 * 5.0;
            assert!(abs_f64(area - expected) < EPS);
        })
        .it("lateral surface zero radius", || {
            let area = cone_surface_lateral(0.0_f64, 4.0_f64);
            assert!(abs_f64(area - 0.0_f64) < EPS);
        })
        .it("lateral surface zero height", || {
            let area = cone_surface_lateral(3.0_f64, 0.0_f64);
            let expected = std::f64::consts::PI * 3.0 * 3.0;
            assert!(abs_f64(area - expected) < EPS);
        })
        .it("total surface normal", || {
            let area = cone_surface(3.0_f64, 4.0_f64);
            let lateral = cone_surface_lateral(3.0_f64, 4.0_f64);
            let base = std::f64::consts::PI * 9.0;
            assert!(abs_f64(area - (lateral + base)) < EPS);
        })
        .it("total surface zero radius", || {
            let area = cone_surface(0.0_f64, 4.0_f64);
            assert!(abs_f64(area - 0.0_f64) < EPS);
        })
        .it("total surface zero height", || {
            let area = cone_surface(3.0_f64, 0.0_f64);
            let expected = std::f64::consts::PI * 9.0 + std::f64::consts::PI * 3.0 * 3.0;
            assert!(abs_f64(area - expected) < EPS);
        })
        .tag("cone")
        .run();

    describe("2D Shapes")
        .it("ellipse area", || {
            let area = ellipse_area(5.0_f64, 3.0_f64);
            let expected = std::f64::consts::PI * 5.0 * 3.0;
            assert!(abs_f64(area - expected) < EPS);
        })
        .it("ellipse perimeter for circle", || {
            let perim = ellipse_perimeter(5.0_f64, 5.0_f64);
            let expected = 2.0 * std::f64::consts::PI * 5.0;
            assert!(abs_f64(perim - expected) < 0.0001);
        })
        .it("triangle area via heron", || {
            let area = triangle_area_heron(3.0_f64, 4.0_f64, 5.0_f64);
            assert!(abs_f64(area - 6.0_f64) < EPS);
        })
        .it("triangle area equilateral", || {
            let area = triangle_area_heron(2.0_f64, 2.0_f64, 2.0_f64);
            let expected = (3.0_f64.sqrt() / 4.0) * 4.0;
            assert!(abs_f64(area - expected) < EPS);
        })
        .it("triangle area base*height/2", || {
            let area = triangle_area(4.0_f64, 3.0_f64);
            assert!(abs_f64(area - 6.0_f64) < EPS);
        })
        .it("polygon area hexagon", || {
            let area = polygon_area(6, 2.0_f64);
            let expected = (3.0 * 3.0_f64.sqrt() / 2.0) * 4.0;
            assert!(abs_f64(area - expected) < EPS);
        })
        .it("polygon perimeter hexagon", || {
            let perim = polygon_perimeter(6, 2.0_f64);
            assert!(abs_f64(perim - 12.0_f64) < EPS);
        })
        .it("polygon area invalid sides returns 0", || {
            let area = polygon_area(2, 1.0_f64);
            assert_eq!(area, 0.0);
        })
        .tag("2d")
        .run();

    describe("Constants")
        .it("pi matches std", || {
            assert!(f64::abs(PI - std::f64::consts::PI) < 1e-15);
        })
        .it("golden_ratio = (1 + √5)/2", || {
            let calculated = (1.0 + SQRT_5) / 2.0;
            assert!(f64::abs(GOLDEN_RATIO - calculated) < 1e-15);
        })
        .it("sqrt values are consistent", || {
            assert!((SQRT_2 * SQRT_2 - 2.0_f64).abs() < 1e-14);
            assert!((SQRT_3 * SQRT_3 - 3.0_f64).abs() < 1e-14);
            assert!((SQRT_5 * SQRT_5 - 5.0_f64).abs() < 1e-14);
        })
        .it("pi relationship: π₂ = π/2, π₄ = π/4, τ = 2π", || {
            assert!(f64::abs(PI_2 * 2.0 - PI) < 1e-15);
            assert!(f64::abs(PI_4 * 4.0 - PI) < 1e-15);
            assert!(f64::abs(TAU - PI * 2.0) < 1e-15);
        })
        .tag("constants")
        .run()
        .assert_all_pass();
}
