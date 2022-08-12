#[allow(non_upper_case_globals)]
// algorithm: https://www.emis.de/journals/GMN/yahoo_site_admin/assets/docs/1_GMN-8492-V28N2.190180001.pdf
fn solve_pells(n: i32) -> i128 {
    const sz: usize = 1000;
    let mut a: [i128; sz] = [0; sz];
    let mut b: [i128; sz] = [0; sz];
    let mut c: [i128; sz] = [0; sz];
    a[0] = 0;
    a[1] = 1;
    b[0] = 1;
    b[1] = 0;
    c[0] = n.into();
    c[1] = 1;
    let mut q_i: i128;
    let mut idx: i32 = -1;
    for i in 1..sz {
        if i >= 2 && 2 * a[i] % c[i] == 0 {
            idx = i as i32;
            break;
        }
        q_i = ((((n as i128 - c[i - 1] * c[i]) as f32).sqrt() + (n as f32).sqrt()) / (c[i] as f32))
            as i128;
        a[i + 1] = a[i] * q_i + a[i - 1];
        b[i + 1] = b[i] * q_i + b[i - 1];
        c[i + 1] = 2 * q_i * (((n as i128 - c[i] * c[i - 1]) as f32).sqrt()) as i128 + c[i - 1]
            - q_i * q_i * c[i];
        /* println!("{} {} {} {}", a[i], b[i], c[i], q_i);
        println!("n-c_i*c_(i-1)={}", (n - c[i - 1] * c[i]) as f32); */
    }
    // x=2*a_i^2/c_i + (-1)^(i+1)
    // y = 2*a_i*b_i/c_i
    let mut x = (2 * a[idx as usize] * a[idx as usize]) / c[idx as usize];
    let y = (2 * a[idx as usize] * b[idx as usize]) / c[idx as usize];
    // idx is i-1 in the original formula because
    // iteration starts from i=-1 there.
    if (idx) & 1 == 0 {
        x += 1;
    } else {
        x -= 1;
    }
    println!("for n: {}, x: {}, y: {}", n, x, y);
    return x;
}
fn calc(n: i32) -> i32 {
    let mut x = 1;
    let mut ans = 1;
    for i in 2..=n {
        if ((i as f32).sqrt().floor()).powi(2) as i32 == i {
            continue;
        }
        let temp = solve_pells(i);
        if temp > x {
            x = temp;
            ans = i;
        }
    }

    return ans;
}
fn main() {
    let x = calc(1000);
    println!("{}", x);
}
