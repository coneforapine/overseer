-- Add migration script here
ALTER TABLE public.cases
    ADD COLUMN number SERIAL;