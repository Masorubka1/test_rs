use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;

fn split_computation<T: Send + 'static, R: Send + 'static>(
    input: Vec<T>,
    f: fn(T) -> R,
    _threshold: usize,
) -> Vec<R> {
    let input_len = input.len();
    let num_threads = 10; // Number of CPU cores
    let chunk_size = (input.len() + num_threads - 1) / num_threads;

    let (tx, rx) = channel();
    let mut vec_threads = Vec::with_capacity(num_threads);

    let mut tmp = Vec::with_capacity(chunk_size);
    let mut cnt = 0;
    for i in input {
        if input_len > _threshold && cnt == chunk_size {
            let local_f = f;
            let local_tx = tx.clone();
            let th = thread::spawn(move || {
                for i in tmp {
                    let res = local_f(i);
                    local_tx.send(res).unwrap();
                }
            });
            vec_threads.push(th);

            tmp = Vec::with_capacity(chunk_size);
            cnt = 0;
        }
        tmp.push(i);
        cnt += 1;
    }
    if tmp.len() > 0 {
        let local_f = f;
        let local_tx = tx.clone();
        let th = thread::spawn(move || {
            for i in tmp {
                let res = local_f(i);
                local_tx.send(res).unwrap();
            }
        });
        vec_threads.push(th);
    }

    drop(tx);

    let mut results = Vec::with_capacity(input_len);
    for val in rx.iter() {
        results.push(val);
    }
    results
}

mod test {
    use super::*;

    fn sum(a: isize) -> isize {
        thread::sleep(std::time::Duration::from_millis(400));
        return a * a;
    }

    const INP: [isize; 6] = [1, 2, 3, 4, 5, 6];
    const OUT: [isize; 6] = [1, 4, 9, 16, 25, 36];

    #[test]
    fn test_2() {
        let start = Instant::now();
        let mut out = split_computation(INP.to_vec(), sum, 2);
        out.sort();
        let duration = start.elapsed();
        println!("time: {:?}", duration);

        assert_eq!(out, OUT);
    }

    #[test]
    fn test_5() {
        let start = Instant::now();
        let mut out = split_computation(INP.to_vec(), sum, 6 - 1);
        out.sort();
        let duration = start.elapsed();
        println!("time: {:?}", duration);

        assert_eq!(out, OUT);
    }

    #[test]
    fn test_10() {
        let start = Instant::now();
        let mut out = split_computation(INP.to_vec(), sum, 10);
        out.sort();
        let duration = start.elapsed();
        println!("time: {:?}", duration);

        assert_eq!(out, OUT);
    }
}

/*fn main() {
    fn sum(a: isize) -> isize {
        thread::sleep(std::time::Duration::from_millis(400));
        return a * a
    }

    const INP: [isize; 6] = [1, 2, 3, 4, 5, 6];
    const OUT: [isize; 6] = [1, 4, 9, 16, 25, 36];

    let start = Instant::now();
    let out = split_computation(INP.to_vec(),  sum,  2);
    let duration = start.elapsed();
    println!("time: {:?}", duration);
}*/
