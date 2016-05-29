use cpu::tests::*;

#[test]
fn test_inc_rr() {
    macro_rules! test_inc_rr(
        ($r1:ident, $r2:ident, $r1_val:expr, $r2_val:expr, $expected_r1:expr, $expected_r2:expr, $cycles:expr, $func:expr) => {{
            let (mut cpu, mut ram) = init(None);
            cpu.$r1 = $r1_val;
            cpu.$r2 = $r2_val;
            test(&mut cpu, &mut ram, $cycles, $func);
            assert!(cpu.$r1 == $expected_r1, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), $expected_r1, cpu.$r1));
            assert!(cpu.$r2 == $expected_r2, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), $expected_r2, cpu.$r2));
        }}
    );
    test_inc_rr!(b, c, 0x0, 0x0, 0x0, 0x1, 8, opcode(0x03));
    test_inc_rr!(d, e, 0x2, 0xFF, 0x3, 0x0, 8, opcode(0x13));
    test_inc_rr!(h, l, 0xFF, 0xFF, 0x0, 0x0, 8, opcode(0x23));
}

#[test]
fn test_inc_sp() {
    let (mut cpu, mut ram) = init(None);
    cpu.sp = 0xFFFE;
    test(&mut cpu, &mut ram, 8, opcode(0x33));
    assert!(cpu.sp == 0xFFFF);
    test(&mut cpu, &mut ram, 8, opcode(0x33));
    assert!(cpu.sp == 0x0);
    test(&mut cpu, &mut ram, 8, opcode(0x33));
    assert!(cpu.sp == 0x1);
}

