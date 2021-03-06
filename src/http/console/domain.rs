
use rocket::request::Form;
use rocket_contrib::json::JsonValue;

use crate::http::console::PagingParams;
use crate::http::user_session;
use crate::http::{session_user, Context};
use crate::models::domain::Domain;
use crate::models::user::UserFlag;
use crate::repo::{DomainRepo};
use crate::{conn, util};

#[derive(FromForm, Debug)]
pub struct DomainEntity {
    pub id: i32,
    pub domain: String,
    pub user_id: i32,
    pub notes: String,
    pub state: i16,
}

#[post("/domain/list", data = "<p>")]
pub fn domain_list(p: PagingParams, ctx: Context) -> JsonValue {
    let user_id: i32 = p
        .data
        .get("user_id")
        .expect("缺少参数:user_id")
        .parse()
        .unwrap_or(0);
    if !user_session::check_access(&ctx.req.cookies(), user_id) {
        return user_session::access_denied();
    }
    let begin = ((p.page - 1) * p.rows) as i64;
    let over = begin + p.rows as i64;
    let (total, rows) = DomainRepo::take_domains(&conn(), user_id, begin, over);
    json!({"total":total,"rows":rows})
}

#[post("/domain/get?<id>")]
pub fn get_domain(id: i32, ctx: Context) -> JsonValue {
    match DomainRepo::get(&conn(), id) {
        Some(u) => {
            if !user_session::check_access(&ctx.req.cookies(), u.user_id) {
                return user_session::access_denied();
            }
            json!(u)
        }
        None => json!({"err_msg":"域名不存在"}),
    }
}

#[post("/domain/save", data = "<entity>")]
pub fn save_domain(ctx: Context, entity: Form<DomainEntity>) -> JsonValue {
    let i = UserFlag::SuperUser as i16;
    let u = session_user(&ctx.req.cookies()).unwrap();
    if (u.flag & i != i) && u.id != entity.user_id {
        return json!({"code":1,"err_msg":"no such user"});
    }
    let conn = &conn();
    let mut domain: Domain;
    if entity.id > 0 {
        domain = DomainRepo::get(&conn, entity.id).expect("no such domain");
    } else {
        domain = Domain {
            id: 0,
            user_id: entity.user_id,
            hash: "".to_string(),
            domain: String::from(""),
            flag: 0,
            state: 0,
            notes: String::from(""),
            create_time: 0,
        }
    }
    domain.domain = entity.domain.clone();
    domain.state = entity.state;
    domain.notes = entity.notes.clone();
    match DomainRepo::save(&conn, &domain) {
        Ok(_) => json!({"code":0}),
        Err(err) => json!({"code":1,"err_msg":err.message()}),
    }
}

#[post("/domain/stat_js?<self_host>&<hash>")]
pub fn stat_js(self_host: i16, hash: String, ctx: Context) -> JsonValue {
    if let Some(d) = DomainRepo::get_domain(&conn(), hash.clone()) {
        if d.state == 0 {
            return json!({"code":1,"err_msg":"域名未启用，无法生成统计代码"});
        }
        let mut js_url;
        if self_host == 1 {
            js_url = String::from("http://");
            js_url.push_str(&d.domain);
        } else {
            js_url = util::self_pre_link(&ctx);
        }
        js_url.push_str("/static/domain_stat.js?");
        js_url.push_str(&hash);
        return json!({"code":0,"url":js_url});
    }
    json!({"code":1,"err_msg":"no such domain"})
}
