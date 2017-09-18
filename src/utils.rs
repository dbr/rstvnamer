/// Turns "123" into 123
pub(crate) fn intify(instr: &str) -> u32 {
    // TODO: Better error handling
    instr.to_owned().parse::<u32>().unwrap()
}
