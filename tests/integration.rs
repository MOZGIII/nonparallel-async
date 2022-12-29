use std::sync::Arc;
use tokio::sync::Mutex;

use nonparallel_async::nonparallel_async;

static MUT_A: Mutex<()> = Mutex::const_new(());

const COUNT: usize = 1_000;

#[nonparallel_async(MUT_A)]
async fn append1(vec: Arc<Mutex<Vec<u32>>>) {
    for _ in 0..COUNT {
        vec.lock().await.push(1);
    }
}

#[nonparallel_async(MUT_A)]
async fn append2(vec: Arc<Mutex<Vec<u32>>>) {
    for _ in 0..COUNT {
        vec.lock().await.push(2);
    }
}

#[nonparallel_async(MUT_A)]
async fn append3(vec: Arc<Mutex<Vec<u32>>>) {
    for _ in 0..COUNT {
        vec.lock().await.push(3);
    }
}

#[tokio::test]
async fn it_works() {
    let vecarc = Arc::new(Mutex::new(Vec::new()));

    let v1 = vecarc.clone();
    let t1 = tokio::spawn(async move { append1(v1).await });
    let v2 = vecarc.clone();
    let t2 = tokio::spawn(async move { append2(v2).await });
    let v3 = vecarc.clone();
    let t3 = tokio::spawn(async move { append3(v3).await });
    t1.await.unwrap();
    t2.await.unwrap();
    t3.await.unwrap();

    // Get inner vec
    let vec: Vec<_> = Arc::try_unwrap(vecarc).unwrap().into_inner();

    // Validate vec size
    assert_eq!(
        vec.len(),
        COUNT * 3,
        "Vector does not contain 3*COUNT items"
    );

    // Split vec in three. Every slice should only contain either 1, 2 or 3.
    // (depending on which function was faster).
    let mut vec1 = vec;
    let mut vec2 = vec1.split_off(COUNT);
    let vec3 = vec2.split_off(COUNT);
    assert!(vec1.iter().all(|&val| val == vec1[0]), "vec1 is {:?}", vec1);
    assert!(vec2.iter().all(|&val| val == vec2[0]), "vec2 is {:?}", vec2);
    assert!(vec3.iter().all(|&val| val == vec3[0]), "vec3 is {:?}", vec3);
}
