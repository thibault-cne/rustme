use worker::*;

mod leetcode;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    let caches = Caches::new().await;

    Router::with_data(caches)
        .get_async("/leetcode", leetcode::leetcode_handler)
        .get("/", |_, _| Response::ok("up and running!"))
        .run(req, env)
        .await
}

struct Caches {
    caches: [Cache; 1],
}

impl Caches {
    async fn new() -> Self {
        let leetcode = Cache::open("leetcode".to_string()).await;

        Self { caches: [leetcode] }
    }

    fn leetcode(&mut self) -> &mut Cache {
        &mut self.caches[0]
    }
}
