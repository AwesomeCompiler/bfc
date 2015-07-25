# An optimising compiler for BF

[![Build Status](https://travis-ci.org/Wilfred/bfc.svg?branch=master)](https://travis-ci.org/Wilfred/bfc)

BFC is an optimising compiler for
[BF](https://en.wikipedia.org/wiki/Brainfuck).

It is written in Rust and uses LLVM.

```
BF source -> BF IR -> LLVM IR -> x86_32 Binary
```

GPLv2 or later license.

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc/generate-toc again -->
**Table of Contents**

- [An optimising compiler for BF](#an-optimising-compiler-for-bf)
    - [Compiling](#compiling)
    - [Usage](#usage)
    - [Running tests](#running-tests)
    - [Test programs](#test-programs)
    - [Peephole optimisations](#peephole-optimisations)
        - [Combining Instructions](#combining-instructions)
        - [Loop Simplification](#loop-simplification)
        - [Dead Code Elimination](#dead-code-elimination)
    - [Cell Bounds Analysis](#cell-bounds-analysis)
    - [Other projects optimising BF](#other-projects-optimising-bf)

<!-- markdown-toc end -->

## Compiling

You will need LLVM and Rust beta installed to compile bfc.

    $ cargo build

## Usage

```
$ cargo run -- sample_programs/hello_world.bf
$ lli hello_world.ll
Hello World!
```

## Running tests

```
$ cargo test
```

## Test programs

There are a few test programs in this repo, but
http://www.hevanet.com/cristofd/brainfuck/tests.b is an excellent
collection of test BF programs and
http://esoteric.sange.fi/brainfuck/bf-source/prog/ includes some more
elaborate programs.

## Peephole optimisations

bfc can use LLVM's optimisations, but it also offers some BF-specific
optimisations. There's a roadmap in
[optimisations.md](optimisations.md) of optimisations we will
implement at the BF IR level.

### Combining Instructions

We combine successive increments/decrements:

```
   Compile            Combine
+++  =>   Increment 1   =>   Increment 3
          Increment 1
          Increment 1
```

If increments/decrements cancel out, we remove them entirely.

```
   Compile             Combine
+-   =>   Increment  1    =>   # nothing!
          Increment -1
```

We do the same thing for data increments/decrements:

```
   Compile                Combine
>>>  =>   DataIncrement 1   =>   DataIncrement 3
          DataIncrement 1
          DataIncrement 1

   Compile                 Combine
><   =>   DataIncrement  1    =>   # nothing!
          DataIncrement -1
```

We do the same thing for successive sets:

```
      Combine
Set 1   =>   Set 2
Set 2

```

We combine sets and increments too:

```
  Compile            Known zero:         Combine
+   =>   Increment 1   =>   Set 0      =>   Set 1
                              Increment 1

```

We remove increments when there's a set immediately after:

```
            Combine
Increment 1   =>   Set 2
Set 2

```

We remove both increments and sets if there's a read immediately
after:

```
            Combine
Increment 1   =>   Read
Read

```

### Loop Simplification

`[-]` is a common BF idiom for zeroing cells. We replace that with
`Set`, enabling further instruction combination.

```
   Compile              Simplify
[-]  =>   Loop             =>   Set 0
            Increment -1
```

### Dead Code Elimination

We remove loops that we know are dead.

For example, loops at the beginning of a program:

```
    Compile                  Known zero               DCE
[>]   =>    Loop                 =>     Set 0          => Set 0
              DataIncrement 1           Loop
                                            DataIncrement 
```


Loops following another loop (one BF technique for comments is
`[-][this, is+a comment.]`).

```
      Compile                 Annotate                 DCE
[>][>]   =>  Loop                =>   Loop              =>   Loop
               DataIncrement 1          DataIncrement 1        DataIncrement 1
             Loop                     Set 0                  Set 0
               DataIncrement 1        Loop
                                          DataIncrement 1
```

We remove redundant set commands after loops (often generated by loop
annotation as above).

```
       Remove redundant set
Loop           =>   Loop
  Increment -1        Increment -1
Set 0

```

We also remove dead code at the end of a program.

```
        Remove pure code
Write         =>           Write
Increment 1
```

## Cell Bounds Analysis

BF programs can use up to 30,000 cells, all of which must be
zero-initialised. However, most programs don't use the whole range.

bfc uses static analysis to work out how many cells a BF program may
use, so it doesn't need to allocate or zero-initialise more memory
than necessary.

```
>><< only uses three cells
```

```
[>><<] uses three cells at most
```

```
[>><<]>>> uses four cells at most
```

```
[>] may use any number of cells, so we must assume 30,000
```

## Other projects optimising BF

There are also some interesting other projects for optimising BF
programs:

* https://code.google.com/p/esotope-bfc/wiki/Optimization
* http://calmerthanyouare.org/2015/01/07/optimizing-brainfuck.html
* [http://2π.com/10/brainfuck-using-llvm](http://2π.com/10/brainfuck-using-llvm)
* https://github.com/stedolan/bf.sed (simple optimisations, but
compiles directly to asm)
* https://github.com/matslina/bfoptimization

