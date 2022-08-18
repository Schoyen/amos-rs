use amos_rs;
use approx;
use num;

fn _get_iv_data(path: &str) -> Vec<Vec<f64>> {
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
        let mut lines = s.split_whitespace();
        let mut data_line = Vec::new();

        for l in lines {
            data_line.push(l.parse::<f64>().unwrap());
        }

        data.push(data_line);
    }

    data
}

fn get_iv_data() -> Vec<Vec<f64>> {
    _get_iv_data("./tests/dat/zbesi_test.txt")
}

fn get_ive_data() -> Vec<Vec<f64>> {
    _get_iv_data("./tests/dat/zbesi_e_test.txt")
}

#[test]
fn test_iv() {
    let data = get_iv_data();

    let mut cy_t = Vec::new();
    for dat in data {
        let nu_0 = dat[0];
        let j = dat[1];

        if j.abs() < 1e-12 {
            cy_t = Vec::new();
        }

        let z = num::complex::Complex::new(dat[2], dat[3]);
        cy_t.push(num::complex::Complex::new(dat[4], dat[5]));

        let cy = amos_rs::zbesi::iv(nu_0, z, ((j * j.signum()) as i32) + 1);

        assert_eq!(cy_t.len(), cy.len());

        for i in 0..cy_t.len() {
            approx::assert_abs_diff_eq!(cy_t[i].re, cy[i].re, epsilon = 1e-12);
            approx::assert_abs_diff_eq!(cy_t[i].im, cy[i].im, epsilon = 1e-12);
        }
    }
}

#[test]
fn test_ive() {
    let data = get_ive_data();

    let mut cy_t = Vec::new();
    for dat in data {
        let nu_0 = dat[0];
        let j = dat[1];

        if j.abs() < 1e-12 {
            cy_t = Vec::new();
        }

        let z = num::complex::Complex::new(dat[2], dat[3]);
        cy_t.push(num::complex::Complex::new(dat[4], dat[5]));

        let cy = amos_rs::zbesi::ive(nu_0, z, ((j * j.signum()) as i32) + 1);

        assert_eq!(cy_t.len(), cy.len());

        for i in 0..cy_t.len() {
            approx::assert_abs_diff_eq!(cy_t[i].re, cy[i].re, epsilon = 1e-12);
            approx::assert_abs_diff_eq!(cy_t[i].im, cy[i].im, epsilon = 1e-12);
        }
    }
}
