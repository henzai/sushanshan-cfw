use handler::handle_su_shan_shan;
use worker::*;
mod handler;
mod utils;

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
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .get_async("/test", |_, ctx| async move {
            let deepl_api_key = ctx.secret("DEEPL_API_KEY")?.to_string();
            let translator = handler::Translator::new(&deepl_api_key);
            let text = translator.translate("apple").await?;
            Response::ok(text)
        })
        .post_async("/sushanshan", |req, ctx| async move {
            handle_su_shan_shan(req, ctx).await
        })
        .run(req, env)
        .await
}
