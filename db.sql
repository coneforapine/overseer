-- Prepwork for enabling uuid
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- guild settings
CREATE TABLE IF NOT EXISTS public.guild_settings (
    id uuid DEFAULT uuid_generate_v4(),
    guild_id varchar(30) UNIQUE NOT NULL,
    announcement_channel varchar(30) NULL,
    case_channel varchar(30) NULL,
    CONSTRAINT guild_settings_pkey PRIMARY KEY (id)
);

-- moderator commands
CREATE TYPE case_types AS ENUM ('ban', 'warn', 'kick');


CREATE TABLE public.cases (
    id uuid DEFAULT uuid_generate_v4(),
    user_id varchar(30) NOT NULL,
    moderator_id varchar(30) NOT NULL,
    reason text NULL,
    case_type case_types NOT NULL
    CONSTRAINT cases_pkey PRIMARY KEY (id)
)