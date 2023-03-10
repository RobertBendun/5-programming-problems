pub fn all_expression_to_100() {
    let program = &mut [0 as usize; 8];

    loop {
        let value = program
            .iter()
            .enumerate()
            .fold(1 as i128, |value, (i, op)| {
                let i = (i + 2) as i128;
                [value + i, value - i, value * 10 + value.signum() * i][*op]
            });

        if value == 100 {
            println!(
                "{}",
                program
                    .iter()
                    .enumerate()
                    .fold(String::from("1"), |s, (i, op)| {
                        s + &([" + ", " - ", ""][*op].to_owned() + &(i + 2).to_string())
                    })
            );
        }

        *program.last_mut().unwrap() += 1;
        let mut c = 0;
        for op in program.iter_mut().rev() {
            *op += c;
            c = *op / 3;
            *op %= 3;
            if c == 0 {
                break;
            }
        }
        if c > 0 {
            break;
        }
    }
}
