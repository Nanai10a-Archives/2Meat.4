fn main() {
    dotenv::dotenv().ok();

    let rt = {
        let worker_ids: Vec<u32> = Vec::new();
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .on_thread_start(|| {})
            .on_thread_stop(|| {})
            .thread_name_fn(move || {
                let id = {
                    let mut i = 0u32;
                    loop {
                        match worker_ids.iter().find(|num| **num == i) {
                            None => break i,
                            Some(_) => (),
                        };
                        i += 1;
                    }
                };

                format!("tokio_worker-{}", id)
            })
            .build()
            .unwrap()
    };

    rt.block_on(async_main());
}

async fn async_main() {}
