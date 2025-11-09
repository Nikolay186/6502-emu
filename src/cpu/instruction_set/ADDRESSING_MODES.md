# Complete 6502 Addressing Modes Reference

The 6502 CPU uses 13 distinct addressing modes to specify how operands are accessed. Each mode has specific timing, memory access patterns, and use cases. This document provides comprehensive coverage of all addressing modes used in the 6502 processor.

## Overview of Addressing Modes

| Mode              | Syntax    | Example       | Bytes | Description                |
| ----------------- | --------- | ------------- | ----- | -------------------------- |
| Immediate         | `#$nn`    | `LDA #$42`    | 2     | Operand is literal value   |
| Zero Page         | `$nn`     | `LDA $80`     | 2     | Address in page 0 ($00xx)  |
| Zero Page,X       | `$nn,X`   | `LDA $80,X`   | 2     | Zero page + X register     |
| Zero Page,Y       | `$nn,Y`   | `LDX $80,Y`   | 2     | Zero page + Y register     |
| Absolute          | `$nnnn`   | `LDA $1234`   | 3     | Full 16-bit address        |
| Absolute,X        | `$nnnn,X` | `LDA $1234,X` | 3     | Absolute + X register      |
| Absolute,Y        | `$nnnn,Y` | `LDA $1234,Y` | 3     | Absolute + Y register      |
| Indexed Indirect  | `($nn,X)` | `LDA ($20,X)` | 2     | Indirect via (ZP + X)      |
| Indirect Indexed  | `($nn),Y` | `LDA ($20),Y` | 2     | Indirect + Y offset        |
| Implied           | -         | `TAX`         | 1     | No operand needed          |
| Accumulator       | `A`       | `ASL A`       | 1     | Operates on accumulator    |
| Relative          | `$nn`     | `BEQ $10`     | 2     | Signed offset for branches |
| Absolute Indirect | `($nnnn)` | `JMP ($1234)` | 3     | Indirect jump only         |

---

## 1. Immediate Addressing (`#$nn`)

### Description

The operand is the byte immediately following the opcode. The CPU reads the value directly from the instruction stream.

### Syntax

```assembly
LDA #$42    ; Load accumulator with immediate value 0x42
CMP #$FF    ; Compare accumulator with immediate value 0xFF
ADC #$01    ; Add 1 to accumulator with carry
```

### Memory Layout

```
Address: $8000  Opcode: $A9 (LDA immediate)
Address: $8001  Operand: $42
```

### Implementation Details

- **Cycles**: Always 2 cycles (fetch opcode, fetch operand)
- **Address calculation**: PC + 1 (operand address)
- **Value source**: Memory[PC + 1]
- **PC increment**: +2 (opcode + operand)
- **Page crossing**: Not applicable

### Usage Patterns

- Loading constants into registers
- Comparing registers with known values
- Setting up counters and flags
- Mathematical operations with constants
- Bit masking operations

### Supported Instructions

`ADC`, `AND`, `CMP`, `CPX`, `CPY`, `EOR`, `LDA`, `LDX`, `LDY`, `ORA`, `SBC`

### Example Code

```assembly
LDA #$00    ; Clear accumulator
LDX #$FF    ; Initialize X to 255
CMP #$80    ; Check if accumulator >= 128
```

---

## 2. Zero Page Addressing (`$nn`)

### Description

Uses an 8-bit address to access the first 256 bytes of memory (page 0: $0000-$00FF). The high byte is always $00, making this the fastest addressing mode after immediate.

### Syntax

```assembly
LDA $80     ; Load from zero page address $0080
STA $90     ; Store to zero page address $0090
INC $A0     ; Increment value at $00A0
```

### Memory Layout

```
Address: $8000  Opcode: $A5 (LDA zero page)
Address: $8001  Zero page address: $80
Target:  $0080  Data value
```

### Implementation Details

- **Cycles**: 3 cycles (fetch opcode, fetch ZP address, read/write data)
- **Address calculation**: $00 << 8 | operand
- **Effective address**: $00nn
- **PC increment**: +2
- **Page crossing**: Not applicable

