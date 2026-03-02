CREATE TABLE IF NOT EXISTS records (
    id VARCHAR(64) PRIMARY KEY,
    user_id VARCHAR(64) NOT NULL,
    location_id VARCHAR(64) NOT NULL,
    latitude DOUBLE NOT NULL,
    longitude DOUBLE NOT NULL,
    timestamp BIGINT NOT NULL,
    status VARCHAR(16) NOT NULL,
    error_message TEXT NULL,
    INDEX idx_user_id (user_id),
    INDEX idx_location_id (location_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
