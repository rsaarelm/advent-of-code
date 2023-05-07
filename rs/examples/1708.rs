use aoc::prelude::*;

fn main() {
    let mut regs: HashMap<String, i32> = HashMap::default();
    let mut max = 0;
    for line in stdin_lines() {
        let [reg, op, n, _, test_reg, c, val]: [&str; 7] =
            line.split(' ').collect::<Vec<&str>>().try_into().unwrap();
        let mut n: i32 = n.parse().unwrap();
        let val: i32 = val.parse().unwrap();
        let test_reg = regs.entry(test_reg.to_string()).or_default();
        if cmp_fn(c)(*test_reg, val) {
            if op == "dec" {
                n = -n;
            }
            let reg = regs.entry(reg.to_string()).or_default();
            *reg += n;
            max = max.max(*reg);
        }
    }

    println!("{}", regs.values().max().unwrap());
    println!("{max}");
}
