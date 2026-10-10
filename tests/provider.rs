use nddev_device_sync_adapter_io::{CancellationToken, NativeIo};
use nddev_device_sync_updater::Updater;

#[tokio::test]
#[ignore = "requires NDS_TEST_UPDATER pointing to the real installed updater"]
async fn observes_native_plan_and_last_run_without_applying_updates() {
    assert_eq!(
        std::env::var("NDS_ISOLATED_PROVIDER").as_deref(),
        Ok("1"),
        "run just provider-check"
    );
    let executable = std::env::var_os("NDS_TEST_UPDATER").expect("native updater required");
    let provider = Updater::new(
        NativeIo::new(1).unwrap().process_handle(),
        executable.into(),
    )
    .unwrap();
    let prior = provider.last_run(CancellationToken::new()).await.unwrap();
    let plan = provider.plan(CancellationToken::new()).await.unwrap();
    assert!(!plan.actions.is_empty());
    let after = provider.last_run(CancellationToken::new()).await.unwrap();
    assert_eq!(prior.started_at, after.started_at);
    assert_eq!(prior.finished_at, after.finished_at);
    assert_eq!(prior.catalog_sequence, after.catalog_sequence);
}
