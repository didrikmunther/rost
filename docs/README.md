# Rost

Compiles the Rost language to a Python stack machine backend.

## Demo (Web assembly backend)

[https://didrik.tech/rost/](https://didrik.tech/rost/)

## Roadmap

- [x] Recursive descent parser
- [x] AST representation
- [x] IR representation
- [x] Scoping of variables
- [x] Type checking
- [x] Generics
- [x] Error handling and reporting
- [x] LSP server
- [x] Builtin functions (externals)
- [x] Builtin types
- [x] Function calls
- [x] Function definitions
- [x] Compilation to Python stack machine
- [ ] Compilation to Webassembly
- [ ] Compilation to Assembly
- [ ] Custom types (structs)
- [ ] Loops
- [ ] If statements
- [ ] Trait system
- [ ] Broken code parsing
- [ ] Import files
- [ ] Package system

## Usage

#### Python backend

> `cargo run --bin main input.ro && python build/out.py`

#### WebAssembly backend

> `cargo run --bin main input.ro -b wasm && ./run-wasm.sh`

You may pass a compilation level argument, e.g. `cargo run --bin main input.ro parsed` to see intermittent representations of the code.

| Argument |
| -------- |
| lexed    |
| parsed   |
| compiled |

## Errors (CLI)

```rost
fn my_func(in: int) {}

my_func("abc");
```

![CleanShot 2024-09-26 at 11 05 04](https://github.com/user-attachments/assets/94b859b9-80df-4340-af2a-e8aa25eece19)

## LSP Server

<table cellpadding="0">
  <tr style="padding: 0">
    <td valign="top">
	<img src="https://github.com/user-attachments/assets/9294157c-000e-45dc-976f-486449061a93">
    </td>
    <td valign="top">
    	<img src="https://github.com/user-attachments/assets/81614c5d-44ad-445b-ba9c-2c4b058ee9c9">
	 <img src="https://github.com/user-attachments/assets/4b1f1ed7-61c7-4f48-b5fc-e25eb47447b5">
    </td>
  </tr>
</table>

- [x] VS Code extension
- [x] Hover support
- [x] Goto declaration/definition support
- [ ] Code action (quick fix) support

### Generic type errors

<img src="https://github.com/user-attachments/assets/a09ded4a-416d-48f8-8f20-b988232e97cc">

### Multi-error support

<img src="https://github.com/user-attachments/assets/080d7009-b981-4459-86a8-ee0ba8d03737">

### Parser error

<img src="https://github.com/user-attachments/assets/db68bff4-189d-4cbc-b28f-f1922e96b445">

### Lexer error

<img src="https://github.com/user-attachments/assets/b4538792-fe12-4155-a937-178e582c2006">

## Preview

### Example program (Python backend)

```rost
__builtin fn printf(format: str, ...args: any);

fn add_two_things<T>(format: str, a: T, b: T) {
	let c: T = a + b;

	printf(format, a, b, c);
}


let a: int = 1;
let c: int = 5;

let strvar = "abc";

add_two_things(
	"%i + %i = %i\n",
	a,
	c
);

add_two_things(
	"%s + %s = %s\n",
	strvar,
	"def"
);
```

<details>
	<summary>Python backend output</summary>

```python
# boilerplate_start begin

import sys


def __compiler__with_regular_args(func):
    def wrapper(*args):
        n_args = __stack.pop()
        args = [__stack.pop() for i in range(n_args)]
        func(*args)

    return wrapper


@__compiler__with_regular_args
def __builtin__printf(format, *args):
    sys.stdout.write(format % args)
    __intrinsic__stack_push(0)


__stack = []
__global_data = []
__global_variables = {}


def __intrinsic__stack_add():
    a = __stack.pop()
    b = __stack.pop()
    __stack.append(a + b)


def __intrinsic__stack_mul():
    a = __stack.pop()
    b = __stack.pop()
    __stack.append(a * b)


def __intrinsic__stack_push(value):
    __stack.append(value)


def __intrinsic__stack_pop():
    return __stack.pop()


# boilerplate_start end

# Builtin function: printf
def __userf__9():
	__builtin__printf()
# User function: add_two_things
def __userf__10():
	__intrinsic__stack_pop()
	_5_format = __intrinsic__stack_pop()
	_6_a = __intrinsic__stack_pop()
	_7_b = __intrinsic__stack_pop()
	__intrinsic__stack_push(_7_b)
	__intrinsic__stack_push(_6_a)
	__intrinsic__stack_add()
	_8_c = __intrinsic__stack_pop()
	__intrinsic__stack_push(_8_c)
	__intrinsic__stack_push(_7_b)
	__intrinsic__stack_push(_6_a)
	__intrinsic__stack_push(_5_format)
	__intrinsic__stack_push(4)
	# Procedure call: printf
	__userf__9()
	__intrinsic__stack_pop()

	__intrinsic__stack_push(0)
	pass

# Builtin function: printf
def __userf__15():
	__builtin__printf()
# User function: add_two_things
def __userf__16():
	__intrinsic__stack_pop()
	_11_format = __intrinsic__stack_pop()
	_12_a = __intrinsic__stack_pop()
	_13_b = __intrinsic__stack_pop()
	__intrinsic__stack_push(_13_b)
	__intrinsic__stack_push(_12_a)
	__intrinsic__stack_add()
	_14_c = __intrinsic__stack_pop()
	__intrinsic__stack_push(_14_c)
	__intrinsic__stack_push(_13_b)
	__intrinsic__stack_push(_12_a)
	__intrinsic__stack_push(_11_format)
	__intrinsic__stack_push(4)
	# Procedure call: printf
	__userf__15()
	__intrinsic__stack_pop()

	__intrinsic__stack_push(0)
	pass


def __setup():
	global __global_data
	__global_data = list(range(4))
	__global_data[0] = "abc"
	__global_data[1] = "%i + %i = %i\n"
	__global_data[2] = "def"
	__global_data[3] = "%s + %s = %s\n"

def __main():
	__intrinsic__stack_push(1)
	_2_a = __intrinsic__stack_pop()
	__intrinsic__stack_push(5)
	_3_c = __intrinsic__stack_pop()
	__intrinsic__stack_push(__global_data[0])
	_4_strvar = __intrinsic__stack_pop()
	__intrinsic__stack_push(_3_c)
	__intrinsic__stack_push(_2_a)
	__intrinsic__stack_push(__global_data[1])
	__intrinsic__stack_push(3)
	# Procedure call: add_two_things
	__userf__10()
	__intrinsic__stack_pop()
	__intrinsic__stack_push(__global_data[2])
	__intrinsic__stack_push(_4_strvar)
	__intrinsic__stack_push(__global_data[3])
	__intrinsic__stack_push(3)
	# Procedure call: add_two_things
	__userf__16()
	__intrinsic__stack_pop()

	pass

# boilerplate_exit begin


def __push_init_args():
    import sys

    args = [
        sys.argv,
        len(sys.argv),
    ]

    for arg in args:
        __intrinsic__stack_push(arg)

    __intrinsic__stack_push(len(args))


if __name__ == "__main__":
    import sys

    __push_init_args()
    __setup()

    __push_init_args()
    status = __main()

    print("stack", __stack)

    sys.exit(status)


# boilerplate_exit end
```

</details>

### Example program (WebAssembly backend)

```rost
__builtin fn print_raw(content: str, length: int);

print_raw("hejsan\n", 12);
```

<details>
	<summary>WebAssembly backend output</summary>

```wasm
	;; boilerplate_entry.wat begin

(module
    ;; Imports from JavaScript namespace
    (import  "imports"  "print_raw" (func  $print_raw (param  i32  i32))) ;; Import log function
    (import  "js"  "mem" (memory  1)) ;; Import 1 page of memory (54kb)

;; boilerplate_entry.wat end
	;; Global data section begin
	(data (i32.const 0) "hejsan\n")

	;; Function definitions begin
	;; Builtin function: 'print_raw' aliasing to '$print_raw'

	;; Main function definition
	(func $__main
		i32.const 12
		i32.const 0
		;; Builtin call: print_raw
		call $print_raw
	)
	(export "__main" (func $__main))
	;; boilerplate_exit.wat begin
)
;; boilerplate_exit.wat end

```

</details>

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

# Legacy:

## Run the nasm code

### Build Docker file

`./build`

### Run the NASM code

`./run.sh first-argument second-argument`

## Registers

Integer arguments: RDI/RSI/RDX/RCX/R8/R9

Float arguments: XMM0..XMM7
