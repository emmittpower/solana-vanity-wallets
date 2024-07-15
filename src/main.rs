use solana_sdk::signature::{Keypair, Signer};
use rand::{rngs::StdRng, SeedableRng};
use bs58;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::thread;
use num_format::{Locale, ToFormattedString};  

fn generate_keypair() -> Keypair {
    let mut rng = StdRng::from_entropy();
    Keypair::generate(&mut rng)
}

fn check_prefix(public_key: &str, prefixes: &[String], case_sensitive: bool) -> Option<String> {
    if case_sensitive {
        for prefix in prefixes {
            if public_key.starts_with(prefix) {
                return Some(prefix.clone());
            }
        }
    } else {
        let public_key = public_key.to_lowercase();
        for prefix in prefixes {
            if public_key.starts_with(&prefix.to_lowercase()) {
                return Some(prefix.clone());
            }
        }
    }
    None
}

fn main() {
    let mut prefixes: Vec<String>;
    println!("\n\x1b[1m\x1b[44m\x1b[30mChange thread count at src/main.rs:53\x1b[0m\n");
    loop {
        println!("\x1b[1mEnter desired prefix(es) separated by spaces:\x1b[0m\n\x1b[3m[prefix1 prefix2 etc.]\x1b[0m");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        prefixes = input.trim().split_whitespace() 
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty()) 
                        .collect();
        if !prefixes.is_empty() {
            break;
        }
        println!("\x1b[31m\x1b[1merror:\x1b[0m No prefixes provided. Please enter at least one prefix.\n");
    }
    println!("\x1b[1m\nCase sensitive search?:\x1b[0m\n\x1b[3m[y] / [n]\x1b[0m");
    

    let num_threads = 6; // adjust thread count based on your computer capacity - default is 6


    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let case_sensitive = input.trim().eq_ignore_ascii_case("y");
    println!("\n\x1b[1m\x1b[42m\x1b[30mSearching...\x1b[0m\n");
    let found = Arc::new(AtomicBool::new(false));
    let attempts = Arc::new(AtomicUsize::new(0));
    let found_prefixes = Arc::new(Mutex::new(vec![]));
    let printed_milestone = Arc::new(Mutex::new(0));
    let needs_clearing = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];

    for _ in 0..num_threads {
        let found = Arc::clone(&found);
        let attempts = Arc::clone(&attempts);
        let prefixes = prefixes.clone();
        let found_prefixes = Arc::clone(&found_prefixes);
        let printed_milestone = Arc::clone(&printed_milestone);
        let needs_clearing = Arc::clone(&needs_clearing);
        let handle = thread::spawn(move || {
            while !found.load(Ordering::Relaxed) {
                let keypair = generate_keypair();
                let public_key = bs58::encode(keypair.pubkey().to_bytes()).into_string();
                attempts.fetch_add(1, Ordering::Relaxed);
                if let Some(prefix) = check_prefix(&public_key, &prefixes, case_sensitive) {
                    {
                        let mut found_prefixes = found_prefixes.lock().unwrap();
                        if !found_prefixes.contains(&prefix) {
                            found_prefixes.push(prefix.clone());
                            if needs_clearing.load(Ordering::Relaxed) {
                                print!("\x1b[A\x1b[K");
                                needs_clearing.store(false, Ordering::Relaxed);
                            }
                            println!("\x1b[32m\x1b[1mFound\x1b[0m wallet \x1b[1m{}\x1b[0m after ~\x1b[35m\x1b[1m{}\x1b[0m attempts. Added to \x1b[36mkeys.txt\x1b[0m.", public_key, attempts.load(Ordering::Relaxed).to_formatted_string(&Locale::en));
                            let private_key = bs58::encode(keypair.to_bytes()).into_string();
                            let mut file = OpenOptions::new().append(true).create(true).open("keys.txt").unwrap();
                            writeln!(file, "Prefix: {}\nPublic Key: {}\nPrivate Key: {}\n", prefix, public_key, private_key).unwrap();
                        }
                    }
                    if found_prefixes.lock().unwrap().len() == prefixes.len() {
                        found.store(true, Ordering::Relaxed);
                    }
                }
                if attempts.load(Ordering::Relaxed) % 1000000 == 0 {
                    let mut printed_milestone = printed_milestone.lock().unwrap();
                    if *printed_milestone != attempts.load(Ordering::Relaxed) {
                        if needs_clearing.load(Ordering::Relaxed) {
                            print!("\x1b[A\x1b[K"); 
                        }
                        let rounded_attempts = ((attempts.load(Ordering::Relaxed) + 500_000) / 1_000_000) * 1_000_000;
                        println!("\x1b[3mChecked \x1b[35m\x1b[1m{}\x1b[0m \x1b[3maddresses...\x1b[0m", rounded_attempts.to_formatted_string(&Locale::en));
                        io::stdout().flush().unwrap();
                        *printed_milestone = attempts.load(Ordering::Relaxed);
                        needs_clearing.store(true, Ordering::Relaxed);
                    }
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    if found.load(Ordering::Relaxed) {
        println!("\n\x1b[32m\x1b[1mFound\x1b[0m all prefixes.");
    } else {
        println!("\x1b[31m\x1b[1mFailed to find all prefixes before exiting.\x1b[0m");
    }
}
