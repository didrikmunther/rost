# Rost

Compiles the Rost language to NASM.

## Compile Rost program

`./compile.sh input.rost [-no-optimize, -no-comments, -sl {0,1,2,3,4}]`

The `-sl` flag determines the compilation level, where the outputs are the following:

| Level | Output                             |
| ----- | ---------------------------------- |
| 0     | Lexed                              |
| 1     | Parsed                             |
| 2     | Compiled                           |
| 3     | CodeRows (Nasm abstraction)        |
| 4     | No output, write nasm to `out.asm` |

## Run the nasm code

### Build Docker file

`./build`

### Run the NASM code

`./run.sh first-argument second-argument`

## Registers

Integer arguments: RDI/RSI/RDX/RCX/R8/R9

Float arguments: XMM0..XMM7

## IR representation project

I want to create a good IR representation which fulfills the following requirements:

- Should be able to transpile into Python, C, Assembly, and WebAssembly
- Should contain debug information to provide runtime error hints

### Examples

#### Easy example

This means we need to store global variables in the IR representation.

```rost
fn main() {
	let a = 1 + 2;
	let b = a + 3 * 4;

	printf("%i %i", a, b);
}
```

```irrepresentation
global_vars = ["%i %i\n"]
instructions = [
	Push(Int(1), Int),
	Push(Int(2), Int),
	IntAdd,
	Assign(Var(0), Int),
	Push(Int(3), Int),
	Push(Int(4), Int),
	IntMul,
	Assign(Var(1), Int),
	Push(Var(1), Int),
	Push(Var(0), Int),
	Push(GlobalVar(0), String),
	BuiltinFunction("printf", 3),
	Pop,
]
```

```python
import sys
def __builtin__printf(format, *args):
	sys.stdout.write(format % args)

stack = []

def __intrinsic__stack_add():
	_1 = stack.pop()
	_2 = stack.pop()
	stack.append(_1 + _2)

def __intrinsic__stack_mul():
	_1 = stack.pop()
	_2 = stack.pop()
	stack.append(_1 * _2)

def __intrinsic__stack_push(value):
	stack.append(value)

def __intrinsic__stack_pop():
	return stack.pop()

def __instrinsic__stack_callf(func, n_args):
	args = [stack.pop() for i in range(n_args)]
	stack.append(func(*args))

_global_vars = {
	'_g1': "%i %i\n",
}

def main():
	__intrinsic__stack_push(1)
	__intrinsic__stack_push(2)
	__intrinsic__stack_add()
	_0_a = __intrinsic__stack_pop()

	__intrinsic__stack_push(3)
	__intrinsic__stack_push(4)
	__intrinsic__stack_mul()
	_1_b = __intrinsic__stack_pop()

	__intrinsic__stack_push(_1_b)
	__intrinsic__stack_push(_0_a)
	__intrinsic__stack_push(_global_vars['_g1'])
	__intrinsic__stack_callf(__builtin__printf, 3)
	__intrinsic__stack_pop() # Discard the return value
```

```asm
.section .data
fmt_string:   .asciz  "%i %i\n"

	.section .text
	.global main
	.extern printf

main:
	# a = 1 + 2
	mov     $1, %eax        # Move the constant 1 into EAX
	add     $2, %eax        # Add 2 to EAX, EAX now holds a = 1 + 2

	# Store result of a in a temporary place on the stack
	mov     %eax, -4(%rsp)  # Store a at offset -4 from the stack pointer (SP)

	# b = a + 3 * 4
	mov     $3, %ebx        # Move 3 into EBX
	imul    $4, %ebx        # Multiply EBX by 4 (EBX = 3 * 4)
	add     %ebx, %eax      # Add the result to a (currently in EAX)

	# Store result of b in another temporary place on the stack
	mov     %eax, -8(%rsp)  # Store b at offset -8 from the stack pointer (SP)

	# Prepare the arguments for printf ("%i %i", a, b)
	mov     -4(%rsp), %esi  # Load a into ESI (second printf argument)
	mov     -8(%rsp), %edi  # Load b into EDI (third printf argument)
	lea     fmt_string(%rip), %rdi  # Load the address of the format string into RDI

	# Call printf
	call    printf

	# Exit the program
	mov     $0, %eax        # Return 0 from main
	ret
```

#### With debug information

This means we need to store the actual variable names, position, and code in the IR representation.

```rost
fn main() {
	let a = [1, 2, 3];

	printf("%i", a[3]); // Error: Index out of bounds
}
```

```python
import sys
def __builtin__printf(format, *args):
	sys.stdout.write(format % args)

_global_vars = {
	'_g1': [1, 2, 3],
	'_g2': "%i\n",
}

def main():
	_1 = _global_vars['_g1'];	# a
	_2 = 3;
	if _2 >= len(_1):
		raise Exception("Index out of bounds exception: 'a[3]' main.rost:4:19")
	_3 = _1[_2];	# b
	__builtin__printf(_global_vars['_g2'] % (_3))
```

#### With structs

This means we need to store the actual structure data in the IR representation.

```rost
struct Person {
	name: char[8],
	age: int,
}

fn main() {
	let p = Person {
		name: "John\0",
		age: 30,
	};

	printf("%s %i", p.name, p.age);
}
```

```python
import sys
def __builtin__printf(format, *args):
	sys.stdout.write(format % args)

_global_vars = {
	'_g1': "John\0",
	'_g2': "%s %i\n",
}

class _s1_Person:
	def __init__(self, _1_name, _2_age):
		self._1_name = _1_name
		self._2_age = _2_age

def main():
	_1_p = _s1_Person(_global_vars['_g1'], 30);
	__builtin__printf(_global_vars['_g3'] % (_1_p._1_name, _1_p._2_age))
```

#### With functions

```rost
fn add(a: int, b: int) -> int {
	return a + b;
}

fn main() {
	let a = add(1, 2);
	let b = add(a, 3);

	printf("%i %i", a, b);
}
```

```irrepresentation
global_vars = ["%i %i\n"]
functions = [
	BuiltInFunction("printf", Varargs(Args([String]), Any), Void),
	Function("add", [Int, Int], Int, [
		Assign(1, Param(0)),
		Add(1, Param(1)),
		Return(1),
	]),
]
instructions = [
	Assign(1, FunctionCall(0, [Int(1), Int(2)])),
	Assign(2, FunctionCall(0, [Var(1), Int(3)])),
	FunctionCall("printf", [GlobalVar(0), Var(1), Var(2)]),
]
```

```python
import sys
def __builtin__printf(format, *args):
	sys.stdout.write(format % args)

_global_vars = {
	'_g1': "%i %i\n",
}

def _f1_add(_a1_a, _a2_b, _debug_info):
	try:
		_1 = _1a_a
		_1 += _2a_b
	except:
		raise Exception(f"Error in function '{_debug_info["function_name"]}' at line {_debug_info["line"]}")

	return _1

def main():
	_1_a = _f1_add(1, 2);		# a
	_2_b = _f1_add(_1_a, 3);	# b
	__builtin__printf(_global_vars['_g1'], _1_a, _2_b)
```

#### With loops

```rost
fn main() {
	let a = 0;
	while a < 10 {
		printf("%i\n", a);
		a = a + 1;
	}
}
```

```python
import sys
def __builtin__printf(format, *args):
	sys.stdout.write(format % args)

_global_vars = {
	'_g1': "%i\n",
}

def main():
	_1_a = 0;
	while _1_a < 10:
		__builtin__printf(_global_vars['_g1'] % (_1_a))
		_1_a = _1_a + 1
```