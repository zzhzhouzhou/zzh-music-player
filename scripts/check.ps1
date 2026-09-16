$ErrorActionPreference = "Stop"

function Invoke-Check {
    param(
        [string]$Name,
        [scriptblock]$Command
    )

    Write-Host "`n==> $Name" -ForegroundColor Cyan
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "$Name failed with exit code $LASTEXITCODE"
    }
}

Invoke-Check "Formatting" { cargo fmt --check }
Invoke-Check "Compilation" { cargo check --locked }
Invoke-Check "Tests" { cargo test --locked }
Invoke-Check "Clippy" { cargo clippy --locked --all-targets -- -D warnings }

Write-Host "`nAll checks passed." -ForegroundColor Green
