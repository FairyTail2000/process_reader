CREATE TABLE system (
    id UUID PRIMARY KEY,
    total_memory INTEGER,
    used_memory INTEGER,
    total_swap INTEGER,
    used_swap INTEGER,
    timestamp TIMESTAMP
);

CREATE TABLE process (
    id UUID PRIMARY KEY,
    gid INTEGER,
    uid INTEGER,
    pid INTEGER,
    name TEXT,
    path TEXT,
    cpu DECIMAL,
    memory BIGINT,
    virtual_memory BIGINT,
    start_time BIGINT,
    parent INTEGER,
    system_id UUID,
    FOREIGN KEY (system_id) REFERENCES system
);

CREATE TABLE thread (
    id UUID PRIMARY KEY,
    gid INTEGER,
    uid INTEGER,
    cpu INTEGER,
    memory BIGINT,
    virtual_memory BIGINT,
    start_time BIGINT,
    parent INTEGER,
    process_id UUID,
    FOREIGN KEY (process_id) REFERENCES process
);