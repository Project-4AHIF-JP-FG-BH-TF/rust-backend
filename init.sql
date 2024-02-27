-- DROP EVERYTHING
DROP TABLE IF EXISTS loggaroo.log_entry;
DROP TABLE IF EXISTS loggaroo.file;
DROP TABLE IF EXISTS loggaroo.session;
DROP TYPE IF EXISTS classification;

DROP SCHEMA IF EXISTS loggaroo;

-- CREATE EVERYTHING
CREATE SCHEMA loggaroo;

CREATE TABLE loggaroo.session
(
    session_id   UUID      NOT NULL DEFAULT GEN_RANDOM_UUID(),
    last_refresh TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT pk_session PRIMARY KEY (session_id)
);

CREATE TABLE loggaroo.file
(
    session_id           UUID    NOT NULL,
    file_name            VARCHAR NOT NULL,
    hash                 VARCHAR NOT NULL,
    chunk_count          INT     NOT NULL,
    uploaded_chunk_count INT     NOT NULL DEFAULT 0,

    CONSTRAINT pk_file PRIMARY KEY (session_id, file_name),
    CONSTRAINT fk_session_id FOREIGN KEY (session_id) REFERENCES loggaroo.session (session_id) ON DELETE CASCADE
);

CREATE TABLE loggaroo.log_entry
(
    session_id      UUID      NOT NULL,
    file_name       VARCHAR   NOT NULL,
    entry_nr        INT       NOT NULL,
    creation_date   TIMESTAMP NOT NULL,
    classification  VARCHAR   NOT NULL CHECK ( classification IN ('info', 'warn') ),
    service_ip      VARCHAR,
    user_id         VARCHAR,
    user_session_id VARCHAR,
    java_class      VARCHAR   NOT NULL,
    content         VARCHAR   NOT NULL,
    sql_raw         VARCHAR,
    sql_data        VARCHAR,


    CONSTRAINT pk_entry PRIMARY KEY (session_id, file_name, entry_nr),
    CONSTRAINT fk_session_id FOREIGN KEY (session_id, file_name) REFERENCES loggaroo.file (session_id, file_name) ON DELETE CASCADE
);
