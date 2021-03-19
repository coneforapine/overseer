-- 
CREATE TABLE public.guild_settings (
	id serial NOT NULL,
	guild_id varchar(30) UNIQUE NOT NULL,
	announcement_channel varchar(30) NULL,
	CONSTRAINT guild_settings_pkey PRIMARY KEY (id)
);