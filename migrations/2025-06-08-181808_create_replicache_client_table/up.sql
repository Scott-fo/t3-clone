CREATE TABLE replicache_clients (
    id VARCHAR(255) PRIMARY KEY,
    client_group_id VARCHAR(255) NOT NULL,
    last_mutation_id INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    INDEX idx_client_group (client_group_id)
);

