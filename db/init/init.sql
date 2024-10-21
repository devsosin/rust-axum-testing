CREATE TABLE tb_user(
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,

    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP
);

CREATE TABLE tb_article(
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255),
    content TEXT,

    writer_id BIGINT,

    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,

    FOREIGN KEY (writer_id) REFERENCES tb_user(id)
);

INSERT INTO tb_user(username) VALUES
    ('test_user'),
    ('test_user2');

INSERT INTO tb_article(title, content, writer_id) VALUES
    ('테스트 제목1', '테스트 내용1', 1),
    ('테스트 제목2', '테스트 내용2', 2);