### Performance Benefits

- Faster than absolute addressing (3 vs 4 cycles)
- Smaller instruction size (2 vs 3 bytes)
- Critical for performance-sensitive code

### Usage Patterns

- Temporary storage and variables
- Frequently accessed data
- Indirect addressing setup
- Compiler-generated temporary values
- System variables and flags

### Zero Page Memory Map (Typical)

```
$00-$01: Indirect addressing pointers
$02-$FF: Available for user programs
$00F0-$00FF: Often reserved for system use
```

### Example Code

```assembly
        LDA #$42
        STA $80     ; Store value in zero page
        INC $80     ; Increment the value
        LDA $80     ; Load it back
```

---

## 3. Zero Page,X Addressing (`$nn,X`)

### Description

Adds the X register to the zero page base address. If the addition overflows beyond $FF, it wraps around within page 0 (this is intentional behavior).

### Syntax

```assembly
LDA $80,X   ; Load from zero page address ($80 + X) & $FF
STA $90,X   ; Store to zero page address ($90 + X) & $FF
INC $A0,X   ; Increment value at ($A0 + X) & $FF
```

### Wrap-around Behavior

```assembly
LDX #$90
LDA $80,X   ; Effective address = ($80 + $90) & $FF = $10
            ; Accesses $0010, not $0110!
```

### Implementation Details

- **Cycles**: 4 cycles (fetch opcode, fetch base, add X, read/write)
- **Address calculation**: ($00 << 8) | ((base + X) & $FF)
- **Wrap behavior**: Addition wraps at page boundary
- **PC increment**: +2
- **Page crossing**: Not applicable (wrapping is intentional)

### Usage Patterns

- Array indexing in zero page
- Data structure traversal
- Loop counters and iterators
- Table lookups
- Buffer manipulation

### Example: Array Processing

```assembly
        LDX #$00        ; Initialize index
loop:   LDA $80,X       ; Load array element
        CLC
        ADC #$01        ; Increment each element
        STA $80,X       ; Store back
        INX
        CPX #$10        ; Process 16 elements
        BNE loop
```

### Wrap-around Consideration

The wrap-around behavior must be implemented correctly in emulators:

```rust
fn zero_page_x_address(&self, base: u8) -> u16 {
    ((base.wrapping_add(self.x)) as u16) & 0xFF
}
```

---

## 4. Zero Page,Y Addressing (`$nn,Y`)

### Description

Similar to Zero Page,X but uses the Y register. This addressing mode is only available with `LDX` and `STX` instructions.

### Syntax

```assembly
LDX $80,Y   ; Load X from zero page address ($80 + Y) & $FF
STX $90,Y   ; Store X to zero page address ($90 + Y) & $FF
```

### Implementation Details

- **Cycles**: 4 cycles
- **Address calculation**: ($00 << 8) | ((base + Y) & $FF)
- **Limited instruction set**: Only `LDX` and `STX`
- **PC increment**: +2
- **Wrap behavior**: Same as Zero Page,X

### Why Only LDX/STX?

This limitation exists because:

- Y register is typically used for vertical indexing
- X register operations with Y indexing provide specific functionality
- Reduces CPU complexity and transistor count

### Usage Patterns

- Specialized for X register operations
- Less common than Zero Page,X
- Used in specific algorithms requiring Y-indexed X operations
- Sprite coordinate calculations

### Example Code

```assembly
        LDY #$00        ; Initialize Y index
        LDX $80,Y       ; Load X coordinate
        STX $90,Y       ; Store to output buffer
```

---

## 5. Absolute Addressing (`$nnnn`)

### Description

Uses a full 16-bit address to access any location in the 64KB address space. The address is stored in little-endian format (low byte first, high byte second).

### Syntax

```assembly
LDA $1234   ; Load from absolute address $1234
JMP $8000   ; Jump to absolute address $8000
STA $2000   ; Store to absolute address $2000
```

### Memory Layout

```
Address: $8000  Opcode: $AD (LDA absolute)
Address: $8001  Low byte: $34
Address: $8002  High byte: $12
Target:  $1234  Data value
```

