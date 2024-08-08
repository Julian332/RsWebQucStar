-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```

--
-- PostgreSQL database dump
--

-- Dumped from database version 16.2 (Debian 16.2-1.pgdg120+2)
-- Dumped by pg_dump version 16.3

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: postgres
--




--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: postgres
--



--
-- Name: trigger_set_timestamp(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.trigger_set_timestamp() RETURNS trigger
    LANGUAGE plpgsql
AS $$
BEGIN
    NEW.update_time = NOW();
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.trigger_set_timestamp() OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;



--
-- Name: following_order; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.following_order (
                                        id bigint NOT NULL,
                                        deleted boolean DEFAULT false NOT NULL,
                                        create_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
                                        update_time timestamp with time zone
);


ALTER TABLE public.following_order OWNER TO postgres;

--
-- Name: following_order_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.following_order_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.following_order_id_seq OWNER TO postgres;

--
-- Name: following_order_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.following_order_id_seq OWNED BY public.following_order.id;


--
-- Name: posts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.posts (
                              id integer NOT NULL,
                              title character varying NOT NULL,
                              body text NOT NULL,
                              published boolean DEFAULT false NOT NULL
);


ALTER TABLE public.posts OWNER TO postgres;

--
-- Name: posts_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.posts_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.posts_id_seq OWNER TO postgres;

--
-- Name: posts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.posts_id_seq OWNED BY public.posts.id;


--
-- Name: tg_user; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tg_user (
                                id bigint NOT NULL,
                                deleted boolean DEFAULT false NOT NULL,
                                create_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
                                update_time timestamp with time zone,
                                address character varying NOT NULL,
                                private_key character varying,
                                fee_staged numeric,
                                fee_received numeric,
                                parent character varying
);


ALTER TABLE public.tg_user OWNER TO postgres;

--
-- Name: tg_user_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.tg_user_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.tg_user_id_seq OWNER TO postgres;

--
-- Name: tg_user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.tg_user_id_seq OWNED BY public.tg_user.id;


--
-- Name: trading_order; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.trading_order (
                                      id bigint NOT NULL,
                                      deleted boolean DEFAULT false NOT NULL,
                                      create_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
                                      update_time timestamp with time zone,
                                      sell_or_buy character varying NOT NULL,
                                      target_token character varying NOT NULL,
                                      from_token character varying NOT NULL,
                                      trading_uer bigint NOT NULL,
                                      boost_mode boolean NOT NULL,
                                      mev_protected boolean NOT NULL,
                                      priority_fee numeric,
                                      is_succeed boolean,
                                      tx_hash character varying,
                                      tx_receipt jsonb,
                                      target_amount numeric,
                                      from_token_amount numeric,
                                      order_type character varying,
                                      pending_target_price numeric,
                                      expire_at timestamp with time zone,
                                      fee numeric
);


ALTER TABLE public.trading_order OWNER TO postgres;

--
-- Name: COLUMN trading_order.sell_or_buy; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.trading_order.sell_or_buy IS 'sell|buy';


--
-- Name: COLUMN trading_order.order_type; Type: COMMENT; Schema: public; Owner: postgres
--

COMMENT ON COLUMN public.trading_order.order_type IS 'trading|pending|following';


--
-- Name: trading_order_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.trading_order_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.trading_order_id_seq OWNER TO postgres;

--
-- Name: trading_order_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.trading_order_id_seq OWNED BY public.trading_order.id;


--
-- Name: following_order id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.following_order ALTER COLUMN id SET DEFAULT nextval('public.following_order_id_seq'::regclass);


--
-- Name: posts id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.posts ALTER COLUMN id SET DEFAULT nextval('public.posts_id_seq'::regclass);


--
-- Name: tg_user id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tg_user ALTER COLUMN id SET DEFAULT nextval('public.tg_user_id_seq'::regclass);


--
-- Name: trading_order id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.trading_order ALTER COLUMN id SET DEFAULT nextval('public.trading_order_id_seq'::regclass);





--
-- Data for Name: following_order; Type: TABLE DATA; Schema: public; Owner: postgres
--



--
-- Data for Name: posts; Type: TABLE DATA; Schema: public; Owner: postgres
--



--
-- Data for Name: tg_user; Type: TABLE DATA; Schema: public; Owner: postgres
--



--
-- Data for Name: trading_order; Type: TABLE DATA; Schema: public; Owner: postgres
--



--
-- Name: following_order_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.following_order_id_seq', 1, false);


--
-- Name: posts_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.posts_id_seq', 1, false);


--
-- Name: tg_user_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.tg_user_id_seq', 1, false);


--
-- Name: trading_order_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.trading_order_id_seq', 1, false);


--


--
-- Name: following_order following_order_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.following_order
    ADD CONSTRAINT following_order_pk PRIMARY KEY (id);


--
-- Name: posts posts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_pkey PRIMARY KEY (id);


--
-- Name: tg_user tg_user_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tg_user
    ADD CONSTRAINT tg_user_pk PRIMARY KEY (id);


--
-- Name: trading_order trading_order_pk; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.trading_order
    ADD CONSTRAINT trading_order_pk PRIMARY KEY (id);


--
-- Name: following_order update_time; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER update_time BEFORE UPDATE ON public.following_order FOR EACH STATEMENT EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: tg_user update_time; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER update_time BEFORE UPDATE ON public.tg_user FOR EACH STATEMENT EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: trading_order update_time; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER update_time BEFORE UPDATE ON public.trading_order FOR EACH STATEMENT EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- PostgreSQL database dump complete
--



