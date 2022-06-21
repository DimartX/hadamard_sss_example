use hadamard_sss::HadamardSSS;
use hadamard_sss::scheme_traits::*;
use ndarray::Array2;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_matrix(filename: &str) -> Result<Array2<i32>, &'static str>{
    let mut f = BufReader::new(File::open(&filename).unwrap());

    let mut num_line = String::new();
    f.read_line(&mut num_line).unwrap();
    f = BufReader::new(File::open(&filename).unwrap());
    let n: usize = num_line.len() - 1;

    // preallocate the array and read the data into it
    let mut mtx = Array2::<i32>::default((n, n));
    for (i, line) in f.lines().enumerate() {
        for (j, ch) in line.unwrap().chars().enumerate() {
            if ch == '1' || ch == '+' {
                mtx[[i, j]] = 1;
            } else {
                mtx[[i, j]] = -1;
            }
        }
    }
    Ok(mtx)
}

fn simple_example() {
    let mtx = read_matrix("../matrices/had8.txt").expect("can't read matrix");
    println!("Matrix was read:");
    println!("{}", mtx);
    let scheme = HadamardSSS::from(&mtx).expect("can't create scheme");

    let secret: u32 = 314159265;
    println!("The initial secret is {}", secret);
    let parts = scheme.share(secret).expect("can't share that secret");
    println!("Parts:");
    for el in &parts {
        println!("({}, {})", el.number(), el.data());
    }

    println!("Trying to reconstruct by 3 random values:");
    let parts_rec = vec![parts[1], parts[3], parts[6]];
    let res_secret = scheme.reconstruct(parts_rec);
    match res_secret {
        Ok(res) => println!("Result is {}", res),
        Err(err_msg) => println!("can't reconstruct secret: {}", err_msg),
    }

    println!("Trying to reconstruct by 5 random values:");
    let parts_rec = vec![parts[1], parts[3], parts[4], parts[5], parts[6]];
    let res_secret = scheme.reconstruct(parts_rec);
    match res_secret {
        Ok(res) => println!("Result is {}", res),
        Err(err_msg) => println!("can't reconstruct secret: {}", err_msg),
    }
}

fn main() {
    simple_example();
}
