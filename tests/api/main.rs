use aurelia::application::Application;

mod handler;

pub fn cfg_fn(app: Application) -> Application {
    app.with_auth_routes()
}
