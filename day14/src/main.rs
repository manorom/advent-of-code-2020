use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Mask {
        one_mask: u64,
        zero_mask: u64,
        x_mask: Vec<u8>
    },
    Mem {
        addr: u64,
        val: u64
    }
}

impl Instruction {
    fn parse_mask(mask: &str) -> (u64, u64, Vec<u8>) {
        let mut one_mask = 0;
        let mut zero_mask = u64::MAX;
        let mut x_mask = Vec::new();
        for (idx, c) in mask.chars().rev().enumerate() {
            if c == '0' {
                zero_mask ^= (1 << idx);
            } else if c == '1' {
                one_mask |= (1 << idx);
            } else if c == 'X' {
                x_mask.push(idx as u8);
            }
        }

        (one_mask, zero_mask, x_mask)
    }
    fn parse(line: &str) -> Instruction {
        let mut pieces = line.split(" ");
        let inst = pieces.next().unwrap();
        let _equals = pieces.next().unwrap();
        let val = pieces.next().unwrap();

        if inst == "mask" {
            let (one_mask, zero_mask, x_mask) = Self::parse_mask(val);
            Instruction::Mask {
                one_mask, zero_mask, x_mask
            }

        } else if &inst[0..3] == "mem" {
            let addr = inst[4..inst.len()-1].parse::<u64>().unwrap();
            let val = val.parse::<u64>().unwrap();
            Instruction::Mem {
                addr, val
            }
        } else {
            panic!("Unknown instruction in line {}", line);
        }
    }
}

struct Machine {
    mem: HashMap<u64, u64>,
    cur_one_mask: u64,
    cur_zero_mask: u64,
    cur_x_mask: Vec<u8>
}

impl Machine {
    fn new() -> Self {
        Machine {
            mem: HashMap::new(),
            cur_one_mask: 0,
            cur_zero_mask: 0,
            cur_x_mask: Vec::new()
        }
    }

    fn generate_mem_addrs_v2(&self, base_addr: u64) -> Vec<u64> {
        let mut base_addr = base_addr | self.cur_one_mask; 
        let mut addrs: Vec<u64> = Vec::new();
        fn add_addr(i: usize, base_addr: u64, cur_x_mask: &Vec<u8>) -> Vec<u64>{
            if i == cur_x_mask.len() {
                return vec![base_addr];
            }
            
            let mut local_vec = Vec::new();
            for v in add_addr(i + 1, base_addr, cur_x_mask).iter() {
                local_vec.push(v ^ 1 << cur_x_mask[i]);
                local_vec.push(*v);
            }
            return local_vec;

        }

        add_addr(0, base_addr, &self.cur_x_mask)
    }

    fn exec_inst_v2(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Mask {
                one_mask, zero_mask, x_mask,
            } => {
                self.cur_one_mask = *one_mask;
                self.cur_zero_mask = *zero_mask;
                self.cur_x_mask = x_mask.clone();
            },
            Instruction::Mem {
                addr, val
            } => {
                for a in self.generate_mem_addrs_v2(*addr).iter() {
                    self.mem.insert(*a, *val);
                }
                
            }
        }
    }

    fn exec_inst_v1(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Mask {
                one_mask, zero_mask, x_mask
            } => {
                self.cur_one_mask = *one_mask;
                self.cur_zero_mask = *zero_mask;
            },
            Instruction::Mem {
                addr,
                val,
            } => {
                let realval = (val & self.cur_zero_mask) | self.cur_one_mask;
                //println!("Setting mem[{}] = {} ({})", addr, realval, val);
                self.mem.insert(*addr, realval);
            }

        }
    }

    fn mem_sum(&self) -> u64 {
        self.mem.iter().map(|(k,v)| v).sum()
    }
}

fn main() {
    let input = include_str!("input.txt")
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| Instruction::parse(l))
        .collect::<Vec<_>>();
    let mut machine = Machine::new();
    for inst in input.iter() {
        machine.exec_inst_v1(inst);
    }

    println!("Memory sum v1: {}", machine.mem_sum());

    let mut machine = Machine::new();
    for inst in input.iter() {
        machine.exec_inst_v2(inst);
    }
    println!("Memory sum v2: {}", machine.mem_sum());

}
