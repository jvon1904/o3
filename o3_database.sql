--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2 (Postgres.app)
-- Dumped by pg_dump version 17.2

CREATE DATABASE o3_database;

\connect o3_database

CREATE TABLE public.summaries (
    id integer NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    summary text
);

ALTER TABLE public.summaries OWNER TO __POSTGRES_USER__;

CREATE SEQUENCE public.summaries_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.summaries_id_seq OWNER TO __POSTGRES_USER__;

ALTER SEQUENCE public.summaries_id_seq OWNED BY public.summaries.id;

ALTER TABLE ONLY public.summaries ALTER COLUMN id SET DEFAULT nextval('public.summaries_id_seq'::regclass);

ALTER TABLE ONLY public.summaries
    ADD CONSTRAINT summaries_pkey PRIMARY KEY (id);
