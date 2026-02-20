use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════
// EXISTING REGISTRY TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Represents a smart contract in the registry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Contract {
    pub id: Uuid,
    pub contract_id: String,
    pub wasm_hash: String,
    pub name: String,
    pub description: Option<String>,
    pub publisher_id: Uuid,
    pub network: Network,
    pub is_verified: bool,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Network where the contract is deployed
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "network_type", rename_all = "lowercase")]
pub enum Network {
    Mainnet,
    Testnet,
    Futurenet,
}

/// Contract version information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContractVersion {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub version: String,
    pub wasm_hash: String,
    pub source_url: Option<String>,
    pub commit_hash: Option<String>,
    pub release_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Verification status and details
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Verification {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub status: VerificationStatus,
    pub source_code: Option<String>,
    pub build_params: Option<serde_json::Value>,
    pub compiler_version: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Verification status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "verification_status", rename_all = "lowercase")]
pub enum VerificationStatus {
    Pending,
    Verified,
    Failed,
}

/// Publisher/developer information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Publisher {
    pub id: Uuid,
    pub stellar_address: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub github_url: Option<String>,
    pub website: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Contract interaction statistics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContractStats {
    pub contract_id: Uuid,
    pub total_deployments: i64,
    pub total_interactions: i64,
    pub unique_users: i64,
    pub last_interaction: Option<DateTime<Utc>>,
}

/// Request to publish a new contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRequest {
    pub contract_id: String,
    pub name: String,
    pub description: Option<String>,
    pub network: Network,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub source_url: Option<String>,
    pub publisher_address: String,
}

/// Request to verify a contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub contract_id: String,
    pub source_code: String,
    pub build_params: serde_json::Value,
    pub compiler_version: String,
}

/// Search/filter parameters for contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractSearchParams {
    pub query: Option<String>,
    pub network: Option<Network>,
    pub verified_only: Option<bool>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}



/// Pagination params for contract versions (limit/offset style)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionPaginationParams {
    #[serde(default = "default_version_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_version_limit() -> i64 {
    20
}

/// Paginated version response (limit/offset style per issue #32 spec)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedVersionResponse {
    pub items: Vec<ContractVersion>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

/// Paginated response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: i64, page: i64, page_size: i64) -> Self {
        let total_pages = (total as f64 / page_size as f64).ceil() as i64;
        Self {
            items,
            total,
            page,
            page_size,
            total_pages,
        }
    }
}

/// A single benchmark run result for one method invocation
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BenchmarkRun {
    pub id: Uuid,
    pub benchmark_id: Uuid,
    pub iteration: i32,
    pub execution_time_ms: f64,
    pub cpu_instructions: Option<i64>,
    pub memory_bytes: Option<i64>,
    pub created_at: DateTime<Utc>,
}

/// A benchmark session: N iterations of one method on one contract version
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BenchmarkRecord {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub contract_version: String,
    pub method_name: String,
    pub iterations: i32,
    /// JSON-encoded method arguments used for this benchmark
    pub args_json: Option<String>,
    /// Computed stats (populated after all runs complete)
    pub min_ms: f64,
    pub max_ms: f64,
    pub avg_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub stddev_ms: f64,
    pub contract_size_bytes: Option<i64>,
    pub status: BenchmarkStatus,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Status of a benchmark job
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum BenchmarkStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Performance regression alert
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PerformanceAlert {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub method_name: String,
    pub baseline_benchmark_id: Uuid,
    pub current_benchmark_id: Uuid,
    pub baseline_p95_ms: f64,
    pub current_p95_ms: f64,
    pub regression_pct: f64,
    pub alert_threshold_pct: f64,
    pub resolved: bool,
    pub created_at: DateTime<Utc>,
}

/// CLI request body — POST /contracts/:id/benchmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunBenchmarkRequest {
    /// Method name to benchmark (e.g. "transfer", "swap")
    pub method: String,
    /// Number of iterations to run (default 100, max 1000)
    #[serde(default = "default_iterations")]
    pub iterations: i32,
    /// JSON array of arguments to pass to the method
    pub args_json: Option<String>,
    /// Contract version tag (e.g. "v1.2.0")
    pub version: Option<String>,
    /// Regression alert threshold in % (default 10.0)
    #[serde(default = "default_threshold")]
    pub alert_threshold_pct: f64,
}

