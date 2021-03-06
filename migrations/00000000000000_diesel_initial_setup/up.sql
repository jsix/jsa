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
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TABLE u_user (id  SERIAL NOT NULL, "user" varchar(20) NOT NULL, name varchar(20) NOT NULL, pwd varchar(40) NOT NULL, flag int2 NOT NULL, email varchar(50) NOT NULL, create_time int4 NOT NULL, PRIMARY KEY (id));
COMMENT ON TABLE u_user IS '用户';
COMMENT ON COLUMN u_user.id IS '编号';
COMMENT ON COLUMN u_user."user" IS '用户';
COMMENT ON COLUMN u_user.name IS '昵称';
COMMENT ON COLUMN u_user.pwd IS '密码';
COMMENT ON COLUMN u_user.flag IS '标志';
COMMENT ON COLUMN u_user.email IS '邮箱';
COMMENT ON COLUMN u_user.create_time IS '创建时间';
CREATE TABLE d_domain (id  SERIAL NOT NULL, user_id int4 NOT NULL, hash varchar(20) NOT NULL, domain varchar(40) NOT NULL, flag int2 NOT NULL, state int2 NOT NULL, notes varchar(255) NOT NULL, create_time int4 NOT NULL, PRIMARY KEY (id));
COMMENT ON TABLE d_domain IS '域名';
COMMENT ON COLUMN d_domain.id IS '编号';
COMMENT ON COLUMN d_domain.user_id IS '用户编号';
COMMENT ON COLUMN d_domain.hash IS 'hash值';
COMMENT ON COLUMN d_domain.domain IS '域名';
COMMENT ON COLUMN d_domain.flag IS '标志';
COMMENT ON COLUMN d_domain.state IS '状态';
COMMENT ON COLUMN d_domain.notes IS '备注';
COMMENT ON COLUMN d_domain.create_time IS '创建时间';

