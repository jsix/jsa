pub enum StatFromType {
    Query = 1,
    /// Detect by host
    Host = 2,
    /// Detect base location query
    /// Detect base referer url
    Referer = 3,
    /// Detect base user-agent
    UserAgent = 4,
}

/// Request from
pub struct StatFrom {
    /// name
    pub key: String,
    /// detect type
    pub detect_type: i16,
    /// keyword
    pub keyword: String,
}

impl StatFrom {
    pub fn new(key: &str, detect_type: i16, keyword: &str) -> Self {
        Self {
            key: key.to_owned(),
            detect_type,
            keyword: keyword.to_owned(),
        }
    }
}

lazy_static! {
    pub static ref INTERNAL_STAT_FROM_VEC: Vec<StatFrom> = vec![
        StatFrom::new("百度", StatFromType::Referer as i16, "www.baidu.com"),
        StatFrom::new(
            "百度推广",
            StatFromType::Referer as i16,
            "www.baidu.com/baidu.php"
        ),
        StatFrom::new(
            "百度-移动端",
            StatFromType::Referer as i16,
            "m.baidu.com/from="
        ),
        StatFrom::new(
            "百度推广-移动端",
            StatFromType::Referer as i16,
            "m.baidu.com/baidu.php"
        ),
        StatFrom::new(
            "百度知道",
            StatFromType::Referer as i16,
            "zhidao.baidu.com"
        ),
        StatFrom::new(
            "百度贴吧",
            StatFromType::Referer as i16,
            "tieba.baidu.com"
        ),
        StatFrom::new(
            "百度百科",
            StatFromType::Referer as i16,
            "baike.baidu.com"
        ),
        StatFrom::new("搜狗", StatFromType::Referer as i16, "www.sogou.com/link"),
        StatFrom::new(
            "搜狗-移动端",
            StatFromType::Referer as i16,
            "m.sogou.com/web"
        ),
        StatFrom::new(
            "搜狗推广",
            StatFromType::Referer as i16,
            "www.sogou.com/bill_cpc"
        ),
        StatFrom::new(
            "搜狗推广-移动端",
            StatFromType::Referer as i16,
            "m.sogou.com/bill_cpc"
        ),
        StatFrom::new(
            "搜狗百科",
            StatFromType::Referer as i16,
            "baike.sogou.com"
        ),
        StatFrom::new("360", StatFromType::Referer as i16, "so.com/link"),
        StatFrom::new(
            "360推广",
            StatFromType::Referer as i16,
            "so.com/search/eclk"
        ),
        StatFrom::new("神马搜索",StatFromType::Referer as i16,"sm.cn/adclick?url="),
        StatFrom::new("Bing", StatFromType::Referer as i16, ".bing.com"),
        StatFrom::new("Google", StatFromType::Referer as i16, ".google."),
        StatFrom::new("今日头条", StatFromType::Referer as i16, ".toutiao.com"),
        StatFrom::new("今日惠州", StatFromType::Referer as i16, ".huizhou.cn"),
        StatFrom::new("360推广", StatFromType::Host as i16, "mf.baolibao.cn"),
        StatFrom::new("UC",StatFromType::Host as i16,"daili.meizhuli365.com"),
    ];
}
