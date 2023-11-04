extern crate capstone;

use std::{
    fs::File,
    io::{BufReader, Read},
};

use capstone::prelude::*;

fn main() {
    let mut cs = Capstone::new()
        .mips()
        .mode(arch::mips::ArchMode::Mips64)
        .endian(capstone::Endian::Big)
        .detail(true)
        .build()
        .expect("Failed to create Capstone object");

    cs.set_skipdata(true).unwrap();

    //let mut buf: Vec<u8> = Vec::new();

    for i in 0i64..0xFFFFFFFF {
        let bytes = i.to_le_bytes();
        let res = cs.disasm_all(&bytes, i.try_into().unwrap());
        println!("{}", res.unwrap()[0]);
    }

    //println!("we put {:x} bytes in the raw buffer", buf.len());

    /*let f = File::open("addiu_simpleboot.z64").expect("fuck!");
    let mut read = BufReader::new(f);
    read.read_to_end(&mut buf).expect("double fuck");
    println!("we put {:x} bytes in the raw buffer", buf.len());
    buf.drain(0..0x1000);*/

    //let res = cs.disasm_all(&buf, 0x1000).expect("fuck");

    /*println!("there are {:x} instructions in the instr buffer", res.len());

    for instr in res.as_ref() {
        println!("{}", instr);
    }*/

    /*let insns = cs
        .disasm_all(X86_CODE, 0x1000)
        .expect("Failed to disassemble");
    println!("Found {} instructions", insns.len());
    for i in insns.as_ref() {
        println!();
        println!("{}", i);

        let detail: InsnDetail = cs.insn_detail(&i).expect("Failed to get insn detail");
        let arch_detail: ArchDetail = detail.arch_detail();
        let ops = arch_detail.operands();

        let output: &[(&str, String)] = &[
            ("insn id:", format!("{:?}", i.id().0)),
            ("bytes:", format!("{:?}", i.bytes())),
            ("read regs:", reg_names(&cs, detail.regs_read())),
            ("write regs:", reg_names(&cs, detail.regs_write())),
            ("insn groups:", group_names(&cs, detail.groups())),
        ];

        for &(ref name, ref message) in output.iter() {
            println!("{:4}{:12} {}", "", name, message);
        }

        println!("{:4}operands: {}", "", ops.len());
        for op in ops {
            println!("{:8}{:?}", "", op);
        }
    }*/
}
