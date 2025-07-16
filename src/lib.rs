pub mod aoc {
    // define a trait alias
    pub fn format_with_time<Output: std::fmt::Display, F: FnOnce() -> Output>(f: F) -> String {
        let now = std::time::Instant::now();
        let result = f();
        let elapsed = now.elapsed();

        format!("{} ({:}ms elapsed)", result, elapsed.as_millis())
    }

    pub fn run_parts<T: std::fmt::Display, F1: FnOnce(&str) -> T, F2: FnOnce(&str) -> T>(
        input: &str,
        part_1: F1,
        part_2: F2,
    ) {
        println!("part_1: {}", format_with_time(|| part_1(input)));
        println!("part_2: {}", format_with_time(|| part_2(input)));
    }
}
