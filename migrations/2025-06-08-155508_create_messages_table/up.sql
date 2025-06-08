CREATE TABLE messages (
  id                  VARCHAR(255) PRIMARY KEY,
  chat_id             VARCHAR(255) NOT NULL,
  user_id             VARCHAR(255) NOT NULL,
  body                TEXT         NOT NULL,
  version             INT          NOT NULL DEFAULT 1,
  created_at          DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at          DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  INDEX idx_msgs_chat      (chat_id)
);


