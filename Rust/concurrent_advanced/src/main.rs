#[cfg(test)]

mod tests{
    use std::thread;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[test]
    fn buggy_thread(){
        // 不直接 COUNT + 1 的原因是 Rust1.83 之后编译器认为多线程共享 static mut 就是 bug
        // 所以只能使用裸指针 (*mut u64) 操作
        static mut COUNT:u64 = 0;
        // 为什么这里要 as uszize
        let addr = core::ptr::addr_of_mut!(COUNT) as usize;

        // *mut T 类型是一个 原始指针类型
        unsafe { core::ptr::write(addr as *mut u64,0) };
        // unsafe { (addr as *mut u64).write(0) };

        let mut handles = vec![];

        for _ in 0..10{
            handles.push(thread::spawn(move || {
                let ptr = addr as *mut u64;
                for _ in 0..100_000{
                    unsafe {
                        let val = core::ptr::read_volatile(ptr);
                        core::ptr::write_volatile(ptr, val+1);
                    }
                }
            }));
        };
        for h in handles {
            h.join().unwrap();
        }
        let result = unsafe{ (addr as *mut u64).read()};
        println!("{}", result);
    }

    #[test]
    fn fixed_counter(){
        // 这里的 static mut 改为了 static， 因为 我们使用原子操作保证内部可变
        static COUNT: AtomicU64 = AtomicU64::new(0);
        // store 用于重置一个原子类型的变量
        // Ordering 是一个枚举，用于指定原子操作的顺序。
        // 一般有 5 种顺序：
        // Relaxed：只保证该操作本身是原子的，不阻止编译器/CPU 对其他内存操作进行重排。即：它前后的普通读写可能被移到它前面或后面。
        // Acquire：用于读操作（load）。禁止该操作之后的内存操作被重排到它之前（即：后续操作不能“提前”）。常用于获取锁或读标志位后读数据。
        // Release：用于写操作（store）。禁止该操作之前的内存操作被重排到它之后（即：前面的操作不能“延后”）。常用于写完数据后再释放锁或设标志位。
        // AcqRel：同时具有 Acquire + Release 语义，用于读-改-写操作（如 fetch_add, compare_exchange）。
        // SeqCst：在 AcqRel 基础上，所有线程看到的所有 SeqCst 操作有一个全局一致的顺序（最强一致性）。
        COUNT.store(0,Ordering::Relaxed);

        let mut handles = vec![];
        for _ in 0..10{
            handles.push(thread::spawn(||{
                for _ in 0..100_000{
                    // 这里使用 fetch_add 方法，这个是由 CPU 硬件去保证原子操作的实现
                    COUNT.fetch_add(1,Ordering::Relaxed);
                }
            }));
        }
        for h in handles{
            h.join().unwrap();
        }
        // load 用于获取当前值
        let result = COUNT.load(Ordering::Relaxed);
        assert_eq!(result, 1_000_000);
    }

    fn atomic_multiply(val: &AtomicU64, multiplier: u64) -> (u64, u64){
        let mut retries = 0;
        loop{
            let old  = val.load(Ordering::Relaxed);
            let new = old * multiplier;
            match val.compare_exchange(old, new, Ordering::Relaxed, Ordering::Relaxed){
                Ok(_) => return (old, new),
                Err(x) => {
                    retries += 1;
                    println!("CAS 失败，期望={}，实际={}，第{}次重试", old,x,retries);
                }
            }
        } 
    }

    #[test]
    fn test_cas_multiply(){
        static VAL: AtomicU64 = AtomicU64::new(2);

        atomic_multiply(&VAL, 3);
        assert_eq!(VAL.load(Ordering::Relaxed),6);

        atomic_multiply(&VAL, 5);
        assert_eq!(VAL.load(Ordering::Relaxed),30);
    }

    #[test]
    fn test_cas_mul_multithread(){
        static VAL: AtomicU64 = AtomicU64::new(2);
        let multiplier: u64 = 2;
        let mut handles = vec![];
        for _ in 0..100{
            handles.push(thread::spawn(move || {
                for _ in 0..30{
                    atomic_multiply(&VAL, multiplier);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        // 验证最终结果
        let final_val = VAL.load(Ordering::Relaxed);
        let expected = 2 * (multiplier.pow(3 * 10)); // 2 * (2^30) = 2^31
        assert_eq!(final_val, expected, "Expected {}, got {}", expected, final_val);
    }
}