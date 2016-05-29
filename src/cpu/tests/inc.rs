use cpu::tests::*;

#[test]
fn test_inc_rr() {
    macro_rules! test_inc_rr(
        ($r1:ident, $r2:ident, $func:expr) => {{
            let (mut cpu, mut ram) = init(None);
            cpu.$r1 = 0x0;
            cpu.$r2 = 0x0;
            test(&mut cpu, &mut ram, 8, $func);
            assert!(cpu.$r1 == 0x00, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x00, cpu.$r1));
            assert!(cpu.$r2 == 0x01, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x01, cpu.$r2));

            cpu.$r1 = 0x2;
            cpu.$r2 = 0xFF;
            test(&mut cpu, &mut ram, 8, $func);
            assert!(cpu.$r1 == 0x3, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x3, cpu.$r1));
            assert!(cpu.$r2 == 0x0, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x0, cpu.$r2));

            cpu.$r1 = 0xFF;
            cpu.$r2 = 0xFF;
            test(&mut cpu, &mut ram, 8, $func);
            assert!(cpu.$r1 == 0x0, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x0, cpu.$r1));
            assert!(cpu.$r2 == 0x0, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x0, cpu.$r2));
        }}
    );
    test_inc_rr!(b, c, opcode(0x03));
    test_inc_rr!(d, e, opcode(0x13));
    test_inc_rr!(h, l, opcode(0x23));
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

