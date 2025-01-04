Test some opcodes:

using nasm:
```bash
nasm -o test.asm -o listings/part1/test
cargo run test
```

TODO:
- [ ] refactor parse to return a list of operations, i.e: [Mov(RegisterTarget::AX, RegisterTarget::SI)]
- [ ] improve lifetime-soup function signatures
- [ ] add clap for sane argument parsing
- [ ] 8086 simulator
- [ ] maybe introduce workspaces for each section of the course?
