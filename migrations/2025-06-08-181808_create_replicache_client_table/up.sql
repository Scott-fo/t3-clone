CREATE TABLE replicache_clients (
    id VARCHAR(255) PRIMARY KEY,
    client_group_id VARCHAR(255) NOT NULL,
    last_mutation_id INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_client_group (client_group_id)
);

