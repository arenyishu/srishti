use colored::*;

pub struct VerificationReport {
    pub passed: usize,
    pub failed: usize,
    pub total: usize,
    results: Vec<(String, bool)>,
}

impl VerificationReport {
    pub fn new() -> Self {
        Self {
            passed: 0,
            failed: 0,
            total: 0,
            results: Vec::new(),
        }
    }

    pub fn record(&mut self, name: &str, success: bool) {
        self.total += 1;
        if success {
            self.passed += 1;
            println!("{} {}", "PASS".green().bold(), name);
        } else {
            self.failed += 1;
            println!("{} {}", "FAIL".red().bold(), name);
        }
        self.results.push((name.to_string(), success));
    }

    pub fn print_summary(&self) {
        println!("\n{:-<52}", "");
        println!("Total Checks: {}", self.total);
        println!("Passed: {}", self.passed);
        println!("Failed: {}", self.failed);
        println!("\nOverall Status:\n");
        if self.failed == 0 {
            println!("{}", "SRISHTI OS VERIFIED".green().bold());
            println!("The Operating System for AI Agents");
        } else {
            println!("{}", "SRISHTI OS VERIFICATION FAILED".red().bold());
        }
        println!("{:=<52}", "");
    }
}
