#!/bin/bash

# 1. 테스트 실행 및 커버리지 리포트 생성
./gradlew clean test jacocoTestReport

# 테스트 실패 시 스크립트 종료
if [ $? -ne 0 ]; then
    echo "테스트 실패. 배포를 중단합니다."
    exit 1
fi

# 2. 커버리지 퍼센트 계산
# JaCoCo 리포트 파일 경로
JACOCO_REPORT="build/reports/jacoco/test/jacocoTestReport.xml"

# 커버리지 데이터 추출 (LINE 커버리지 기준)
line=$(grep '<counter type="LINE"' $JACOCO_REPORT)
covered=$(echo $line | sed 's/.*covered="\([0-9]*\)".*/\1/')
missed=$(echo $line | sed 's/.*missed="\([0-9]*\)".*/\1/')

if [ -z "$covered" ] || [ -z "$missed" ]; then
    echo "커버리지 데이터를 파싱할 수 없습니다. 배포를 중단합니다."
    exit 1
fi

total_lines=$((covered + missed))
coverage_percentage=$(echo "scale=2; $covered*100/($covered+$missed)" | bc)

echo "현재 라인 커버리지는 $coverage_percentage% 입니다."

# 커버리지가 80% 이상인지 확인
required_coverage=80.0
coverage_check=$(echo "$coverage_percentage >= $required_coverage" | bc -l)

if [ "$coverage_check" -eq 1 ]; then
    echo "커버리지가 $required_coverage% 이상입니다. 배포를 진행합니다."
else
    echo "커버리지가 $required_coverage% 미만입니다. 배포를 중단합니다."
    exit 1
fi

chmod +x ./gradlew

# 3. 빌드 진행
./gradlew clean build

# 빌드 실패 시 스크립트 종료
if [ $? -ne 0 ]; then
    echo "빌드 실패. 배포를 중단합니다."
    exit 1
fi

# 4. AWS 서버 정보 설정
AWS_USER=                                       # AWS 사용자 이름
AWS_HOST=                                       # AWS 호스트 주소
AWS_KEY_PATH=                                   # SSH 키 파일 경로
REMOTE_DIR=                                     # 원격 서버에서의 디렉토리
LOCAL_JAR=                                      # 빌드된 JAR 파일 경로 -> 파일 확인

# 5. 빌드 파일을 AWS 서버로 전송
scp -i $AWS_KEY_PATH $LOCAL_JAR $AWS_USER@$AWS_HOST:$REMOTE_DIR

# docker-compose.yml 파일 전송을 추가할 수 있음

# 6. AWS 서버에서 Docker Compose 재시작
ssh -i $AWS_KEY_PATH $AWS_USER@$AWS_HOST << EOF
    cd $REMOTE_DIR
    sudo docker-compose down
    sudo docker-compose up --build -d
    exit
EOF

echo "배포가 완료되었습니다."
