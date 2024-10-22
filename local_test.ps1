param (
    [string]$TARGET
)

$env:DATABASE_URL = "postgres://test:test1234@localhost:5432/test_db"

# Write-Output $env:DATABASE_URL

cargo test "domain::$TARGET"