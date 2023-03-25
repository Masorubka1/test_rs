use std::thread;
use std::time::Instant;

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

        let mut vec_threads = Vec::with_capacity(num_threads);
        for chunk in chunks {
            let inp = chunk.to_vec();
            let local_f = f;
            let th = thread::spawn(move || {
                inp.into_iter().map(local_f).collect::<Vec<_>>()
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



mod test {
    use super::*;

    fn sum(a: isize) -> isize {
        thread::sleep(std::time::Duration::from_millis(400));
        return a * a
    }

    const INP: [isize; 6] = [1, 2, 3, 4, 5, 6];

    #[test]
    fn test_2() {
        let start = Instant::now();
        let out = split_computation(INP.to_vec(),  sum,  2);
        let duration = start.elapsed();
        println!("time: {:?}", duration);
        
        assert_eq!(out, [1, 4, 9, 16, 25, 36]);
    }

    #[test]
    fn test_5() {
        let start = Instant::now();
        let out = split_computation(INP.to_vec(),  sum,  6 - 1);
        let duration = start.elapsed();
        println!("time: {:?}", duration);

        assert_eq!(out, [1, 4, 9, 16, 25, 36]);
    }

    #[test]
    fn test_10() {
        let start = Instant::now();
        let out = split_computation(INP.to_vec(),  sum,  10);
        let duration = start.elapsed();
        println!("time: {:?}", duration);

        assert_eq!(out, [1, 4, 9, 16, 25, 36]);
    }
}