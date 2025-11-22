-- Create table: profiles
CREATE TABLE profiles (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    pass_hash TEXT NOT NULL
);

-- Create table: secrets
CREATE TABLE secrets (
    id TEXT PRIMARY KEY NOT NULL,
    profile_id TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    data TEXT NOT NULL,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);