fn default_iterations() -> i32 {
    100
}
fn default_threshold() -> f64 {
    10.0
}

/// Response for a completed benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResponse {
    pub benchmark: BenchmarkRecord,
    /// Individual run timings for charting
    pub runs: Vec<BenchmarkRun>,
    /// Alert if regression detected vs previous baseline
    pub alert: Option<PerformanceAlert>,
    /// How this compares to the previous benchmark for the same method
    pub comparison: Option<BenchmarkComparison>,
}

/// Side-by-side comparison with a previous benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    pub previous_benchmark_id: Uuid,
    pub previous_version: String,
    pub previous_p95_ms: f64,
    pub current_p95_ms: f64,
    pub delta_ms: f64,
    pub delta_pct: f64,
    pub is_regression: bool,
}

/// Summary of all benchmarks for a contract (for the dashboard)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractBenchmarkSummary {
    pub contract_id: Uuid,
    pub total_benchmarks: i64,
    pub methods_benchmarked: Vec<String>,
    pub latest_benchmarks: Vec<BenchmarkRecord>,
    pub active_alerts: Vec<PerformanceAlert>,
}

/// Historical trend point for charting
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BenchmarkTrendPoint {
    pub benchmark_id: Uuid,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub p95_ms: f64,
    pub avg_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
}


// ═══════════════════════════════════════════════════════════════════════════
// QUALITY SCORE TYPES  — append to shared/src/lib.rs
// ═══════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────
// Raw metric types
// ─────────────────────────────────────────────────────────

/// Low-level source code metrics computed from raw source
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct CodeMetrics {
    pub lines_of_code: i64,
    pub blank_lines: i64,
    pub comment_lines: i64,
    pub cyclomatic_complexity: f64, // avg across all functions
    pub max_function_complexity: i64, // worst single function
    pub function_count: i64,
    pub avg_function_length: f64, // lines per function
    pub deeply_nested_count: i64, // blocks nested > 3 levels
}

/// Test quality metrics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct TestMetrics {
    pub test_count: i64,
    pub test_lines: i64,
    /// 0.0 – 1.0 (fraction of functions covered by at least one test)
    pub line_coverage: f64,
    pub function_coverage: f64,
    pub branch_coverage: f64,
    /// test lines / source lines
    pub test_to_code_ratio: f64,
    pub has_integration_tests: bool,
    pub has_property_tests: bool,
}

/// Documentation quality metrics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct DocMetrics {
    /// Fraction of pub functions with a doc comment
    pub public_fn_doc_coverage: f64,
    /// Fraction of pub structs/enums with a doc comment
    pub type_doc_coverage: f64,
    pub has_readme: bool,
    pub has_changelog: bool,
    pub has_license: bool,
    pub example_count: i64,
}

/// Security-specific quality metrics (derived from AuditRecord)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Default)]
pub struct SecurityMetrics {
    /// Overall audit score from AuditRecord (0–100)
    pub audit_score: f64,
    pub critical_findings: i64,
    pub high_findings: i64,
    pub medium_findings: i64,
    pub low_findings: i64,
    pub is_verified: bool,
    pub has_formal_audit: bool,
}

// ─────────────────────────────────────────────────────────
// Aggregate score + weights
// ─────────────────────────────────────────────────────────

/// Weights used when combining sub-scores into overall quality
/// All must sum to 1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityWeights {
    pub code: f64,       // default 0.25
    pub tests: f64,      // default 0.30
    pub docs: f64,       // default 0.20
    pub security: f64,   // default 0.25
}

impl Default for QualityWeights {
    fn default() -> Self {
        Self { code: 0.25, tests: 0.30, docs: 0.20, security: 0.25 }
    }
}

impl QualityWeights {
    pub fn is_valid(&self) -> bool {
        let sum = self.code + self.tests + self.docs + self.security;
        (sum - 1.0).abs() < 1e-6
    }
}

/// Per-dimension score breakdown (each 0–100)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QualityScoreBreakdown {
    pub code_score: f64,
    pub test_score: f64,
    pub doc_score: f64,
    pub security_score: f64,
    pub overall_score: f64,
}

/// Quality badge derived from overall score
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QualityBadge {
    Excellent,  // 90–100
    Good,       // 75–89
    Fair,       // 50–74
    Poor,       // 25–49
    Critical,   // 0–24
}