### Implementation Details

- **Cycles**: 4 cycles for read operations, 4 cycles for write operations
- **Address calculation**: (high_byte << 8) | low_byte
- **Byte order**: Little-endian (low byte first)
- **PC increment**: +3 (opcode + low byte + high byte)
- **Page crossing**: Not applicable

### Little-Endian Format

The 6502 stores multi-byte values with the least significant byte first:

```
Memory:   $8001: $34 (low byte)
          $8002: $12 (high byte)
Address:  $1234
```

### Usage Patterns

- Accessing specific memory locations
- Memory-mapped I/O registers
- ROM data and lookup tables
- Jump targets for subroutines
- Variable storage outside zero page
- Hardware control registers

### Memory Map Examples

```assembly
; PPU registers (NES example)
LDA $2002   ; PPU status register
STA $2001   ; PPU control register

; RAM storage
LDA $0300   ; Load from RAM
STA $0400   ; Store to different RAM location

; ROM data
LDA $8000   ; Load from ROM
```

---

## 6. Absolute,X Addressing (`$nnnn,X`)

### Description

Adds the X register to a 16-bit base address. This mode can cross page boundaries, which incurs a timing penalty for read operations.

### Syntax

```assembly
LDA $1234,X ; Load from address $1234 + X
STA $2000,X ; Store to address $2000 + X
INC $3000,X ; Increment value at $3000 + X
```

### Page Crossing Behavior

```assembly
LDX #$FF
LDA $12FF,X ; Base: $12FF, X: $FF
            ; Result: $13FE (crosses page boundary)
            ; Takes 5 cycles instead of 4 for reads
```

### Implementation Details

- **Base cycles**: 4 cycles for reads, 5 cycles for writes
- **Page cross penalty**: +1 cycle for reads when crossing page boundary
- **Address calculation**: base_address + X_register
- **PC increment**: +3
- **Overflow**: Address can wrap around at $FFFF

### Page Crossing Detection

A page boundary is crossed when the high byte changes:

```rust
fn page_crossed(base: u16, indexed: u16) -> bool {
    (base & 0xFF00) != (indexed & 0xFF00)
}
```

### Timing Examples

```assembly
; 4 cycles - no page cross
LDX #$10
LDA $1200,X ; $1200 + $10 = $1210 (same page)

; 5 cycles - page cross
LDX #$10
LDA $12F0,X ; $12F0 + $10 = $1300 (page crossed)

; 5 cycles - write operation (always)
LDX #$05
STA $2000,X ; Always 5 cycles regardless of page crossing
```

### Usage Patterns

- Array processing and iteration
- String manipulation
- Buffer operations
- Data table access
- Horizontal scrolling in graphics
- Memory copying operations

### Example: Array Sum

```assembly
        LDX #$00        ; Initialize index
        LDA #$00        ; Initialize sum
loop:   CLC
        ADC $1000,X     ; Add array element
        INX
        CPX #$20        ; Process 32 elements
        BNE loop
        STA $2000       ; Store result
```

---

## 7. Absolute,Y Addressing (`$nnnn,Y`)

### Description

Identical to Absolute,X but uses the Y register instead of X. Has the same page crossing behavior and timing characteristics.

### Syntax

```assembly
LDA $1234,Y ; Load from address $1234 + Y
STA $2000,Y ; Store to address $2000 + Y
```

### Implementation Details

- **Cycles**: Same as Absolute,X (4 base, +1 for page cross on reads)
- **Page crossing**: Same penalty rules apply
- **Address calculation**: base_address + Y_register
- **PC increment**: +3

### Usage Patterns

- Vertical array processing (Y often represents rows/vertical position)
- Sprite data manipulation
- Screen buffer operations
- 2D array access (Y for rows)
- Graphics operations

### Example: 2D Array Access

```assembly
; Access element at row Y, treating each row as 40 bytes
; Base address: $2000
; Element = base + (Y * 40) + X

        LDY #$02        ; Row 2
        LDX #$05        ; Column 5

        ; Calculate Y * 40 (simplified approach)
        LDA row_offsets,Y ; Pre-calculated row offsets
        CLC
        ADC #$05        ; Add column offset
        TAX
        LDA $2000,X     ; Load the element
```

