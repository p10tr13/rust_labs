use std::{collections::BTreeSet, time, hint::black_box, io::{self, Read, Write}};
use core::{num::NonZero};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;

fn main() {
    // Ex. 1
    let n = match NonZero::new(55) {
        Some(n) => n,
        None => {
            eprintln!("Zero in not allowed");
            return;
        }
    };
    let set = divisors(n);
    println!("divisors: {:?}", set);

    // Ex. 2
    //let v = vec![1,2,3,4,6,5,7,8,9,10]; // Uncomment to check panic
    let v = vec![1,2,3,4,5,6,7,8,9,10]; // Comment to check panic
    assert_sorted(&v);

    // Ex. 3
    let now = time::Instant::now();
    for i in 1..100 {
        if let Some(nz) = NonZero::new(i) {
            black_box(divisors(black_box(nz)));
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.6}", (elapsed.as_micros() as f64)/100000.0);

    // Ex. 5
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind {}", e);
            return;
        }
    };
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error in client handling: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
}

fn divisors(n: NonZero<u32>) -> BTreeSet<NonZero<u32>> {
    let mut tree = BTreeSet::<NonZero<u32>>::new();
    for i in 1..n.isqrt().get() {
        if n.get().is_multiple_of(i) {
            if let Some(v) = NonZero::new(i) {
                tree.insert(v);
            }
            if i * i != n.get() && let Some(v) = NonZero::new(n.get()/i) {
                tree.insert(v);
            }
        }
    }
    tree
}

fn assert_sorted(buf: &[i32]) {
    buf.windows(2).for_each(|p| {
        if p[0] > p[1] {
            panic!("{:?} > {:?}", p[0], p[1]);
        }
    })
}

// Ex. 4
fn bulk_write(stream: &mut TcpStream, buf: &[u8]) -> io::Result<()> {
    let mut written = 0;
    while written < buf.len() {
        match stream.write(&buf[written..])? {
            0 => return Err(io::Error::new(io::ErrorKind::WriteZero, "stream closed")),
            n => written += n,
        }
    }
    Ok(())
}

// Ex. 4
fn bulk_read(stream: &mut TcpStream, size: usize) -> io::Result<Vec<u8>> {
    let mut read = 0;
    let mut buf = vec![0u8; size];

    while read < buf.len() {
        match stream.read(&mut buf[read..])? {
            0 => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "stream closed")),
            n => {
                read += n;
                if buf[..read].contains(&b'\n') {
                    break;
                }
            },
        }
    }

    buf.truncate(read);
    Ok(buf)
}

// Ex. 7
fn handle_client (mut stream: TcpStream) -> io::Result<()> {
    println!("New connection {:?}", stream.peer_addr()?);

    let data = bulk_read(&mut stream, 100)?;
    if data.is_empty() {
        println!("No data");
        return Ok(());
    }

    let path_str = match String::from_utf8(data) {
        Ok(s) => s.trim().to_string(),
        Err(_) => {
            bulk_write(&mut stream,b"Bad path\n")?;
            return Ok(());
        }
    };

    let path = match PathBuf::from_str(&path_str) {
        Ok(p) => p,
        Err(_) => {
            bulk_write(&mut stream,b"Bad path\n")?;
            return Ok(());
        }
    };

    let entries = match fs::read_dir(&path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error while opening directory {:?}: {}", path, e);
            bulk_write(&mut stream, b"Bad dir\n")?;
            return Ok(());
        }
    };

    let mut listing = String::new();
    for entry in entries {
        match entry {
            Ok(e) => {
                if let Some(name) = e.file_name().to_str() {
                    listing.push_str(name);
                    listing.push('\n');
                }
            }
            Err(err) => eprintln!("Error while iterating through the catalog: {}", err),
        }
    }

    bulk_write(&mut stream, listing.as_bytes())?;
    println!("Contents of the directory sent {:?}", path);

    Ok(())
}