impl QualityBadge {
    pub fn from_score(score: f64) -> Self {
        match score as u32 {
            90..=100 => QualityBadge::Excellent,
            75..=89  => QualityBadge::Good,
            50..=74  => QualityBadge::Fair,
            25..=49  => QualityBadge::Poor,
            _        => QualityBadge::Critical,
        }
    }
}

impl std::fmt::Display for QualityBadge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// ─────────────────────────────────────────────────────────
// Database row — one quality snapshot per contract version
// ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QualityRecord {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub contract_version: String,
    // raw metrics stored as JSONB
    pub code_metrics: serde_json::Value,
    pub test_metrics: serde_json::Value,
    pub doc_metrics: serde_json::Value,
    pub security_metrics: serde_json::Value,
    // aggregate scores
    pub code_score: f64,
    pub test_score: f64,
    pub doc_score: f64,
    pub security_score: f64,
    pub overall_score: f64,
    pub badge: String,
    pub computed_at: DateTime<Utc>,
}

// ─────────────────────────────────────────────────────────
// Trend / history
// ─────────────────────────────────────────────────────────

/// One point in a quality trend chart
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QualityTrendPoint {
    pub quality_id: Uuid,
    pub contract_version: String,
    pub computed_at: DateTime<Utc>,
    pub overall_score: f64,
    pub code_score: f64,
    pub test_score: f64,
    pub doc_score: f64,
    pub security_score: f64,
    pub badge: String,
}

// ─────────────────────────────────────────────────────────
// Benchmarking — comparison to similar contracts
// ─────────────────────────────────────────────────────────

/// How this contract's quality compares to its category peers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryBenchmark {
    pub category: String,
    pub peer_count: i64,
    pub category_avg_score: f64,
    pub category_p25_score: f64,
    pub category_p75_score: f64,
    pub category_p95_score: f64,
    pub this_contract_score: f64,
    /// percentile rank within the category (0–100)
    pub percentile_rank: f64,
    pub above_average: bool,
}

// ─────────────────────────────────────────────────────────
// Quality threshold / target
// ─────────────────────────────────────────────────────────

/// A quality gate — minimum scores that must be met
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QualityThreshold {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub min_overall_score: f64,
    pub min_code_score: f64,
    pub min_test_score: f64,
    pub min_doc_score: f64,
    pub min_security_score: f64,
    pub fail_on_critical_finding: bool,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Result of checking a score against a threshold
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdCheckResult {
    pub passed: bool,
    pub violations: Vec<ThresholdViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdViolation {
    pub dimension: String,
    pub required: f64,
    pub actual: f64,
    pub gap: f64,
}

// ─────────────────────────────────────────────────────────
// API request / response shapes
// ─────────────────────────────────────────────────────────

/// POST /contracts/:id/quality  (trigger a fresh calculation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeQualityRequest {
    pub source_code: String,
    pub version: String,
    pub test_output: Option<String>,    // cargo test output with coverage
    pub audit_id: Option<Uuid>,         // link existing audit result
    pub weights: Option<QualityWeights>,
}

/// Full response for GET /contracts/:id/quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityResponse {
    pub record: QualityRecord,
    pub breakdown: QualityScoreBreakdown,
    pub code_metrics: CodeMetrics,
    pub test_metrics: TestMetrics,
    pub doc_metrics: DocMetrics,
    pub security_metrics: SecurityMetrics,
    pub badge: QualityBadge,
    pub threshold_result: Option<ThresholdCheckResult>,
    pub benchmark: Option<CategoryBenchmark>,
}

/// POST /contracts/:id/quality/threshold
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetThresholdRequest {
    pub min_overall_score: f64,
    pub min_code_score: f64,
    pub min_test_score: f64,
    pub min_doc_score: f64,
    pub min_security_score: f64,
    #[serde(default)]
    pub fail_on_critical_finding: bool,
    pub created_by: String,
}
// ═══════════════════════════════════════════════════════════════════════════
// SECURITY AUDIT TYPES
// ═══════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────
// Static checklist definition types
// ─────────────────────────────────────────────────────────

