CREATE DATABASE loggaroo;

CREATE SCHEMA "loggaroo";

CREATE TABLE session
(
    session_id   UUID NOT NULL DEFAULT GEN_RANDOM_UUID(),
    last_refresh DATE NOT NULL DEFAULT CURRENT_DATE,

    CONSTRAINT pk_session PRIMARY KEY (session_id)
);

CREATE TABLE file
(
    session_id           UUID    NOT NULL,
    file_name            VARCHAR NOT NULL,
    hash                 VARCHAR NOT NULL,
    chunk_count          INT     NOT NULL,
    uploaded_chunk_count INT     NOT NULL DEFAULT 0,

    CONSTRAINT pk_file PRIMARY KEY (session_id, file_name),
    CONSTRAINT fk_session_id FOREIGN KEY (session_id) REFERENCES session (session_id) ON DELETE CASCADE
);

CREATE TYPE classification AS ENUM ('info', 'error');
CREATE TABLE log_entry
(
    session_id      UUID           NOT NULL,
    file_name       VARCHAR        NOT NULL,
    entry_nr        INT            NOT NULL,
    creation_date   DATE           NOT NULL,
    classification  CLASSIFICATION NOT NULL,
    service_ip      VARCHAR        NOT NULL,
    user_id         VARCHAR        NOT NULL,
    user_session_id VARCHAR        NOT NULL,
    java_class      VARCHAR        NOT NULL,
    content         VARCHAR        NOT NULL,

    CONSTRAINT pk_entry PRIMARY KEY (session_id, file_name, entry_nr),
    CONSTRAINT fk_session_id FOREIGN KEY (session_id, file_name) REFERENCES file (session_id, file_name) ON DELETE CASCADE
);