---

## 8. Indexed Indirect (`($nn,X)`) - "Pre-indexed"

### Description

This complex addressing mode first adds X to a zero page address, then reads a 16-bit pointer from that location, and finally accesses the address pointed to by that pointer.

### Syntax

```assembly
LDA ($20,X) ; Load from address pointed to by ($20 + X)
STA ($30,X) ; Store to address pointed to by ($30 + X)
```

### Step-by-Step Execution

```
Given: X = $04, memory at $24-$25 contains $00 $30
1. Calculate pointer address: ($20 + $04) & $FF = $24
2. Read low byte from $0024 = $00
3. Read high byte from $0025 = $30
4. Final address = $3000
5. Access data at $3000
```

### Zero Page Wrap-around

```assembly
LDX #$FF
LDA ($80,X) ; Pointer address = ($80 + $FF) & $FF = $7F
            ; Low byte from $007F
            ; High byte from $0000 (wraps around!)
```

### Implementation Details

- **Cycles**: 6 cycles (most complex addressing mode)
- **Zero page wrap**: Pointer address calculation wraps within zero page
- **Little-endian pointer**: Low byte first, high byte second
- **PC increment**: +2
- **Page crossing**: Not applicable

### Memory Layout Example

```
Zero Page:
$0024: $00 (low byte of pointer)
$0025: $30 (high byte of pointer)

Target:
$3000: $42 (actual data)

Instruction:
LDA ($20,X) ; where X = $04
```

### Usage Patterns

- Function pointer tables
- Indirect jumps through tables
- Dynamic addressing schemes
- Complex data structures
- Object-oriented programming patterns
- Interpreter implementations

### Example: Jump Table

```assembly
; Jump table in zero page
jump_table:
        .word routine1  ; $20-$21
        .word routine2  ; $22-$23
        .word routine3  ; $24-$25

; Jump to routine based on A register
        ASL             ; A * 2 (word size)
        TAX
        JMP ($20,X)     ; Jump via table
```

---

## 9. Indirect Indexed (`($nn),Y`) - "Post-indexed"

### Description

Reads a 16-bit pointer from zero page, then adds the Y register to that address. This is the most commonly used indirect addressing mode.

### Syntax

```assembly
LDA ($20),Y ; Load from (pointer at $20-$21) + Y
STA ($30),Y ; Store to (pointer at $30-$31) + Y
```

### Step-by-Step Execution

```
Given: Y = $05, memory at $20-$21 contains $00 $30
1. Read low byte from $0020 = $00
2. Read high byte from $0021 = $30
3. Base address = $3000
4. Add Y: $3000 + $05 = $3005
5. Access data at $3005
```

### Implementation Details

- **Base cycles**: 5 cycles
- **Page cross penalty**: +1 cycle if Y addition crosses page boundary
- **Zero page pointer**: Pointer must be in zero page
- **PC increment**: +2
- **Little-endian pointer**: Standard little-endian format

### Page Crossing with Y

```assembly
; Setup pointer at $20-$21 = $30FF
LDY #$05
LDA ($20),Y ; Base: $30FF, Y: $05 = $3104
            ; Page crossed ($30 to $31), +1 cycle = 6 total
```

### Usage Patterns

- Array of arrays/structures
- String tables and text processing
- Sprite data tables
- Dynamic data structures
- Memory management
- Screen buffer manipulation

### Example: String Table

```assembly
; String table - each entry points to a string
string_table:
        .word string1   ; $20-$21
        .word string2   ; $22-$23
        .word string3   ; $24-$25

; Print character Y from string X
print_char:
        ; X = string index, Y = character index
        TXA
        ASL             ; X * 2 for word offset
        TAX
        LDA ($20,X)     ; Get string pointer
        STA $40         ; Store in zero page
        LDA ($21,X)
        STA $41
        LDA ($40),Y     ; Get character
        ; ... print character
```

