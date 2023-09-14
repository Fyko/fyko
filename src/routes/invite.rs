//! API route for generating single-use invites to my Discord server.  
//! This is to prevent spam and or abuse of the invite link.  
//! We'll post the user's location, ip, etc. to a private staff channel.  

use std::str::FromStr;
use twilight_http::Client;
use twilight_model::id::{Id, marker::{ChannelMarker, WebhookMarker}};
use worker::*;

// create an export a Router that we can import in lib.rs
pub fn register(router: Router<()>) -> Router<()> {
    router
        .get_async("/api/discord/invite", |req, ctx| async move {
            // 1. create the invite code
			let discord_token = ctx.var("DISCORD_TOKEN")?.to_string();
			let invite_channel = ctx.var("DISCORD_INVITE_CHANNEL_ID")?.to_string();
			let invite_channel = Id::<ChannelMarker>::from_str(&invite_channel).unwrap();

			let client = Client::new(discord_token);
			let res = client.create_invite(invite_channel)
				.max_uses(1).map_err(|_| return Response::error("Failed to create invite", 500)).unwrap()
				.max_age(30).map_err(|_| return Response::error("Failed to create invite", 500)).unwrap()
				.unique(true)
				.await.map_err(|_| return Response::error("Failed to create invite", 500)).unwrap()
				.model().await.map_err(|_| return Response::error("Failed to create invite", 500)).unwrap();

			// 2. send the user's ip, location, etc. to a discord webhook
			let webhook_id = ctx.var("DISCORD_WEBHOOK_ID")?.to_string();
			let webhook_id = Id::<WebhookMarker>::from_str(&webhook_id).unwrap();
			let webhook_token = ctx.var("DISCORD_WEBHOOK_TOKEN")?.to_string();

			let cf = req.cf();
			let content: Vec<String> = vec![
				format!("Location: [{}, {}]({}) (`{}`)",
					cf.city().unwrap_or_else(|| "Unknow City".to_string()),
					cf.region().unwrap_or_else(|| "Unknown Region".to_string()),
					cf.coordinates().map(|(lat, long)| format!("https://www.google.com/maps/place/{lat}/{long}")).unwrap_or_else(|| "None".to_string()),
					cf.colo()
				),
				format!("User Agent: `{:?}`", req.headers().get("user-agent")),
				format!("Invite created: `{}`", res.code),
			];
			let status = client
				.execute_webhook(webhook_id, &webhook_token)
				.content(&content.join("\n")).map_err(|_| return Response::error("Failed to create invite", 500)).unwrap()
				.await.map_err(|_| return Response::error("Failed to create invite", 500)).unwrap()
				.status();
			
			console_log!("webhook status: {}", status);

            return Response::from_bytes(res.code.into_bytes());
        })
}
