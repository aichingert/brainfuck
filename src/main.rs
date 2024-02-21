use std::io::Read;

fn process(source: &Vec<char>, ptr: &mut usize, stk: &mut [u8; 30_000]) {
    let mut loc = 0usize;

    while loc < source.len() {
        match source[loc] {
            '<' => *ptr+=1,
            '>' => *ptr-=1,
            '+' => stk[*ptr]+=1,
            '-' => stk[*ptr]-=1,
            '.' => print!("{}", stk[*ptr] as char),
            ',' => {
                let mut buf = [0u8; 1];
                std::io::stdin().read_exact(&mut buf).expect("unable to read stdin");
                stk[*ptr] = buf[0];
            }
            '[' => {
                let (mut dst, mut lps) = (loc + 1, 0);
                let mut lp = Vec::new();

                while lps != 0 || source[dst] != ']' {
                    match source[dst] {
                        '[' => lps += 1,
                        ']' => lps -= 1,
                        _ => {},
                    }

                    lp.push(source[dst]);
                    dst += 1;
                }

                while stk[*ptr] != 0 {
                    process(&lp, ptr, stk);
                }

                loc += lp.len() + 1;
            }
            _ => {
                println!("{loc}: {}", source[loc]);
            }
        }

        loc += 1;
    }
}

fn main() {
    let inp = std::fs::read_to_string("examples/rot13.bf").unwrap().trim().to_string();
    let inp = inp.chars().filter(|c| matches!(c, '+' | '-' | ',' | '.' | '[' | ']' | '>' | '<')).collect::<Vec<_>>();

    let mut stk = [0u8; 30_000];
    let mut ptr = 15_000usize;

    process(&inp, &mut ptr, &mut stk);
}
