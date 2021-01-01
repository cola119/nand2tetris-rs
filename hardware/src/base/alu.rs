#![allow(dead_code)]
use crate::base::arithmetic::*;
use crate::base::logic::bit::O;
use crate::base::logic::*;

// -> (f(x,y), zr, ng)
pub fn alu(
    x: Word,
    y: Word,
    zx: bit,
    nx: bit,
    zy: bit,
    ny: bit,
    f: bit,
    no: bit,
) -> (Word, bit, bit) {
    let all0 = Word::new([O; 16]);
    let x1 = mux16(x, all0, zx);
    let x2 = mux16(x1, not16(x1), nx);
    let y1 = mux16(y, all0, zy);
    let y2 = mux16(y1, not16(y1), ny);
    let out1 = mux16(and16(x2, y2), add16(x2, y2), f);
    let out2 = mux16(out1, not16(out1), no);
    let zr = not(or(
        or8way([
            out2[0], out2[1], out2[2], out2[3], out2[4], out2[5], out2[6], out2[7],
        ]),
        or8way([
            out2[8], out2[9], out2[10], out2[11], out2[12], out2[13], out2[14], out2[15],
        ]),
    ));
    let ng = out2[0];
    (out2, zr, ng)
}

#[cfg(test)]
mod test {
    use super::*;
    use bit::I;

    #[test]
    fn for_alu() {
        assert_eq!(
            alu(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word::new([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                I,
                O,
                I,
                O,
                I,
                O
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                I,
                O
            )
        );
        assert_eq!(
            alu(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word::new([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                I,
                I,
                I,
                I,
                I,
                I
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]),
                O,
                O
            )
        );
        assert_eq!(
            alu(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word::new([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                O,
                I,
                O,
                O,
                I,
                I
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]),
                O,
                O
            )
        );
        assert_eq!(
            alu(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                I,
                O,
                I,
                O,
                I,
                O
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                I,
                O
            )
        );
        assert_eq!(
            alu(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                O,
                O,
                I,
                I,
                O,
                O
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
                O,
                O
            )
        );
        assert_eq!(
            alu(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                O,
                O,
                I,
                I,
                I,
                I
            ),
            (
                Word::new([I, I, I, I, I, I, I, I, I, I, I, O, I, I, I, I]),
                O,
                I
            )
        );
        assert_eq!(
            alu(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                O,
                O,
                I,
                I,
                I,
                O
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, O]),
                O,
                O
            )
        );
        // f(x,y)=y
        assert_eq!(
            alu(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                I,
                I,
                O,
                O,
                O,
                O
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                O,
                O
            )
        );
    }
}
