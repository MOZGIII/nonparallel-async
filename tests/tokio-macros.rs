use tokio::sync::Mutex;

static MUT_A: Mutex<()> = Mutex::const_new(());

#[nonparallel_async::nonparallel_async(MUT_A)]
#[tokio::test]
async fn testmacro() {}
