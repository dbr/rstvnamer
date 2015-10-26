/// Turns "123" into 123
pub fn intify(instr: &str) -> i32{
    // TODO: Better error handling
    instr.to_owned().parse::<i32>().unwrap()
}
