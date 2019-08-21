use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, RwLock, mpsc};
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (file_1, file_2) = parse_args(&args);

    let a: Vec<Vec<f64>> = parse_matrix(&file_1);
    let b: Vec<Vec<f64>> = parse_matrix(&file_2);


    let output_matrix = multiply_matrices(&a, &b);
    println!("result is a {}x{} matrix:\n", a.len(), b[0].len());
    /*for x in output_matrix {
        println!("{:?}", x);
    }*/
}

fn parse_args(args: &[String]) -> (&str, &str) {

    let file_1 = &args[1];
    let file_2 = &args[2];

    (file_1, file_2)
}

fn parse_matrix(file_1: &str) -> (Vec<Vec<f64>>) {

    let mut matrix: Vec<Vec<f64>> = Vec::new();

    let f = BufReader::new(File::open(file_1).unwrap());

    for (i, line) in f.lines().enumerate() {
        matrix.push(Vec::new());
        for (_, num) in line.unwrap().split(char::is_whitespace).enumerate() {
            matrix[i].push(num.trim().parse().unwrap());
        }
    }

    matrix
}


fn multiply_matrices(matrix_a: &Vec<Vec<f64>>, matrix_b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let width: usize = matrix_b[0].len();
    let height: usize = matrix_a.len();
    let mut output = vec![vec![0.0; width]; height];
    let b = matrix_b.to_vec();
    let b_lock = Arc::new(RwLock::new(b));

    assert_eq!(matrix_a[0].len(), matrix_b.len());

    let (tx, rx) = mpsc::channel();

    for y in 0..height{
        let b_clone = Arc::clone(&b_lock);
        let tx1 = mpsc::Sender::clone(&tx);
        let line = matrix_a[y].to_vec();
        thread::spawn(move || {
            for i in 0..width {
                let mut intermediate = 0.0;
                let b = b_clone.read().unwrap();
                for x in 0..b.len() {
                    intermediate = intermediate + (line[x] * b[x][i])
                }
                tx1.send((intermediate, y, i)).unwrap();
            }
        });
    }
    drop(tx);
    for received in rx {
        let (val, x, y) = received;
        output[x][y] = val;
    }
    output
}
