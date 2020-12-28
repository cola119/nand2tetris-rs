#![allow(dead_code, non_snake_case)]
use crate::{
    alu::alu,
    dff::Clock,
    dff::ClockState::{Tick, Tock},
    logic::bit::{I, O},
    logic::{and, bit, mux16, not, or, Word},
    pc::PC,
    register::Register,
};

pub struct CPU {
    pc: PC,
    d_register: Register,
    a_register: Register,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: PC::new(),
            a_register: Register::new(),
            d_register: Register::new(),
        }
    }

    // -> outM, writeM, addressM[15], pc[15]
    pub fn run(
        &mut self,
        clock_t: &Clock,
        in_m: Word,
        instruction: Word,
        reset: bit,
    ) -> (Word, bit, [bit; 15], [bit; 15]) {
        let clock_t_1 = match clock_t.state {
            Tick => {
                let mut c = Clock::new();
                c.next();
                c
            }
            Tock => Clock::new(),
        };
        let current_a_value = self.a_register.output(&clock_t_1);
        let current_d_value = self.d_register.output(&clock_t_1);
        let (i, a, cccccc, ddd, jjj) = CPU::decode(instruction);

        let (alu, zr, ng) = alu(
            current_d_value,
            mux16(current_a_value, in_m, a),
            cccccc[0],
            cccccc[1],
            cccccc[2],
            cccccc[3],
            cccccc[4],
            cccccc[5],
        );

        self.a_register.input(
            clock_t,
            mux16(instruction, alu, i),
            or(/* A命令 */ not(i), /* C命令 */ ddd[0]),
        );

        self.d_register.input(clock_t, alu, ddd[1]);

        let is_jump = or(
            or(and(jjj[0], ng), and(jjj[1], zr)),
            and(jjj[2], and(not(zr), not(ng))),
        );
        let pc_result = self
            .pc
            .run(clock_t, current_a_value, I, and(is_jump, i), reset);

        (
            alu,
            and(i, ddd[2]),
            [
                current_a_value[1],
                current_a_value[2],
                current_a_value[3],
                current_a_value[4],
                current_a_value[5],
                current_a_value[6],
                current_a_value[7],
                current_a_value[8],
                current_a_value[9],
                current_a_value[10],
                current_a_value[11],
                current_a_value[12],
                current_a_value[13],
                current_a_value[14],
                current_a_value[15],
            ],
            [
                pc_result[1],
                pc_result[2],
                pc_result[3],
                pc_result[4],
                pc_result[5],
                pc_result[6],
                pc_result[7],
                pc_result[8],
                pc_result[9],
                pc_result[10],
                pc_result[11],
                pc_result[12],
                pc_result[13],
                pc_result[14],
                pc_result[15],
            ],
        )
    }

    fn decode(word: Word) -> (bit, bit, [bit; 6], [bit; 3], [bit; 3]) {
        (
            word[0],
            word[3],
            [word[4], word[5], word[6], word[7], word[8], word[9]],
            [word[10], word[11], word[12]],
            [word[13], word[14], word[15]],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::io::prelude::*;

    #[test]
    fn for_cpu() {
        let mut clock = Clock::new();
        let mut cpu = CPU::new();

        let word0 = Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]);
        let word1 = Word::new([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]);

        // CLOCK: TICK
        let (outM, writeM, addressM, pc) = cpu.run(
            &clock,
            word0,
            /* A命令 addr: [O, I, I, O, O, O, O, O, O, I, I, I, O, O, I] */
            Word::new([O, O, I, I, O, O, O, O, O, O, I, I, I, O, O, I]),
            O,
        );
        assert_eq!(outM, word0);
        assert_eq!(writeM, O);
        assert_eq!(addressM, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]);
        assert_eq!(pc, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]);

        clock.next();

        // CLOCK: TOCK
        let (outM, writeM, addressM, pc) = cpu.run(
            &clock,
            word0,
            Word::new([O, O, I, I, O, O, O, O, O, O, I, I, I, O, O, I]),
            O,
        );
        assert_eq!(outM, word0);
        assert_eq!(writeM, O);
        assert_eq!(addressM, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]);
        assert_eq!(pc, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]);
        assert_eq!(
            cpu.a_register.output(&clock),
            Word::new([O, O, I, I, O, O, O, O, O, O, I, I, I, O, O, I])
        );

        clock.next();

        // CLOCK: TICK
        let (outM, writeM, addressM, pc) = cpu.run(
            &clock,
            word0,
            /* comp: A, dest: D -> D=A */
            Word::new([I, I, I, O, I, I, O, O, O, O, O, I, O, O, O, O]),
            O,
        );
        assert_eq!(
            outM,
            Word::new([O, O, I, I, O, O, O, O, O, O, I, I, I, O, O, I]) // Aレジスタの値
        );
        assert_eq!(writeM, O);
        assert_eq!(addressM, [O, I, I, O, O, O, O, O, O, I, I, I, O, O, I]);
        assert_eq!(pc, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]);
        assert_eq!(
            cpu.a_register.output(&clock),
            Word::new([O, O, I, I, O, O, O, O, O, O, I, I, I, O, O, I])
        );

        clock.next();

        // CLOCK: TOCK
        let (outM, writeM, addressM, pc) = cpu.run(
            &clock,
            word0,
            Word::new([I, I, I, O, I, I, O, O, O, O, O, I, O, O, O, O]),
            O,
        );
        assert_eq!(
            outM,
            Word::new([O, O, I, I, O, O, O, O, O, O, I, I, I, O, O, I])
        );
        assert_eq!(writeM, O);
        assert_eq!(addressM, [O, I, I, O, O, O, O, O, O, I, I, I, O, O, I]); // 12345
        assert_eq!(pc, [O, O, O, O, O, O, O, O, O, O, O, O, O, I, O]);

        clock.next();

        // CLOCK: TICK
        let (outM, writeM, addressM, pc) = cpu.run(
            &clock,
            word0,
            /* comp: A, dest: D -> D=A */
            Word::new([I, I, I, O, I, I, O, O, O, O, O, I, O, O, O, O]),
            O,
        );
        assert_eq!(
            outM,
            Word::new([O, O, I, I, O, O, O, O, O, O, I, I, I, O, O, I])
        );
        assert_eq!(writeM, O);
        assert_eq!(addressM, [O, I, I, O, O, O, O, O, O, I, I, I, O, O, I]);
        assert_eq!(pc, [O, O, O, O, O, O, O, O, O, O, O, O, O, I, O]);
        assert_eq!(
            cpu.a_register.output(&clock),
            Word::new([O, O, I, I, O, O, O, O, O, O, I, I, I, O, O, I])
        );

        clock.next();

        // CLOCK: TOCK
        let (outM, writeM, addressM, pc) = cpu.run(
            &clock,
            word0,
            Word::new([I, I, I, O, I, I, O, O, O, O, O, I, O, O, O, O]),
            O,
        );
        assert_eq!(
            outM,
            Word::new([O, O, I, I, O, O, O, O, O, O, I, I, I, O, O, I])
        );
        assert_eq!(writeM, O);
        assert_eq!(addressM, [O, I, I, O, O, O, O, O, O, I, I, I, O, O, I]); // 12345
        assert_eq!(pc, [O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]);

        clock.next();

        // CLOCK: TICK
        let (outM, writeM, addressM, pc) = cpu.run(
            &clock,
            word1,
            /* comp: D-M, dest: D -> D=D-M */
            /* D=12345, M=word1=-1 */
            Word::new([I, I, I, I, O, I, O, O, I, I, O, I, O, O, O, O]),
            O,
        );
        assert_eq!(
            outM,
            // 12346
            Word::new([O, O, I, I, O, O, O, O, O, O, I, I, I, O, I, O]) // D-M=12345-(-1)
        );
        assert_eq!(writeM, O);
        assert_eq!(addressM, [O, I, I, O, O, O, O, O, O, I, I, I, O, O, I]);
        assert_eq!(pc, [O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]);
        assert_eq!(
            cpu.a_register.output(&clock),
            Word::new([O, O, I, I, O, O, O, O, O, O, I, I, I, O, O, I])
        );

        clock.next();

        // CLOCK: TOCK
        let (outM, writeM, addressM, pc) = cpu.run(
            &clock,
            word0,
            Word::new([I, I, I, O, I, I, O, O, O, O, O, I, O, O, O, O]),
            O,
        );
        assert_eq!(
            outM,
            Word::new([O, O, I, I, O, O, O, O, O, O, I, I, I, O, O, I])
        );
        assert_eq!(writeM, O);
        assert_eq!(addressM, [O, I, I, O, O, O, O, O, O, I, I, I, O, O, I]);
        assert_eq!(pc, [O, O, O, O, O, O, O, O, O, O, O, O, I, O, O]);
    }
}
