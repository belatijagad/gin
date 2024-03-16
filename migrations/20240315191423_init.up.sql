-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS wallets (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        wallet_name VARCHAR(255) NOT NULL UNIQUE,
        balance INT DEFAULT 0,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
