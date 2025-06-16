CREATE TABLE shared_chats (
  id                VARCHAR(255) PRIMARY KEY,
  original_chat_id  VARCHAR(255) NOT NULL,  
  owner_user_id     VARCHAR(255) NOT NULL, 
  title             VARCHAR(255) NULL,
  created_at        TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
  
  INDEX idx_shared_chats_original_chat_id (original_chat_id),
  INDEX idx_shared_chats_owner_user_id    (owner_user_id)
);
