CREATE TABLE messages (
  id                  VARCHAR(255) PRIMARY KEY,
  chat_id             VARCHAR(255) NOT NULL,
  user_id             VARCHAR(255) NOT NULL,
  role                VARCHAR(255) NOT NULL DEFAULT 'user',
  body                TEXT         NOT NULL,
  version             INT          NOT NULL DEFAULT 1,
  created_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  INDEX idx_msgs_chat      (chat_id)
);