### Complex Example: 2D Array Access

```assembly
; Access 2D array: array[row][col]
; Each row pointer stored in zero page

        LDA row         ; Get row index
        ASL             ; * 2 for word size
        TAX
        LDY col         ; Get column index
        LDA ($80,X)     ; Get row pointer via indexed indirect
        STA temp        ; Store in temporary location
        LDA ($81,X)
        STA temp+1
        LDA (temp),Y    ; Access array[row][col]
```

---

## 10. Implied Addressing

### Description

No operand is needed. The instruction operates on CPU registers, has fixed behavior, or the operand is implicit in the instruction itself.

### Syntax

```assembly
TAX         ; Transfer A to X (register to register)
NOP         ; No operation
RTS         ; Return from subroutine (uses stack)
CLC         ; Clear carry flag
```

### Implementation Details

- **Cycles**: Varies by instruction (2-6 cycles)
- **Instruction size**: 1 byte (opcode only)
- **PC increment**: +1
- **Operands**: None explicitly specified

### Instruction Categories

#### Register Transfers (2 cycles each)

```assembly
TAX         ; Transfer A to X
TAY         ; Transfer A to Y
TXA         ; Transfer X to A
TYA         ; Transfer Y to A
TSX         ; Transfer Stack pointer to X
TXS         ; Transfer X to Stack pointer
```

#### Stack Operations

```assembly
PHA         ; Push A to stack (3 cycles)
PLA         ; Pull A from stack (4 cycles)
PHP         ; Push Processor status to stack (3 cycles)
PLP         ; Pull Processor status from stack (4 cycles)
```

#### Register Arithmetic (2 cycles each)

```assembly
INX         ; Increment X
INY         ; Increment Y
DEX         ; Decrement X
DEY         ; Decrement Y
```

#### Status Flag Operations (2 cycles each)

```assembly
CLC         ; Clear Carry flag
CLD         ; Clear Decimal flag
CLI         ; Clear Interrupt disable flag
CLV         ; Clear Overflow flag
SEC         ; Set Carry flag
SED         ; Set Decimal flag
SEI         ; Set Interrupt disable flag
```

#### Control Instructions

```assembly
NOP         ; No Operation (2 cycles)
BRK         ; Break (7 cycles)
RTI         ; Return from Interrupt (6 cycles)
RTS         ; Return from Subroutine (6 cycles)
```

### Usage Examples

```assembly
; Register manipulation
        LDA #$42
        TAX             ; Copy A to X
        TXA             ; Copy X back to A

; Status flag control
        CLC             ; Ensure carry is clear
        ADC #$01        ; Add with carry

; Stack operations
        PHA             ; Save A on stack
        LDA #$00        ; Do something else
        PLA             ; Restore A from stack
```

---

## 11. Accumulator Addressing (`A`)

### Description

The instruction operates directly on the accumulator register. The accumulator is both the source and destination of the operation.

### Syntax

```assembly
ASL A       ; Arithmetic shift left accumulator
ROL A       ; Rotate left accumulator
LSR A       ; Logical shift right accumulator
ROR A       ; Rotate right accumulator
```

### Implementation Details

- **Cycles**: 2 cycles
- **No memory access**: Operates on register only
- **PC increment**: +1
- **Instruction size**: 1 byte

### Supported Instructions

- `ASL A` - Arithmetic Shift Left
- `LSR A` - Logical Shift Right
- `ROL A` - Rotate Left
- `ROR A` - Rotate Right

### Assembler Variations

Some assemblers allow omitting the 'A':

```assembly
ASL         ; Same as ASL A
LSR         ; Same as LSR A
ROL         ; Same as ROL A
ROR         ; Same as ROR A
```

### Bit Operations Detail

#### ASL A (Arithmetic Shift Left)

```
Before: C <- [7][6][5][4][3][2][1][0] <- 0
After:  C <- [6][5][4][3][2][1][0][0]
```

#### LSR A (Logical Shift Right)

```
Before: 0 -> [7][6][5][4][3][2][1][0] -> C
After:     [0][7][6][5][4][3][2][1] -> C
```

