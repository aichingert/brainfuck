use std::io::Read;

type Stack = [u8; 30_000];

#[derive(Debug)]
enum Instr {
    Read,
    Write,
    Inc(u8),
    Dec(u8),
    PtrInc(usize),
    PtrDec(usize),
    Loop(Vec<Instr>),
}

impl Instr {
    fn parse(source: &[u8]) -> Vec<Self> {
        let mut loc = 0;
        let mut inst = Vec::new();

        while loc < source.len() {
            inst.push(match source[loc] {
                b'+' | b'-' | b'<' | b'>' => {
                    let mut j = loc;

                    while j < source.len() && source[j] == source[loc] {
                        j += 1;
                    }

                    let instr = match source[loc] {
                        b'+' => Self::Inc((j - loc) as u8),
                        b'-' => Self::Dec((j - loc) as u8),
                        b'>' => Self::PtrInc(j - loc),
                        b'<' => Self::PtrDec(j - loc),
                        _ => unreachable!(),
                    };
                    loc = j - 1;
                    instr
                }
                b'.' => Self::Write,
                b',' => Self::Read,
                b'[' => {
                    let mut end = loc + 1;
                    let mut cnt = 0;

                    while end < source.len() && (cnt != 0 || source[end] != b']') {
                        match source[end] {
                            b'[' => cnt += 1,
                            b']' => cnt -= 1,
                            _ => (),
                        }
                        end += 1;
                    }

                    if source.len() <= end {
                        panic!("ERROR: loop doesnt close start at: {} - ...", loc);
                    }

                    let instr = Self::Loop(Self::parse(&source[loc + 1..end]));
                    loc = end;
                    instr
                }
                _ => unreachable!(),
            });

            loc += 1;
        }

        inst
    }

    fn exec(source: &[Self], ptr: &mut usize, stk: &mut Stack) {
        for instr in source {
            match instr {
                Instr::PtrInc(n) => *ptr += *n,
                Instr::PtrDec(n) => *ptr -= *n,
                Instr::Inc(n) => stk[*ptr] = stk[*ptr].wrapping_add(*n),
                Instr::Dec(n) => stk[*ptr] = stk[*ptr].wrapping_sub(*n),
                Instr::Read => {
                    let mut buf = [0u8; 1];
                    std::io::stdin().read_exact(&mut buf).expect("unable to read stdin");
                    stk[*ptr] = buf[0];
                }
                Instr::Write => print!("{}", stk[*ptr] as char),
                Instr::Loop(program) => {
                    while stk[*ptr] != 0 {
                        Instr::exec(&program, ptr, stk);
                    }
                }
            }
        }
    }
}

fn main() {
    let inp = std::fs::read_to_string("examples/mandelbrot.bf").unwrap().trim().to_string();
    let inp = inp.bytes().filter(|c| matches!(c, b'+' | b'-' | b',' | b'.' | b'[' | b']' | b'>' | b'<')).collect::<Vec<_>>();

    let instr = Instr::parse(&inp);
    Instr::exec(&instr, &mut 15_000, &mut [0u8; 30_000]);
}
