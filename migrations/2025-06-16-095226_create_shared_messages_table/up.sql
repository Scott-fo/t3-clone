CREATE TABLE shared_messages (
  id               VARCHAR(255) PRIMARY KEY,
  shared_chat_id   VARCHAR(255) NOT NULL,
  role             VARCHAR(255) NOT NULL DEFAULT 'user',
  body             TEXT         NOT NULL,
  reasoning        TEXT         NULL,
  created_at       TIMESTAMP    NOT NULL,
  
  INDEX idx_shared_msgs_chat (shared_chat_id)
);
