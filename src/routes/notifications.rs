use crate::model::notification::Notification;
use worker::*;

pub fn register(router: Router<()>) -> Router<()> {
    router.post_async("/api/notifications/sab", |mut req, ctx| async move {
        let form = req.form_data().await?;

        let notif = Notification::from(form);

        let webhook_url = ctx.var("SAB_WEBHOOK_URL")?.to_string();
        let embed = notif.create_embed();

        let client = reqwest::Client::new();
        let res = client
            .post(webhook_url)
            .body(embed)
            .header("Content-Type", "application/json")
            .send().await;

        match res {
			Ok(_) => Response::ok("Okay!"),
			Err(e) => {
				console_log!("Error: {}", e);
				return Response::error("gurl idk", 500);
			}
		}
    })
}
