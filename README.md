# Rost

Compiles the Rost language to NASM.

## Running

### Compile Rost program

`./compile.sh input.rost [-no-optimize, -no-comments, -sl {0,1,2,3,4}]`

The `-sl` flag determines the compilation level, where the outputs are the following:

|Level|Output|
|-|-|
|0|Lexed|
|1|Parsed|
|2|Compiled|
|3|CodeRows (Nasm abstraction)|
|4|No output, write nasm to `out.asm`|

### Run the nasm code

#### Build Docker file

`./build`

#### Run the NASM code

`./run.sh first-argument second-argument`

## Registers

Integer arguments: RDI/RSI/RDX/RCX/R8/R9

Float arguments: XMM0..XMM7
