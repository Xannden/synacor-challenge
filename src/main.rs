use std::{collections::HashMap, io::Write};
use std::{collections::VecDeque, fs::File};

struct Input {
    buffer: VecDeque<u16>,
    interrupt: bool,
}

impl Input {
    fn new() -> Input {
        Input {
            buffer: VecDeque::new(),
            interrupt: false,
        }
    }

    fn next(&mut self) -> Option<u16> {
        if self.buffer.is_empty() {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();

            if buf.trim() == "debug" {
                self.interrupt = true;
                return None;
            }

            buf = buf.replace("\r\n", "\n");

            self.buffer
                .extend(buf.as_bytes().iter().map(|byte| *byte as u16));
        }

        Some(self.buffer.pop_front().unwrap())
    }
}

struct Synacor {
    regs: [u16; 8],
    mem: Box<[u16]>,
    stack: Vec<u16>,
    pc: usize,
    input: Input,
}

impl Synacor {
    fn new(program: Vec<u16>, input: Input) -> Synacor {
        let mut mem = vec![0; 32769].into_boxed_slice();

        for idx in 0..program.len() {
            mem[idx] = program[idx] as u16;
        }

        Synacor {
            regs: [0; 8],
            mem,
            stack: vec![],
            pc: 0,
            input,
        }
    }

