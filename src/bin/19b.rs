fn main() {
    let mut r0: i64 = 1;
    let mut r1: i64 = 0;
    // #ip 2
    let mut r2: i64 = 0;
    let mut r3: i64 = 0;
    let mut r4: i64 = 0;
    let mut r5: i64 = 0;
    let mut ip: usize = 0;
    loop {
        r2 = ip as i64;
        match ip {
            // addi 2 16 2
            0 => r2 += 16,
            // seti 1 0 4
            1 => {
                println!("{} {}", r1, r3);
                r4 = 1;
            }
            // seti 1 5 5
            2 => r5 = 1,
            // mulr 4 5 1
            3 => r1 = r4 * r5,
            // eqrr 1 3 1
            4 => r1 = (r1 == r3) as i64,
            // addr 1 2 2
            5 => r2 += r1,
            // addi 2 1 2
            6 => r2 += 1,
            // addr 4 0 0
            7 => r0 += r4,
            // addi 5 1 5
            8 => r5 += 1,
            // gtrr 5 3 1
            9 => r1 = (r5 > r3) as i64,
            // addr 2 1 2
            10 => r2 += r1,
            // seti 2 6 2
            11 => r2 = 2,
            // addi 4 1 4
            12 => r4 += 1,
            // gtrr 4 3 1
            13 => r1 = (r4 > r3) as i64,
            // addr 1 2 2
            14 => r2 += r1,
            // seti 1 7 2
            15 => r2 = 1,
            // mulr 2 2 2
            16 => r2 *= r2,
            // addi 3 2 3
            17 => {
                r3 += 2;
                println!("{} {}", r1, r3);
            }
            // mulr 3 3 3
            18 => {
                r3 *= r3;
                println!("{} {}", r1, r3);
            }
            // mulr 2 3 3
            19 => {
                r3 *= r2;
                println!("{} {}", r1, r3);
            }
            // muli 3 11 3
            20 => {
                r3 *= 11;
                println!("{} {}", r1, r3);
            }
            // addi 1 6 1
            21 => {
                r1 += 6;
                println!("{} {}", r1, r3);
            }
            // mulr 1 2 1
            22 => {
                r1 *= r2;
                println!("{} {}", r1, r3);
            }
            // addi 1 6 1
            23 => {
                r1 += 6;
                println!("{} {}", r1, r3);
            }
            // addr 3 1 3
            24 => {
                r3 += r1;
                println!("{} {}", r1, r3);
            }
            // addr 2 0 2
            25 => {
                r2 += r0;
                println!("{} {}", r1, r3);
            }
            // seti 0 3 2
            26 => {
                r2 = 0;
                println!("{} {}", r1, r3);
            }
            // setr 2 3 1
            27 => {
                r1 = r2;
                println!("{} {}", r1, r3);
            }
            // mulr 1 2 1
            28 => {
                r1 *= r2;
                println!("{} {}", r1, r3);
            }
            // addr 2 1 1
            29 => {
                r1 += r2;
                println!("{} {}", r1, r3);
            }
            // mulr 2 1 1
            30 => {
                r1 *= r2;
                println!("{} {}", r1, r3);
            }
            // muli 1 14 1
            31 => {
                r1 *= 14;
                println!("{} {}", r1, r3);
            }
            // mulr 1 2 1
            32 => {
                r1 *= r2;
                println!("{} {}", r1, r3);
            }
            // addr 3 1 3
            33 => {
                r3 += r1;
                println!("{} {}", r1, r3);
            }
            // seti 0 9 0
            34 => {
                r0 = 0;
                println!("{} {}", r1, r3);
            }
            // seti 0 5 2
            35 => r2 = 0,
            // end
            _ => break
        }
        ip = r2 as usize;
        ip += 1;
    }
    println!("{}", r0);
}
