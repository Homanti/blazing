-- Guild members
CREATE TABLE guild_members (
    guild_id UUID NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    PRIMARY KEY (guild_id, user_id)
);

CREATE INDEX idx_guild_members_user_id ON guild_members(user_id);
CREATE INDEX idx_guild_members_guild_id ON guild_members(guild_id);