    fn get(&self, addr: u16) -> u16 {
        match addr {
            0..=32767 => addr,
            32768..=32775 => self.regs[(addr - 32768) as usize],
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, val: u16) {
        match addr {
            32768..=32775 => self.regs[(addr - 32768) as usize] = val,
            _ => unreachable!(),
        }
    }

    fn read_arg(&mut self) -> u16 {
        let temp = self.read();
        self.get(temp)
    }

    fn read(&mut self) -> u16 {
        let temp = self.mem[self.pc];
        self.pc += 1;

        temp
    }

    fn step(&mut self) -> bool {
        let op = self.read();
        // println!("Op: {}", op);
        match op {
            //Halt
            0 => return false,
            //Set
            1 => {
                let a = self.read();
                let b = self.read_arg();

                self.set(a, b);
            }
            //Push
            2 => {
                let a = self.read_arg();

                self.stack.push(a);
            }
            //Pop
            3 => {
                let val = self.stack.pop().unwrap();

                let a = self.read();

                self.set(a, val);
            }
            //Eq
            4 => {
                let a = self.read();
                let b = self.read_arg();
                let c = self.read_arg();

                // println!("{}=={}", b, c);

                self.set(a, if b == c { 1 } else { 0 });
            }
            //Gt
            5 => {
                let a = self.read();
                let b = self.read_arg();
                let c = self.read_arg();

                self.set(a, if b > c { 1 } else { 0 });
            }
            //Jmp
            6 => {
                let a = self.read_arg();

                self.pc = a as usize;
            }
            //Jt
            7 => {
                let a = self.read_arg();
                let b = self.read_arg();

                if a != 0 {
                    self.pc = b as usize;
                }
            }
            //Jf
            8 => {
                let a = self.read_arg();
                let b = self.read_arg();

                if a == 0 {
                    self.pc = b as usize;
                }
            }
            //Add
            9 => {
                let a = self.read();

                let b = self.read_arg();
                let c = self.read_arg();

                let result = (b.wrapping_add(c)) % 32768;

                self.set(a, result);
            }
            //Mult
            10 => {
                let a = self.read();

                let b = self.read_arg();
                let c = self.read_arg();

                let result = (b.wrapping_mul(c)) % 32768;

                self.set(a, result);
            }
            //Mod
            11 => {
                let a = self.read();

                let b = self.read_arg();
                let c = self.read_arg();

                let result = b % c;

                self.set(a, result);
            }
            //And
            12 => {
                let a = self.read();

                let b = self.read_arg();
                let c = self.read_arg();

                let result = (b & c) % 32768;

                self.set(a, result);
            }
            //Or
            13 => {
                let a = self.read();

                let b = self.read_arg();
                let c = self.read_arg();

                let result = (b | c) % 32768;

                self.set(a, result);
            }
            //Not
            14 => {
                let a = self.read();

                let b = self.read_arg();

                let result = !b % 32768;

                self.set(a, result);
            }
            //Rmem
            15 => {
                let a = self.read();
                let b = self.read_arg();

                self.set(a, self.mem[b as usize]);
            }
            //Wmem
            16 => {
                let a = self.read_arg();
                let b = self.read_arg();

                self.mem[a as usize] = b;
            }
            //Call
            17 => {
                let a = self.read_arg();

                self.stack.push(self.pc as u16);

                self.pc = a as usize;
            }
            //Ret
            18 => {
                if self.stack.is_empty() {
                    return false;
                }

                self.pc = self.stack.pop().unwrap() as usize;
            }
            //Out
            19 => {
                let val = self.read();
                print!("{}", self.get(val) as u8 as char);
            }
            //In
            20 => {
                let a = self.read();

                let next = match self.input.next() {
                    Some(val) => val,
                    None => {
                        self.pc -= 2;
                        return true;
                    }
                };

                // println!("{:x}", next);

                self.set(a, next);
            }
            //Noop
            21 => (),
            _ => unreachable!(),
        }

        true
    }
}

fn main() {
    let program = std::fs::read("challenge.bin")
        .unwrap()
        .chunks(2)
        .map(|chunk| (chunk[0] as u16) | ((chunk[1] as u16) << 8))
        .collect();

    let mut input = Input::new();

    let save = std::fs::read_to_string("save.txt").unwrap();

    input
        .buffer
        .extend(save.as_bytes().iter().map(|byte| *byte as u16));

    let mut synacor = Synacor::new(program, input);

    // let mut file = File::create("trace.txt").unwrap();

    while synacor.step() {
        // print_instruction(&synacor, 0, &mut file);
        if synacor.input.interrupt {
            interface(&mut synacor);
            synacor.input.interrupt = false;
        }
    }
}

fn print_instruction(synacor: &Synacor, offset: usize, file: &mut impl Write) -> usize {
    let pc = synacor.pc + offset;
    let mem = &synacor.mem;
    let args = [mem[pc + 1], mem[pc + 2], mem[pc + 3]];
    write!(file, "{}: ", pc).unwrap();
    match mem[pc] {
        0 => {
            writeln!(file, "Halt").unwrap();
            1
        }
        1 => {
            writeln!(file, "Set {} {}", args[0], args[1]).unwrap();
            3
        }
        2 => {
            writeln!(file, "Push {}", args[0]).unwrap();
            2
        }
        3 => {
            writeln!(file, "Pop {}", args[0]).unwrap();
            2
        }
        4 => {
            writeln!(file, "Eq {} {} {}", args[0], args[1], args[2]).unwrap();
            4
        }
        5 => {
            writeln!(file, "Gt {} {} {}", args[0], args[1], args[2]).unwrap();
            4
        }
        6 => {
            writeln!(file, "Jmp {}", args[0]).unwrap();
            2
        }
        7 => {
            writeln!(file, "Jt {} {}", args[0], args[1]).unwrap();
            3
        }
        8 => {
            writeln!(file, "Jf {} {}", args[0], args[1]).unwrap();
            3
        }
        9 => {
            writeln!(file, "Add {} {} {}", args[0], args[1], args[2]).unwrap();
            4
        }
        10 => {
            writeln!(file, "Mult {} {} {}", args[0], args[1], args[2]).unwrap();
            4
        }
        11 => {
            writeln!(file, "Mod {} {} {}", args[0], args[1], args[2]).unwrap();
            4
        }
        12 => {
            writeln!(file, "And {} {} {}", args[0], args[1], args[2]).unwrap();
            4
        }
        13 => {
            writeln!(file, "Or {} {} {}", args[0], args[1], args[2]).unwrap();
            4
        }
        14 => {
            writeln!(file, "Not {} {}", args[0], args[1]).unwrap();
            3
        }
        15 => {
            writeln!(file, "Rmem {} {}", args[0], args[1]).unwrap();
            3
        }
        16 => {
            writeln!(file, "Wmem {} {}", args[0], args[1]).unwrap();
            3
        }
        17 => {
            writeln!(file, "Call {}", args[0]).unwrap();
            2
        }
        18 => {
            writeln!(file, "Ret").unwrap();
            1
        }
        19 => {
            writeln!(file, "Out \"{}\"", synacor.get(args[0]) as u8 as char).unwrap();
            2
        }
        20 => {
            writeln!(file, "In {}", args[0]).unwrap();
            2
        }
        21 => {
            writeln!(file, "Noop").unwrap();
            1
        }
        _ => {
            writeln!(file).unwrap();
            1
        }
    }
}

enum Command {
    Regs,
    SetReg(usize, u16),
    SetMem(usize, u16),
    GetMem(usize),
    Print(usize),
    Exit,
}

fn get_command() -> Command {
    let mut buf = String::new();

    loop {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();

        let mut args = buf.split(' ');

        let cmd = args.next().unwrap();
        match cmd.trim() {
            "exit" => return Command::Exit,
            "regs" => return Command::Regs,
            "setreg" => {
                return Command::SetReg(
                    args.next().unwrap().trim().parse().unwrap(),
                    args.next().unwrap().trim().parse().unwrap(),
                );
            }
            "setmem" => {
                return Command::SetMem(
                    args.next().unwrap().trim().parse().unwrap(),
                    args.next().unwrap().trim().parse().unwrap(),
                );
            }
            "getmem" => {
                return Command::GetMem(args.next().unwrap().trim().parse().unwrap());
            }
            "print" => return Command::Print(args.next().unwrap().trim().parse().unwrap()),
            _ => println!("Unknown command: {}", cmd.trim()),
        }
    }
}

fn interface(synacor: &mut Synacor) {
    println!("Entering Debugger");
    loop {
        match get_command() {
            Command::Regs => {
                println!("Regs: {:?}", synacor.regs);
            }
            Command::SetReg(reg, val) => synacor.regs[reg] = val,
            Command::Exit => break,
            Command::SetMem(addr, val) => {
                synacor.mem[addr] = val;
            }
            Command::Print(amount) => {
                let mut offset = 0;
                for _ in 0..amount {
                    offset += print_instruction(synacor, offset, &mut std::io::stdout());
                }
            }
            Command::GetMem(addr) => {
                println!("Mem[{}] = {}", addr, synacor.mem[addr]);
            }
        }
    }

    println!("Exiting Debugger");
}
