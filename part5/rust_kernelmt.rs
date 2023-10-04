use kernel::prelude::*;
use kernel::sync::{CondVar, Mutex};
use kernel::task::Task;
const INIT_VAL: u64 = 1;
const W: usize = 10;
const N: usize = 100;
use mymem;

module! {
    type: TestModule,
    name: "rust_kernelmt",
    author: "Bienn",
    description: "Rust module that interacts with mymem memory driver",
    license: "GPL",
}

kernel::init_static_sync! {
    static COUNT: Mutex<usize> = 0;
    static COUNT_IS_ZERO: CondVar;
}

pub struct TestModule;

impl TestModule {
    fn initialize(&self) {
        let mut init_buffer = [0u8; 8];
        mymem::mymem_read(&mut init_buffer, 0);
        init_buffer.copy_from_slice(&INIT_VAL.to_ne_bytes());
        mymem::mymem_write(&init_buffer, 0);
    }

    fn worker() {
        pr_info!("Running from thread {}\n", Task::current().pid());
        let mut guard = COUNT.lock();
        for _ in 0..N {
            let mut outbuf = [0u8; 8];
            mymem::mymem_read(&mut outbuf, 0);
            let value = u64::from_ne_bytes(outbuf);
            let incremented = value + 1;
            let inbuf = incremented.to_ne_bytes();
            mymem::mymem_write(&inbuf, 0);
        }
        *guard -= 1;
        if *guard == 0 {
            COUNT_IS_ZERO.notify_all();
        }
    }

    fn run_tests(&self) {
        *COUNT.lock() = W;
        let final_value = INIT_VAL;
        for i in 0..W {
            Task::spawn(fmt!("test{i}"), Self::worker).unwrap();
        }

        let mut guard = COUNT.lock();
        while *guard != 0 {
            COUNT_IS_ZERO.wait(&mut guard);
        }

        let mut outbuf = [0u8; 8];
        mymem::mymem_read(&mut outbuf, 0);
        let final_value = u64::from_ne_bytes(outbuf);

        pr_info!("Final value: {}\n", final_value);
    }
}


impl kernel::Module for TestModule {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("rust_kernelmt module loaded\n");
        let test_module = TestModule;
        test_module.initialize();
        test_module.run_tests();
        Ok(test_module)
    }
}
