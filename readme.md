8086 Decoder & Simulator:
https://edge.edx.org/c4x/BITSPilani/EEE231/asset/8086_family_Users_Manual_1_.pdf

Make some changes to `test.asm`

Compile it using [nasm](https://www.nasm.us/):
```bash
nasm -o test.asm -o listings/part1/test
```

Run your test
```
cargo run test
```

TODO:
- [ ] add clap for sane argument parsing
- [ ] 8086 simulator
- [ ] maybe introduce workspaces for each section of the course?
