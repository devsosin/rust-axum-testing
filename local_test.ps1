$ErrorActionPreference = "Stop"

param(
    [string]$TARGET
)

# 변수 설정
$DOCKER_COMPOSE_FILE = "docker-compose-test.yml"
$SERVICE_NAME = "test-db"

# Health Check를 위한 최대 대기 시간 (초)
$MAX_WAIT = 60
$SLEEP_INTERVAL = 3

# Health Check 함수
function Check-Health {
    $container_id = docker compose -f $DOCKER_COMPOSE_FILE ps -q $SERVICE_NAME
    $status = docker inspect --format='{{.State.Health.Status}}' $container_id
    Write-Host "Current health status of $SERVICE_NAME: $status"

    if ($status -eq "healthy") {
        return $true
    } elseif ($status -eq "unhealthy") {
        Write-Host "Service $SERVICE_NAME is unhealthy."
        return $false
    } else {
        return $false
    }
}

Write-Host "#### 테스트 환경 준비 ####"

docker compose -f $DOCKER_COMPOSE_FILE up --build -d $SERVICE_NAME

Write-Host "Waiting for $SERVICE_NAME to become healthy..."
$SECONDS_WAITED = 0

while (-not (Check-Health)) {
    if ($SECONDS_WAITED -ge $MAX_WAIT) {
        Write-Host "Error: $SERVICE_NAME did not become healthy within $MAX_WAIT seconds."
        Write-Host "Shutting down Docker Compose services..."
        docker compose -f $DOCKER_COMPOSE_FILE down
        exit 1
    }
    Write-Host "Waiting for $SERVICE_NAME to be healthy... ($SECONDS_WAITED/$MAX_WAIT)"
    Start-Sleep -Seconds $SLEEP_INTERVAL
    $SECONDS_WAITED += $SLEEP_INTERVAL
}

Write-Host "$SERVICE_NAME is healthy."

Write-Host "#### 테스트 실행 ####"

$env:DATABASE_URL = "postgres://test:test1234@localhost:5432/test_db"

Write-Host "Running tests for domain::$TARGET..."
# cargo test -- --test-threads=1 "domain::$TARGET"

# Run cargo test and capture the output
$TestOutput = & cargo test -- --test-threads=1 domain::$TARGET 2>&1

# Extract the summary line that contains the test results
$SummaryLine = $TestOutput | Select-String '^test result:' | ForEach-Object { $_.Line }

# Use regex to extract the numbers of passed and failed tests
$Passed = if ($SummaryLine -match '(\d+)\s+passed;') { $Matches[1] } else { 0 }
$Failed = if ($SummaryLine -match '(\d+)\s+failed;') { $Matches[1] } else { 0 }
$Ignored = if ($SummaryLine -match '(\d+)\s+ignored;') { $Matches[1] } else { 0 }
$Measured = if ($SummaryLine -match '(\d+)\s+measured;') { $Matches[1] } else { 0 }
$FilteredOut = if ($SummaryLine -match '(\d+)\s+filtered out;') { $Matches[1] } else { 0 }

# Convert counts to integers
$Passed = [int]$Passed
$Failed = [int]$Failed
$Ignored = [int]$Ignored
$Measured = [int]$Measured
$FilteredOut = [int]$FilteredOut

# Calculate the total number of tests run (passed + failed)
$Total = $Passed + $Failed

Write-Host "#### 테스트 완료, 테스트 환경 정리 ####"

# Run docker compose down
docker compose -f "$DOCKER_COMPOSE_FILE" down

# Calculate the coverage percentage
if ($Total -ne 0) {
    $Coverage = [math]::Round(($Passed * 100.0) / $Total, 2)
} else {
    $Coverage = 0
}

Write-Host ""
Write-Host "Test Coverage: $Coverage% ($Passed out of $Total tests passed)"
Write-Host "#### 테스트 종료 #### "