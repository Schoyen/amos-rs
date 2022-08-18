use amos_rs;
use approx;
use num::complex::Complex;

fn _get_hankel_data(path: &str) -> Vec<Vec<f64>> {
    let filename = std::path::Path::new(path);
    let contents = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();

    let mut split = contents.split("\n");
    // Skip header
    split.next();

    let mut data = Vec::new();

    for s in split {
        let lines = s.split_whitespace();
        let mut data_line = Vec::new();

        for l in lines {
            data_line.push(l.parse::<f64>().unwrap());
        }

        data.push(data_line);
    }

    data
}

fn get_hankel1_data() -> Vec<Vec<f64>> {
    _get_hankel_data("./tests/dat/zbesh1_test.txt")
}

fn get_hankel1e_data() -> Vec<Vec<f64>> {
    _get_hankel_data("./tests/dat/zbesh1_e_test.txt")
}

fn get_hankel2_data() -> Vec<Vec<f64>> {
    _get_hankel_data("./tests/dat/zbesh2_test.txt")
}

fn get_hankel2e_data() -> Vec<Vec<f64>> {
    _get_hankel_data("./tests/dat/zbesh2_e_test.txt")
}

fn test_hankel(func: i64, data: Vec<Vec<f64>>) {
    let mut cy_t = Vec::new();
    for dat in data {
        let nu_0 = dat[0];
        let j = dat[1];

        if j.abs() < 1e-12 {
            cy_t = Vec::new();
        }

        let z = Complex::new(dat[2], dat[3]);
        cy_t.push(Complex::new(dat[4], dat[5]));

        let cy = match func {
            1 => amos_rs::zbesh::hankel1(nu_0, z, ((j * j.signum()) as i32) + 1),
            2 => amos_rs::zbesh::hankel1e(nu_0, z, ((j * j.signum()) as i32) + 1),
            3 => amos_rs::zbesh::hankel2(nu_0, z, ((j * j.signum()) as i32) + 1),
            4 => amos_rs::zbesh::hankel2e(nu_0, z, ((j * j.signum()) as i32) + 1),
            _ => panic!("Invalid func"),
        };

        assert_eq!(cy_t.len(), cy.len());

        for i in 0..cy_t.len() {
            approx::assert_abs_diff_eq!(cy_t[i].re, cy[i].re, epsilon = 1e-12);
            approx::assert_abs_diff_eq!(cy_t[i].im, cy[i].im, epsilon = 1e-12);
        }
    }
}

#[test]
fn test_hankel1() {
    let func = 1;
    let data = get_hankel1_data();

    test_hankel(func, data);
}

#[test]
fn test_hankel1e() {
    let func = 2;
    let data = get_hankel1e_data();

    test_hankel(func, data);
}

#[test]
fn test_hankel2() {
    let func = 3;
    let data = get_hankel2_data();

    test_hankel(func, data);
}

#[test]
fn test_hankel2e() {
    let func = 4;
    let data = get_hankel2e_data();

    test_hankel(func, data);
}