#### ROL A (Rotate Left)

```
Before: C <- [7][6][5][4][3][2][1][0] <- C
After:  C <- [6][5][4][3][2][1][0][C]
```

#### ROR A (Rotate Right)

```
Before: C -> [7][6][5][4][3][2][1][0] -> C
After:  [C][7][6][5][4][3][2][1] -> C
```

### Usage Examples

```assembly
; Multiply by 2 (fast)
        LDA value
        ASL A           ; A = A * 2
        STA result

; Divide by 2 (fast)
        LDA value
        LSR A           ; A = A / 2
        STA result

; Extract high nibble
        LDA value
        LSR A
        LSR A
        LSR A
        LSR A           ; High nibble now in low 4 bits
```

---

## 12. Relative Addressing (`$nn`)

### Description

Used exclusively for branch instructions. An 8-bit signed offset (-128 to +127) is added to the current program counter to determine the branch target.

### Syntax

```assembly
BEQ $10     ; Branch if equal, offset +16
BNE $F0     ; Branch if not equal, offset -16 (240 = -16 in two's complement)
BCC forward ; Branch forward (assembler calculates offset)
```

### Signed Offset Interpretation

- `$00` to `$7F`: +0 to +127 (forward branches)
- `$80` to `$FF`: -128 to -1 (backward branches)

### Address Calculation

```
Branch Target = Current PC + signed_offset
```

Where Current PC points to the instruction after the branch.

### Implementation Details

- **Base cycles**: 2 cycles (branch not taken)
- **Branch taken**: +1 cycle (3 total)
- **Page cross on branch**: +1 additional cycle (4 total)
- **PC increment**: +2 (then add offset if branch taken)

### Branch Timing Examples

```assembly
start:  LDA #$00        ; Address $8000
        CMP #$01        ; Address $8002
        BEQ skip        ; Address $8004, PC now = $8006
        LDA #$FF        ; Address $8006 (if branch not taken)
skip:   NOP             ; Address $8008

; If branch taken: PC = $8006 + offset to reach $8008
; Offset = $8008 - $8006 = $02
```

### Page Crossing Penalty

A page boundary is crossed when the branch changes the high byte of PC:

```assembly
        ; At address $80FE
        BEQ $7F         ; Branch to $817F
        ; PC before: $8100 (after reading branch)
        ; PC after:  $817F
        ; Page crossed: $81 to $81 (no crossing)

        ; At address $80FE
        BEQ $02         ; Branch to $8102
        ; PC before: $8100
        ; PC after:  $8102
        ; Page crossed: $81 to $81 (no crossing)

        ; At address $80FE
        BEQ $82         ; Branch to $8182
        ; PC before: $8100
        ; PC after:  $8182
        ; Page crossed: $81 to $81 (no crossing - same page!)
```

### All Branch Instructions

```assembly
BPL     ; Branch if Plus (N = 0)
BMI     ; Branch if Minus (N = 1)
BVC     ; Branch if Overflow Clear (V = 0)
BVS     ; Branch if Overflow Set (V = 1)
BCC     ; Branch if Carry Clear (C = 0)
BCS     ; Branch if Carry Set (C = 1)
BNE     ; Branch if Not Equal (Z = 0)
BEQ     ; Branch if Equal (Z = 1)
```

### Usage Patterns

- Conditional execution
- Loops and iteration
- Error handling and validation
- Program flow control
- State machines

### Example: Simple Loop

```assembly
        LDX #$10        ; Loop counter
loop:   DEX             ; Decrement counter
        BNE loop        ; Branch if not zero
        ; Loop executes 16 times
```

### Example: Bounds Checking

```assembly
        LDA index
        CMP #$80        ; Check upper bound
        BCS error       ; Branch if >= 128
        ; Continue normal processing
error:  ; Handle error condition
```

---

## 13. Absolute Indirect (`($nnnn)`)

### Description

Used exclusively by the `JMP` instruction. Reads a 16-bit address from the specified location and jumps to that address. This mode has a famous hardware bug when the indirect address crosses a page boundary.

