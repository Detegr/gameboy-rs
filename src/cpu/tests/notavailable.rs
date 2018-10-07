use cpu::tests::*;
use std::panic;

#[test]
fn test_na() {
    let na_opcodes = [
        0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD,
    ];
    for na_opcode in na_opcodes.into_iter() {
        let result = panic::catch_unwind(|| {
            let (mut cpu, mut ram) = init(None);
            test(&mut cpu, &mut ram, 0, opcode(*na_opcode));
        });
        if let Err(e) = result {
            if let Some(errormsg) = e.downcast_ref::<&'static str>() {
                assert_eq!(*errormsg, "Instruction not available. This is a bug.");
            } else {
                assert!(false, "Could not interpret error message as &str");
            }
        } else {
            assert!(
                false,
                format!("Opcode {:2X} shouldn't have been implemented", na_opcode)
            );
        }
    }
}
