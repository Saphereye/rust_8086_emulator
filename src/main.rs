mod architecture;

fn main() {
    let mut processor = architecture::Processor::default(Vec::from([0x40, 0x41, 0x43, 0x90]));
    println!("{:#?}", processor);
    processor.compile_byte_code_to_quasi_compiled();
    processor.execute_quasi_compiled();
    println!("{:#?}", processor);
}