### Syntax

```assembly
JMP ($1234) ; Jump to address stored at $1234-$1235
JMP (vector) ; Jump to address stored at label 'vector'
```

### Step-by-Step Execution

```
Given: Memory at $1234 = $00, Memory at $1235 = $80
1. Read low byte from $1234 = $00
2. Read high byte from $1235 = $80
3. Jump to address $8000
```

### The Famous Page Boundary Bug

The 6502 has a well-documented bug with this addressing mode when the indirect address ends in $FF:

```assembly
JMP ($12FF) ; Bug case!
```

**Expected behavior:**

- Read low byte from `$12FF`
- Read high byte from `$1300`

**Actual 6502 behavior:**

- Read low byte from `$12FF`
- Read high byte from `$1200` (wraps within the same page!)

### Bug Implementation

```rust
fn jmp_indirect_with_6502_bug(&mut self, addr: u16) -> u16 {
    let low = self.memory.read(addr);
    let high_addr = if (addr & 0xFF) == 0xFF {
        addr & 0xFF00  // Bug: wrap to start of same page
    } else {
        addr + 1       // Normal: next sequential address
    };
    let high = self.memory.read(high_addr);
    ((high as u16) << 8) | (low as u16)
}
```

### Implementation Details

- **Cycles**: 5 cycles
- **Page wrap bug**: High byte address wraps within same page
- **Only instruction**: `JMP` absolute indirect
- **PC behavior**: Direct jump (no increment, PC = target address)

### Bug Examples

```assembly
; Normal case - no bug
JMP ($1234) ; Reads from $1234 and $1235 correctly

; Bug case
JMP ($12FF) ; Reads low byte from $12FF
            ; Reads high byte from $1200 (NOT $1300!)

; If $12FF contains $34 and $1200 contains $56
; Jump target becomes $5634 instead of expected address
```

### Workarounds for the Bug

Programmers learned to avoid placing jump vectors at page boundaries:

```assembly
; BAD - could trigger bug
vectors:    .word routine1  ; At $12FF-$1300

; GOOD - safe placement
        .align 256          ; Align to page boundary
vectors:    .word routine1  ; At $1300-$1301
```

### Usage Patterns

- Jump tables and dispatch tables
- Function pointers and callbacks
- Dynamic dispatch mechanisms
- Bootloader and reset vectors
- Interrupt vector tables

### Example: Jump Table

```assembly
; Jump table with safety padding
        .align 256      ; Ensure no page boundary issues
jump_table:
        .word routine0  ; Function 0
        .word routine1  ; Function 1
        .word routine2  ; Function 2
        .word routine3  ; Function 3

; Jump based on A register value
dispatch:
        ASL             ; A * 2 (word size)
        TAX
        JMP (jump_table,X) ; Use indexed indirect instead

; Or using absolute indirect
        ASL
        CLC
        ADC #<jump_table ; Add table base address
        STA vector
        LDA #>jump_table
        ADC #$00
        STA vector+1
        JMP (vector)    ; Safe if vector not at page boundary
```

---

## Memory Access Timing Summary

Understanding cycle timing is crucial for accurate emulation and performance optimization:

| Addressing Mode   | Read Cycles | Write Cycles | Page Cross Penalty | Notes                  |
| ----------------- | ----------- | ------------ | ------------------ | ---------------------- |
| Immediate         | 2           | N/A          | No                 | Fastest operand access |
| Zero Page         | 3           | 3            | No                 | Fastest memory access  |
| Zero Page,X       | 4           | 4            | No                 | Wraps within page 0    |
| Zero Page,Y       | 4           | 4            | No                 | LDX/STX only           |
| Absolute          | 4           | 4            | No                 | Standard memory access |
| Absolute,X        | 4\*         | 5            | Yes (+1 read)      | Very common            |
| Absolute,Y        | 4\*         | 5            | Yes (+1 read)      | Graphics operations    |
| Indexed Indirect  | 6           | 6            | No                 | Complex, 6502 bug      |
| Indirect Indexed  | 5\*         | 6            | Yes (+1 read)      | Most useful indirect   |
| Implied           | 2-6         | N/A          | No                 | Varies by instruction  |
| Accumulator       | 2           | N/A          | No                 | Register operation     |
| Relative          | 2\*\*       | N/A          | Yes\*\*\*          | Branch timing          |
| Absolute Indirect | 5           | N/A          | No                 | JMP only, has bug      |

