COMMENT ON TABLE d_domain IS '域名';
COMMENT ON COLUMN d_domain.id IS '编号';
COMMENT ON COLUMN d_domain.user_id IS '用户编号';
COMMENT ON COLUMN d_domain.hash IS 'hash值';
COMMENT ON COLUMN d_domain.domain IS '域名';
COMMENT ON COLUMN d_domain.flag IS '标志';
COMMENT ON COLUMN d_domain.state IS '状态';
COMMENT ON COLUMN d_domain.notes IS '备注';
COMMENT ON COLUMN d_domain.create_time IS '创建时间';
CREATE TABLE u_user (id  SERIAL NOT NULL, "user" varchar(20) NOT NULL, name varchar(20) NOT NULL, pwd varchar(40) NOT NULL, flag int2 NOT NULL, email varchar(50) NOT NULL, create_time int4 NOT NULL, PRIMARY KEY (id));
COMMENT ON TABLE u_user IS '用户';
COMMENT ON COLUMN u_user.id IS '编号';
COMMENT ON COLUMN u_user."user" IS '用户';
COMMENT ON COLUMN u_user.name IS '昵称';
COMMENT ON COLUMN u_user.pwd IS '密码';
COMMENT ON COLUMN u_user.flag IS '标志';
COMMENT ON COLUMN u_user.email IS '邮箱';
COMMENT ON COLUMN u_user.create_time IS '创建时间';