/// Category of a security checklist item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CheckCategory {
    InputValidation,
    StateManagement,
    AccessControl,
    Reentrancy,
    NumericalSafety,
    AuthenticationAuthorization,
    DataSerialization,
    ErrorHandling,
    StoragePatterns,
    TokenSafety,
    EventLogging,
    Upgradeability,
    CrossContractCalls,
    ResourceLimits,
}

impl std::fmt::Display for CheckCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CheckCategory::InputValidation => "Input Validation",
            CheckCategory::StateManagement => "State Management",
            CheckCategory::AccessControl => "Access Control",
            CheckCategory::Reentrancy => "Reentrancy",
            CheckCategory::NumericalSafety => "Numerical Safety",
            CheckCategory::AuthenticationAuthorization => "Authentication & Authorization",
            CheckCategory::DataSerialization => "Data Serialization",
            CheckCategory::ErrorHandling => "Error Handling",
            CheckCategory::StoragePatterns => "Storage Patterns",
            CheckCategory::TokenSafety => "Token Safety",
            CheckCategory::EventLogging => "Event Logging",
            CheckCategory::Upgradeability => "Upgradeability",
            CheckCategory::CrossContractCalls => "Cross-Contract Calls",
            CheckCategory::ResourceLimits => "Resource Limits",
        };
        write!(f, "{}", s)
    }
}

/// Severity of a security finding
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Describes how a checklist item can be detected
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DetectionMethod {
    /// Detected purely by pattern-matching source code
    Automatic { patterns: Vec<String> },
    /// Must be reviewed by a human auditor
    Manual,
    /// Partially automatable — patterns hint but human confirms
    SemiAutomatic { patterns: Vec<String> },
}

/// One item in the security audit checklist (static/compile-time data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: String,
    pub category: CheckCategory,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub detection: DetectionMethod,
    pub remediation: String,
    pub references: Vec<String>,
}

// ─────────────────────────────────────────────────────────
// Runtime / database types
// ─────────────────────────────────────────────────────────

/// Status of a single checklist item within an audit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum CheckStatus {
    Passed,
    Failed,
    NotApplicable,
    #[default]
    Pending,
}

/// One row in `audit_checks` — per-check status within a single audit
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditCheckRow {
    pub id: Uuid,
    pub audit_id: Uuid,
    pub check_id: String,
    pub status: CheckStatus,
    pub notes: Option<String>,
    pub auto_detected: bool,
    pub evidence: Option<String>,
    pub updated_at: DateTime<Utc>,
}

/// One row in `security_audits` — a complete audit session for a contract
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditRecord {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub contract_source: Option<String>,
    pub auditor: String,
    pub audit_date: DateTime<Utc>,
    pub overall_score: f64,
    pub summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ─────────────────────────────────────────────────────────
// API request / response shapes
// ─────────────────────────────────────────────────────────

/// POST /contracts/:id/security-audit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuditRequest {
    pub auditor: String,
    pub source_code: Option<String>,
}

/// PATCH .../checks/:check_id
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckRequest {
    pub status: CheckStatus,
    pub notes: Option<String>,
}

/// Full audit response — static checklist metadata merged with live status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResponse {
    pub audit: AuditRecord,
    pub checks: Vec<CheckWithStatus>,
    pub category_scores: Vec<CategoryScore>,
    pub auto_detected_count: usize,
}

/// A checklist item merged with its current audit status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckWithStatus {
    // static metadata
    pub id: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub severity: String,
    pub detection_type: String,
    pub auto_patterns: Vec<String>,
    pub remediation: String,
    pub references: Vec<String>,
    // live audit state
    pub status: CheckStatus,
    pub notes: Option<String>,
    pub auto_detected: bool,
    pub evidence: Option<String>,
}

/// Per-category breakdown of the audit score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryScore {
    pub category: String,
    pub score: f64,
    pub passed: usize,
    pub total: usize,
    pub failed_critical: usize,
    pub failed_high: usize,
}

/// Lightweight score summary for contract card display
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContractSecuritySummary {
    pub audit_id: Uuid,
    pub audit_date: DateTime<Utc>,
    pub auditor: String,
    pub overall_score: f64,
    pub score_badge: String,
}

/// Query params for the Markdown export endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    #[serde(default = "default_true")]
    pub include_descriptions: bool,
    #[serde(default)]
    pub failures_only: bool,
}

fn default_true() -> bool {
    true
}
