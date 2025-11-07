-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    wallet_address VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL CHECK (role IN ('professional', 'institution', 'employer')),
    is_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create institutions table
CREATE TABLE IF NOT EXISTS institutions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    institution_name VARCHAR(255) NOT NULL,
    institution_type VARCHAR(100) NOT NULL,
    country VARCHAR(100) NOT NULL,
    accreditation_number VARCHAR(255),
    is_accredited BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id)
);

-- Create credentials table
CREATE TABLE IF NOT EXISTS credentials (
    id UUID PRIMARY KEY,
    credential_id VARCHAR(255) UNIQUE NOT NULL,
    holder_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    issuer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    credential_type VARCHAR(50) NOT NULL CHECK (credential_type IN ('certificate', 'license', 'degree', 'workexperience', 'skill')),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    ipfs_hash VARCHAR(255) NOT NULL,
    chain_hash VARCHAR(255) NOT NULL,
    issue_date TIMESTAMPTZ NOT NULL,
    expiry_date TIMESTAMPTZ,
    status VARCHAR(50) NOT NULL CHECK (status IN ('pending', 'issued', 'revoked', 'expired')),
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_wallet ON users(wallet_address);
CREATE INDEX idx_institutions_user_id ON institutions(user_id);
CREATE INDEX idx_credentials_credential_id ON credentials(credential_id);
CREATE INDEX idx_credentials_holder_id ON credentials(holder_id);
CREATE INDEX idx_credentials_issuer_id ON credentials(issuer_id);
CREATE INDEX idx_credentials_status ON credentials(status);
