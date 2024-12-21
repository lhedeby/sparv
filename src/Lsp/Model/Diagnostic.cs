public record class Diagnostic(Range Range, DiagnosticSeverity? Severity, string? Source, string Message);
