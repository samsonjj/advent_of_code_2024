pub mod aoc {
    // define a trait alias
    pub fn format_with_time<Output: std::fmt::Display, F: FnOnce() -> Output>(f: F) -> String {
        let now = std::time::Instant::now();
        let result = f();
        let elapsed = now.elapsed();

        format!("{} ({:}ms elapsed)", result, elapsed.as_millis())
    }
}
