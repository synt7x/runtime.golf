CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    admin BOOLEAN NOT NULL DEFAULT FALSE,
    about TEXT DEFAULT '',
    github_id INTEGER UNIQUE NOT NULL,
    username VARCHAR(255) NOT NULL
);

CREATE TABLE holes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE submissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    hole_id INTEGER NOT NULL REFERENCES holes(id) ON DELETE CASCADE,
    code TEXT NOT NULL,
    language VARCHAR(50) NOT NULL,
    runtime_ms INTEGER NOT NULL,
    avg_test_time_ms INTEGER NOT NULL,
    char_count INTEGER NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'failed', 'passed')),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE VIEW leaderboard AS
SELECT
    s.hole_id,
    s.user_id,
    u.username,
    u.github_id,
    s.language,
    MIN(s.runtime_ms) AS best_runtime_ms,
    MIN(s.avg_test_time_ms) AS best_avg_test_time_ms,
    MIN(s.char_count) AS best_char_count
FROM submissions s
JOIN users u on s.user_id = u.id
JOIN holes h on s.hole_id = h.id
WHERE s.status = 'passed'
GROUP BY s.hole_id, s.user_id, s.language
ORDER BY best_runtime_ms ASC;

CREATE VIEW languages AS
SELECT
    language,
    COUNT(*) AS solution_count
FROM submissions
WHERE status = 'passed'
GROUP BY language
ORDER BY solution_count DESC;