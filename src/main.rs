use rayon::prelude::*;


fn main()
{
    let start_of_program = std::time::Instant::now();
    let target_amount = 10_000_000;
    let upper_limit = ((target_amount as f32).log2() * target_amount as f32 * if target_amount < 10_000{1.28}else{1.2}) as usize;

    let start = std::time::Instant::now();
    
    let precomputed_prime_count = (target_amount as f32).sqrt() as usize;
    let mut primes_up_to_sqrt:Vec<u32> = Vec::with_capacity(precomputed_prime_count);
    
    find_primes(&mut primes_up_to_sqrt, precomputed_prime_count);
    
    let end = std::time::Instant::now();
    let duration = end - start;
    
    println!("Duration of precalculating primes: {}", duration.as_secs_f32());

    let mut buffer = Vec::with_capacity(upper_limit);
    buffer.resize(upper_limit, true);
    buffer[0] = false;
    buffer[1] = false;

    let start = std::time::Instant::now();
    
    run_sieve(&mut buffer, &primes_up_to_sqrt);
    
    let end = std::time::Instant::now();
    let duration = end - start;
    println!("Duration of sieve: {}", duration.as_secs_f32());

    let mut primes = Vec::new();
    for (i, &v) in buffer.iter().enumerate()
    {
        if v
        {
            primes.push(i as u64);
        }
    }

    let last_prime = primes[target_amount - 1];
    
    let end_of_program = std::time::Instant::now();
    let duration = end_of_program - start_of_program;
    println!("{target_amount}th prime: {}", last_prime);
    println!("duration of program: {}", duration.as_secs_f32());
}

fn find_primes(primes: &mut Vec<u32>, target_amount: usize)
{
    primes.clear();
    primes.resize(target_amount, 0);
    primes[..2].copy_from_slice(&[2,3]);

    for current_amount in 2..primes.len()
    {
        let mut next_prime = primes[current_amount - 1] + 2;

        let mut index = 0;
        while primes[index] * primes[index] <= next_prime
        {
            if next_prime % primes[index] == 0
            {
                next_prime += 2;
                index = 0;
            }
            index += 1;
        }

        primes[current_amount] = next_prime;
    }
}

fn run_sieve(buffer: &mut [bool], primes_up_to_sqrt: &[u32])
{
    const CHUNK_SIZE:usize = 100000;
    let whole_buf_len = buffer.len();
    buffer.par_chunks_mut(CHUNK_SIZE).enumerate().for_each(|(j, buf)|
        {
            let offset = j * CHUNK_SIZE;
            buf[0] = false;
            //for &prime in primes_up_to_sqrt
            for &prime in primes_up_to_sqrt
            {
                let divisible_offset = ((offset + prime as usize)/prime as usize) * prime as usize;
                let mut i = divisible_offset + if j == 0 {prime as usize} else {0};
                let limit = offset + buf.len() + if j == whole_buf_len {1} else {0};
                while i < limit
                {
                    buf[i-offset] = false;
                    i += prime as usize;
                }
            }
        });
}