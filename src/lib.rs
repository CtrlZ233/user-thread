#![feature(naked_functions)]
mod user_thread;


#[cfg(test)]
mod tests {
    use crate::user_thread::*;

    #[test]
    fn it_works() {
        let mut runtime = Runtime::new(); 
        runtime.init();

        runtime.spawn(coroutine1, &runtime as *const Runtime as u64);
        runtime.spawn(coroutine2, &runtime as *const Runtime as u64);
        runtime.run();
    }

    fn coroutine1(rt: u64) {

        let runtime = unsafe {
            &mut *(rt as *mut Runtime)
        };
        println!("THREAD 1 STARTING");
        let id = 1;
        for i in 0..12 {
            println!("thread: {} counter: {}", id, i);
            // yield_thread(runtime);
        }
        
        println!("THREAD 1 FINISHED");
    }

    fn coroutine2(rt: u64) {
        let runtime = unsafe {
            &mut *(rt as *mut Runtime)
        };

        println!("THREAD 2 STARTING");
        let id = 2;
        for i in 0..12 {
            println!("thread: {} counter: {}", id, i);
            // yield_thread(runtime);
        }
        println!("THREAD 2 FINISHED");
    }
}
