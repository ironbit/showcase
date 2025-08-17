use {
    anyhow::Result,
    lvm::{core, mem, vm},
    std::fs::read_to_string,
};

fn main() -> Result<()> {
    // Load 'commands'
    let content = read_to_string("res/command-r32.json")?;
    let cmds: Vec<core::Cmd> = serde_json::from_str(&content)?;
    let commands = vm::Factory::new().with_codes(cmds)?.make();

    // Load 'program'
    let content = read_to_string("res/prog-num-sum.json")?;
    let codes: Vec<core::Code> = serde_json::from_str(&content)?;
    let program = vm::Program::new(codes);

    // Make 'processor'
    let stack = mem::Stack::new();
    let ctx = vm::Context::new(stack);
    let mut vm = vm::Processor::new(ctx, commands);

    // Process 'program'
    vm.execute(program)?;
    println!("{vm}");

    // Done
    Ok(())
}
