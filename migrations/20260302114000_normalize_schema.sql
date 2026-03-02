-- users: id 规范为 CHAR(36)、审计列、索引（外键稍后统一添加）
ALTER TABLE users
    MODIFY id CHAR(36) NOT NULL,
    ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    ADD INDEX idx_users_admin_id (admin_id),
    ADD INDEX idx_users_location_id (location_id);

-- locations: id 规范为 CHAR(36)、审计列、索引（外键稍后统一添加）
ALTER TABLE locations
    MODIFY id CHAR(36) NOT NULL,
    ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    ADD INDEX idx_locations_admin_id (admin_id);

-- records: id 规范为 CHAR(36)、审计列（外键稍后统一添加）
ALTER TABLE records
    MODIFY id CHAR(36) NOT NULL,
    ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP;

-- 统一添加外键，确保引用列类型已更新
ALTER TABLE users
    ADD CONSTRAINT fk_users_location
        FOREIGN KEY (location_id) REFERENCES locations(id)
        ON DELETE SET NULL ON UPDATE CASCADE;

ALTER TABLE users
    ADD CONSTRAINT fk_users_admin
        FOREIGN KEY (admin_id) REFERENCES users(id)
        ON DELETE SET NULL ON UPDATE CASCADE;

ALTER TABLE locations
    ADD CONSTRAINT fk_locations_admin
        FOREIGN KEY (admin_id) REFERENCES users(id)
        ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE records
    ADD CONSTRAINT fk_records_user
        FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE records
    ADD CONSTRAINT fk_records_location
        FOREIGN KEY (location_id) REFERENCES locations(id)
        ON DELETE CASCADE ON UPDATE CASCADE;
