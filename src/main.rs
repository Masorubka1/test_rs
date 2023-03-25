use std::thread;
use std::time::{Duration, Instant};

fn split_computation<T: Send + Sync + Clone + 'static, R: Send + 'static>(
    input: Vec<T>,
    f: fn(T) -> R,
    threshold: usize,
) -> Vec<R> {
    if input.len() <= threshold {
        // If input is short enough, compute f sequentially
        input.into_iter().map(f).collect()
    } else {
        // Otherwise, split input and compute f in parallel
        let num_threads = 10; // num_cpus::get(); // Number of CPU cores
        let chunk_size = (input.len() + num_threads - 1) / num_threads;
        let chunks = input.chunks(chunk_size);

        let mut vec_threads = Vec::new();
        for chunk in chunks {
            let inp = chunk.to_vec();
            let local_f = f;
            let th = thread::spawn(move || {
                let results = inp.into_iter().map(local_f).collect::<Vec<_>>();
                results
            });
            vec_threads.push(th);
        }

        let mut results = Vec::with_capacity(input.len());
        for th in vec_threads {
            let chunk_result = th.join().unwrap().into_iter();
            results.extend(chunk_result);
        }
        results
    }
}

fn sum(a: isize) -> isize {
    thread::sleep(std::time::Duration::from_millis(400));
    return a * a
}

fn main() {
    let inp = vec![1, 2, 3, 4, 5, 6];

    let start = Instant::now();
    let out = split_computation(inp.clone(),  sum,  2);
    println!("out: {:?}", out);
    let duration = start.elapsed();
    println!("time: {:?}", duration);

    let start = Instant::now();
    let out = split_computation(inp.clone(),  sum,  6 - 1);
    println!("out: {:?}", out);
    let duration = start.elapsed();
    println!("time: {:?}", duration);

    let start = Instant::now();
    let out = split_computation(inp.clone(),  sum,  10);
    println!("out: {:?}", out);
    let duration = start.elapsed();
    println!("time: {:?}", duration);
}