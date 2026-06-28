//! Android entry point for the QR code generator app.

use android_activity::AndroidApp;
use winio::prelude::*;

use crate::model::MainModel;

/// Native entry point called by the Android system.
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let app = App::builder()
        .android_app(app)
        .build()
        .expect("cannot create app");
    app.spawn(|| async {
        _ = MainModel::run_until_event(()).await;
    })
}
