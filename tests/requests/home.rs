use insta::assert_debug_snapshot;
use loco_rs::testing;
use nigerian_login::app::App;
use serial_test::serial;

// TODO: see how to dedup / extract this to app-local test utils
// not to framework, because that would require a runtime dep on insta
macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("home_request");
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn can_get_home() {
    configure_insta!();

    testing::request::<App, _, _>(|request, _ctx| async move {
        let notes = request.get("/api").await;

        assert_debug_snapshot!((notes.status_code(), notes.text()));
    })
    .await;
}
