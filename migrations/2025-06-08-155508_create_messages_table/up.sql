CREATE TABLE messages (
  id                  VARCHAR(255) PRIMARY KEY,
  chat_id             VARCHAR(255) NOT NULL,
  user_id             VARCHAR(255) NOT NULL,
  role                VARCHAR(255) NOT NULL DEFAULT 'user',
  body                TEXT         NOT NULL,
  reasoning           TEXT         NULL,
  version             INT          NOT NULL DEFAULT 1,
  created_at          TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  updated_at          TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
  INDEX idx_msgs_chat      (chat_id)
);


