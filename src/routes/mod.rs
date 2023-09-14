use worker::Router;

pub mod invite;
pub mod notifications;

pub fn route(router: Router<()>) -> Router<()> {
	invite::register(notifications::register(router))
}
