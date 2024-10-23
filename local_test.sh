#!/bin/bash
set -e

TARGET=$1

# 변수 설정
DOCKER_COMPOSE_FILE="docker-compose-test.yml"
SERVICE_NAME="test-db"

# Health Check를 위한 최대 대기 시간 (초)
MAX_WAIT=60
SLEEP_INTERVAL=3

# Health Check 함수
check_health() {
  local status
  status=$(docker inspect --format='{{.State.Health.Status}}' "$(docker compose -f "$DOCKER_COMPOSE_FILE" ps -q "$SERVICE_NAME")")
  echo "Current health status of $SERVICE_NAME: $status"
  if [ "$status" == "healthy" ]; then
    return 0
  elif [ "$status" == "unhealthy" ]; then
    echo "Service $SERVICE_NAME is unhealthy."
    return 1
  else
    return 1
  fi
}

echo "#### 테스트 환경 준비 ####"

docker compose -f "$DOCKER_COMPOSE_FILE" up --build -d "$SERVICE_NAME"

echo "Waiting for $SERVICE_NAME to become healthy..."
SECONDS_WAITED=0

while ! check_health; do
  if [ "$SECONDS_WAITED" -ge "$MAX_WAIT" ]; then
    echo "Error: $SERVICE_NAME did not become healthy within $MAX_WAIT seconds."
    echo "Shutting down Docker Compose services..."
    docker compose -f "$DOCKER_COMPOSE_FILE" down
    exit 1
  fi
  echo "Waiting for $SERVICE_NAME to be healthy... ($SECONDS_WAITED/$MAX_WAIT)"
  sleep "$SLEEP_INTERVAL"
  SECONDS_WAITED=$((SECONDS_WAITED + SLEEP_INTERVAL))
done

echo "$SERVICE_NAME is healthy."

echo "#### 테스트 실행 ####"

export DATABASE_URL=postgres://test:test1234@localhost:5432/test_db
# echo $DATABASE_URL

echo "Running tests for domain::$TARGET..."
# cargo test -- --test-threads=1 domain::$TARGET 

# Run cargo test and capture the output
TEST_OUTPUT=$(cargo test -- --test-threads=1 domain::$TARGET 2>&1)

# echo "$TEST_OUTPUT"

# Extract the summary line that contains the test results
SUMMARY_LINE=$(echo "$TEST_OUTPUT" | grep "^test result:")

# echo "$SUMMARY_LINE"

# Use grep and awk to extract the numbers of passed and failed tests
PASSED=$(echo "$SUMMARY_LINE" | awk '{for(i=1;i<=NF;i++){if($i=="passed;"){print $(i-1)}}}')
FAILED=$(echo "$SUMMARY_LINE" | awk '{for(i=1;i<=NF;i++){if($i=="failed;"){print $(i-1)}}}')
IGNORED=$(echo "$SUMMARY_LINE" | awk '{for(i=1;i<=NF;i++){if($i=="ignored;"){print $(i-1)}}}')
MEASURED=$(echo "$SUMMARY_LINE" | awk '{for(i=1;i<=NF;i++){if($i=="measured;"){print $(i-1)}}}')
FILTERED_OUT=$(echo "$SUMMARY_LINE" | awk '{for(i=1;i<=NF;i++){if($i=="out;"){print $(i-2)}}}')

# Set default values if any of the counts are missing
PASSED=${PASSED:-0}
FAILED=${FAILED:-0}
IGNORED=${IGNORED:-0}
MEASURED=${MEASURED:-0}
FILTERED_OUT=${FILTERED_OUT:-0}

# Calculate the total number of tests run (passed + failed)
TOTAL=$((PASSED + FAILED))

echo "#### 테스트 완료, 테스트 환경 정리 ####"

docker compose -f "$DOCKER_COMPOSE_FILE" down

# Calculate the coverage percentage
if [ "$TOTAL" -ne 0 ]; then
  COVERAGE=$(echo "scale=2; $PASSED * 100 / $TOTAL" | bc)
else
  COVERAGE=0
fi

echo
echo "Test Coverage: $COVERAGE% ($PASSED out of $TOTAL tests passed)"

echo "#### 테스트 종료 #### "