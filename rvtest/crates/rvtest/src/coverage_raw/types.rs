pub struct CoverageTotals {
    pub(crate) total_counters: u64,
    pub(crate) covered_counters: u64,
    pub(crate) total_functions: u64,
    pub(crate) covered_functions: u64,
}

impl CoverageTotals {
    pub(crate) fn new() -> Self {
        CoverageTotals { total_counters: 0, covered_counters: 0, total_functions: 0, covered_functions: 0 }
    }

    pub(crate) fn add(&mut self, other: &CoverageTotals) {
        self.total_counters += other.total_counters;
        self.covered_counters += other.covered_counters;
        self.total_functions += other.total_functions;
        self.covered_functions += other.covered_functions;
    }

    pub(crate) fn line_pct(&self) -> f64 {
        if self.total_counters > 0 {
            (self.covered_counters as f64 / self.total_counters as f64 * 100.0).min(100.0)
        } else {
            0.0
        }
    }

    pub(crate) fn func_pct(&self) -> f64 {
        if self.total_functions > 0 {
            (self.covered_functions as f64 / self.total_functions as f64 * 100.0).min(100.0)
        } else {
            0.0
        }
    }

    pub(crate) fn region_pct(&self) -> f64 {
        self.line_pct()
    }
}
