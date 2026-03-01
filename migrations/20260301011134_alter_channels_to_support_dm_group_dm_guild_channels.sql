-- Migration: alter channels to support dm, group_dm, guild_text, guild_voice

ALTER TABLE channels
    ALTER COLUMN guild_id DROP NOT NULL,
ALTER COLUMN name DROP NOT NULL,
    ALTER COLUMN type TYPE VARCHAR(20);

ALTER TABLE channels
    ADD COLUMN owner_id UUID REFERENCES users(id) ON DELETE SET NULL;

CREATE TABLE channel_recipients (
                                    channel_id UUID NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
                                    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                                    PRIMARY KEY (channel_id, user_id)
);

UPDATE channels SET type = 'guild_' || type WHERE type IN ('text', 'voice', 'announcement');

ALTER TABLE channels
    ADD CONSTRAINT guild_required_for_guild_channels
        CHECK (
            (type LIKE 'guild_%' AND guild_id IS NOT NULL) OR
            (type NOT LIKE 'guild_%' AND guild_id IS NULL)
            );

ALTER TABLE channels
    ADD CONSTRAINT name_required
        CHECK (
            (type IN ('guild_text', 'guild_voice', 'guild_announcement', 'group_dm') AND name IS NOT NULL) OR
            (type = 'dm')
            );

CREATE INDEX idx_channel_recipients_user_id ON channel_recipients(user_id);