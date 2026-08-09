-- 每日两次打卡：增加打卡类型（in=上班卡 / out=下班卡）与打卡日期
ALTER TABLE records
    ADD COLUMN record_type VARCHAR(16) NOT NULL DEFAULT 'in' COMMENT '打卡类型：in=上班卡，out=下班卡',
    ADD COLUMN record_date DATE NULL COMMENT '打卡日期（服务端本地时区）';

-- 历史数据回填（timestamp 为秒级时间戳）
UPDATE records SET record_date = DATE(FROM_UNIXTIME(timestamp));

ALTER TABLE records
    MODIFY COLUMN record_date DATE NOT NULL,
    ADD INDEX idx_records_user_date_type (user_id, record_date, record_type);
