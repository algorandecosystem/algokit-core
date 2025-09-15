/// Common test result types
#[derive(Debug, uniffi::Record)]
pub struct TestResult {
    pub passed: bool,
    pub name: String,
    pub message: String,
    pub error: Option<String>,
}

#[derive(Debug, uniffi::Record)]
pub struct TestSuiteResult {
    pub name: String,
    pub results: Vec<TestResult>,
    pub all_passed: bool,
}

impl TestResult {
    pub fn success(name: &str, message: &str) -> Self {
        Self {
            passed: true,
            name: name.to_string(),
            message: message.to_string(),
            error: None,
        }
    }
    
    pub fn failure(name: &str, error: &str) -> Self {
        Self {
            passed: false,
            name: name.to_string(),
            message: "Test failed".to_string(),
            error: Some(error.to_string()),
        }
    }
}

impl TestSuiteResult {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            results: Vec::new(),
            all_passed: true,
        }
    }
    
    pub fn add_result(&mut self, result: TestResult) {
        if !result.passed {
            self.all_passed = false;
        }
        self.results.push(result);
    }
}