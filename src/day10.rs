pub struct Day10;

struct Computer {
    register: isize,
    cycle: usize,
    instr_ct: usize,
}

impl Computer {
    fn new() -> Self {
        Self {
            register: 1,
            cycle: 0,
            instr_ct: 0,
        }
    }

    fn apply_instruction(&mut self, instruction: Instruction, mut callback: impl FnMut(&Computer)) {
        self.instr_ct += 1;

        match instruction {
            Instruction::Noop => {
                self.cycle += 1;
                callback(self);
            }
            Instruction::Addx(change) => {
                self.cycle += 1;
                callback(self);
                self.cycle += 1;
                callback(self);
                self.register += change;
            }
        }
    }
}

enum Instruction {
    Noop,
    Addx(isize),
}

fn instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|s| match &s[0..1] {
        "n" => Instruction::Noop,
        _ => Instruction::Addx(s[5..].parse().unwrap()),
    })
}

impl crate::runner::Day for Day10 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        let mut computer = Computer::new();
        let mut signal_strength = 0;

        for instruction in instructions(input) {
            computer.apply_instruction(instruction, |cpt| match cpt.cycle {
                20 => signal_strength += 20 * cpt.register,
                60 => signal_strength += 60 * cpt.register,
                100 => signal_strength += 100 * cpt.register,
                140 => signal_strength += 140 * cpt.register,
                180 => signal_strength += 180 * cpt.register,
                220 => signal_strength += 220 * cpt.register,
                _ => {}
            });
        }

        Ok(signal_strength.to_string())
    }
    fn expected_value_part_1() -> Option<&'static str> {
        Some("17840")
    }

    fn part_2(input: &str) -> anyhow::Result<String> {
        let mut computer = Computer::new();
        let mut pixels = [b'.'; 41 * 6];
        for i in 0..6 {
            pixels[i * 41 + 40] = b'\n'
        }

        for instruction in instructions(input) {
            computer.apply_instruction(instruction, |cpt| {
                let cycle_zero_indexed = cpt.cycle as isize - 1;
                let horizontal_pos = cycle_zero_indexed % 40;
                let vertical_pos = cycle_zero_indexed / 40;

                if cpt.register == horizontal_pos
                    || cpt.register + 1 == horizontal_pos
                    || cpt.register - 1 == horizontal_pos
                {
                    pixels[((vertical_pos * 41) + horizontal_pos) as usize] = b'#';
                }
            });
        }

        let result = String::from_utf8_lossy(&pixels);

        Ok(result.to_string())
    }
    fn expected_value_part_2() -> Option<&'static str> {
        Some(
            "####..##..#.....##..#..#.#....###...##..
#....#..#.#....#..#.#..#.#....#..#.#..#.
###..#..#.#....#....#..#.#....#..#.#....
#....####.#....#.##.#..#.#....###..#.##.
#....#..#.#....#..#.#..#.#....#....#..#.
####.#..#.####..###..##..####.#.....###.\n",
        )
    }
}
