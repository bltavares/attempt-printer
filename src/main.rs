use async_std::{prelude::FutureExt, sync::RwLock, task};
use std::{ops::Deref, sync::Arc, time::Duration};

fn main() {
    println!("Hello, world!");

    env_logger::init();

    task::block_on(async {
        let lock = Arc::new(RwLock::new(true));

        {
            let lock = lock.clone();
            task::spawn(async move {
                loop {
                    task::sleep(Duration::from_secs(1))
                        .timeout(Duration::from_secs(5))
                        .await
                        .unwrap();
                    let x = lock.write().await.deref() == &true;
                    println!("awaken 1 {}", x);
                }
            });
        }

        {
            let lock = lock.clone();
            task::spawn(async move {
                loop {
                    task::sleep(Duration::from_secs(1))
                        .timeout(Duration::from_secs(5))
                        .await
                        .unwrap();
                    let x = lock.write().await.deref() == &true;
                    println!("awaken 2 {}", x);
                }
            });
        }
        task::spawn(async move {
            println!("finishin...");
            task::sleep(Duration::from_secs(10)).await;
            let x = lock.write().await.deref() == &true;
            println!("closing {}", x);
        })
        .await
    });

    println!("Bye, world!");
}