**Legend:**

- `*` = Base cycles, +1 if page boundary crossed on reads
- `**` = Base cycles, +1 if branch taken, +1 more if page crossed
- `***` = Page cross penalty only applies when branch is taken

---

## Emulator Implementation Guidelines

### Address Resolution Pattern

```rust
#[derive(Debug, Clone, Copy)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndexedIndirect,    // ($nn,X)
    IndirectIndexed,    // ($nn),Y
    Implied,
    Accumulator,
    Relative,
    AbsoluteIndirect,   // ($nnnn)
}

impl CPU {
    fn resolve_operand_address(&mut self, mode: AddressingMode) -> (u16, bool) {
        match mode {
            AddressingMode::Immediate => {
                let addr = self.pc;
                self.pc += 1;
                (addr, false) // No page crossing possible
            },

            AddressingMode::ZeroPage => {
                let addr = self.read_pc_byte() as u16;
                (addr, false)
            },

            AddressingMode::ZeroPageX => {
                let base = self.read_pc_byte();
                let addr = ((base.wrapping_add(self.x)) as u16) & 0xFF;
                (addr, false) // Wrapping is intentional, no penalty
            },

            AddressingMode::Absolute => {
                let addr = self.read_pc_word();
                (addr, false)
            },

            AddressingMode::AbsoluteX => {
                let base = self.read_pc_word();
                let addr = base.wrapping_add(self.x as u16);
                let page_crossed = (base & 0xFF00) != (addr & 0xFF00);
                (addr, page_crossed)
            },

            AddressingMode::IndirectIndexed => {
                let ptr_addr = self.read_pc_byte() as u16;
                let base = self.read_word_zero_page(ptr_addr);
                let addr = base.wrapping_add(self.y as u16);
                let page_crossed = (base & 0xFF00) != (addr & 0xFF00);
                (addr, page_crossed)
            },

            // ... other modes
        }
    }

    fn read_word_zero_page(&self, addr: u16) -> u16 {
        let low = self.memory.read(addr);
        let high_addr = (addr & 0xFF00) | ((addr + 1) & 0xFF); // Zero page wrap
        let high = self.memory.read(high_addr);
        ((high as u16) << 8) | (low as u16)
    }
}
```

### Cycle Counting Implementation

```rust
fn execute_with_timing(&mut self, opcode: u8) -> u8 {
    let instruction = INSTRUCTION_TABLE[opcode as usize];
    let (address, page_crossed) = self.resolve_operand_address(instruction.addressing_mode);

    let mut cycles = instruction.base_cycles;

    // Apply page crossing penalty for specific modes and operations
    if page_crossed && instruction.has_page_cross_penalty {
        match instruction.operation_type {
            OperationType::Read => cycles += 1,
            OperationType::Write => {}, // Writes always take extra cycle
            OperationType::ReadModifyWrite => {}, // RMW not affected
        }
    }

    // Execute the actual instruction
    self.execute_instruction(instruction, address);

    cycles
}
```

### Branch Timing Implementation

```rust
fn execute_branch(&mut self, condition: bool, offset: i8) -> u8 {
    let mut cycles = 2; // Base branch instruction cycles

    if condition {
        cycles += 1; // Branch taken

        let old_pc = self.pc;
        let new_pc = ((self.pc as i32) + (offset as i32)) as u16;

        // Check for page crossing
        if (old_pc & 0xFF00) != (new_pc & 0xFF00) {
            cycles += 1; // Page boundary crossed
        }

        self.pc = new_pc;
    }

    cycles
}
```

This comprehensive reference covers all 6502 addressing modes with the depth needed for accurate emulation. Each mode's timing, behavior, and edge cases are documented to ensure proper implementation in your CPU emulator.
