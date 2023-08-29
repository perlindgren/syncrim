# The SyncRim RISC-V model

The RISC-V ISA defines four different base integer instructions sets starting with the RV32E, which provides 16 CPU registers and 57 basic instructions spread over 6 different instruction formats, much in the spirit of a RISC architecture. The RV32I provides 16 additional CPU registers, totalling 31 registers, with the instruction set being identical to the RV32E. Aside from the 32-bit variants, base instruction sets for 64-bit and 128-bit architectures are also defined. For SyncRim, we choose to focus on the RV32I instruction set.

In this document, we design a single-cycle RV32I model using standard SyncRim and SyncRim RISC-V components, starting with the instruction fetch stage, and extending it until the entirety of the RV32I instruction set is covered. For implementation details of the components, refer to the component source code.

## Instruction fetch
To start off, we need some way of feeding out model with instructions. To that end consider the following circuit:

The output of the adder will increase by 4 on each clock cycle, yielding the instruction address. All that remains is feeding this signal to the instruction memory component:

and out comes the instruction!

## Instruction decode
Now, with the instruction in hand, we must extract meaningful data from it. To that end, we use the decoder component:

The decoder component will control the signal flow through the model by controlling multiplexers, and forward relevant instruction fields to other components.
The control path, and relevant fields of a RISC-V instruction are determined by it's OPCODE field. The RV32I instruction set includes 11 different opcodes, which we will now implement, adding components to the model as we go along.

## Arithmetic
### OP
The OP instructions implement simple arithmetic register-register operations. To that end, we add a register file to keep track of register state, and an Arithmetic Logic Unit(ALU) to perform the requested arithmetic operation:

The decoder *always* forwards the rs1, rs2 and rd fields of the instruction to the register file. The register file then outputs the contents of the rs1 and rs2 registers to the ALU. The specific arithmetic operation performed by the ALU is decided by a control signal from the decoder. The result of the ALU operation is sent back to the register file, which stores it in register rd.

### OP-IMM
The OP-IMM instructions can be viewed as an extension of the OP instructions, allowing arithmetic operations on the contents of the rs1 register and the immediate field of the instruction. To this end, we let the decoder forward the immediate field through a sign/zero extender controlled by the decoder, to the rs2 input of the ALU, and add a multiplexer, allowing the decoder to control which of the two inputs is used in the arithmetic operation:

## Memory operations
### LOAD
The LOAD instructions implement memory-register loads. To this end, we add a memory component to the model. The decoder forwards the rs1 and rd fields to the register file and the immediate field to the ALU, selects the requested data width, and selects the ADD operation on the ALU to calculate the requested read address. The resulting address is fed from the ALU to the data memory component. Finally, we add a multiplexer to the output to the ALU result output, and feed the data memory output through it, allowing the decoder to decide the register file write back data source.

### STORE
The STORE instructions implement register-memory stores. To implement this, we calculate the target address the same way as for the LOAD. We forward the reg file rs2 output to the data memory data input, and allow the decoder to control a write enable signal to the data memory to unwanted memory writes. Note that, for the first time, we are *not* performing a register write. To that end, we allow the decoder to control a write enable signal on the register file to prevent unwanted register writes.

## Control transfer
### JALR
A JALR instruction is an absolute jump-and-link instruction, adding the contents of the rs1 register to the contents of the (sign-extended) immediate field, and setting the program counter to the resulting value. To accomplish this, we extend the model with a JALR-specific adder. We set the two inputs of the adder to the sign-extended immediate forward of the decoder, and the rs1 output of the register file. Lastly, to follow the RISC-V ISA, the least significant bit of the resulting address must be set to 0 to ensure alignment.

To model the control transfer of the JALR instruction, we extend our, so far, simple instruction fetch stager with a multiplexer. The inputs of the multiplexer will be the results of the PC-adder, and the results of the JALR-adder. To control the multiplexer, we use a Branch Logic Unit(BLU) component. We let the decoder control the selected control transfer operation (in this case JALR).

Finally, we must write the address of the source instruction back to rd. We connect the program counter signal to the ALU rs2 multiplexer, and let the decoder select it. We also add a multiplexer to the rs1 signal of the ALU, and connect it to a constant 0. This allows the decoder to select the ALU ADD operation on the program counter and 0, yielding the program counter.

### JAL
The JAL instruction performs a program counter-relative branch and link. Because the jump is program counter relative, we choose to extend the model with another adder. The inputs of the JAL adder are the current program counter, and the forwarded immediate, and the output is forwarded to the program counter mux. The decoder selects the JAL operation on the BLU, and performs the source address store in the same fashion as the JALR instruction.

### BRANCH
The BRANCH instructions are also program counter-relative branches. Because the branch is program counter relative, we can reuse the JAL adder, feeding the JAL and BRANCH immediates through a multiplexer, and on to the JAL adder input. 

BRANCH instructions are conditional. The decoder forwards the rs1 and rs2 fields to the register file, and the rs1 and rs2 outputs of the register file are forwarded to the BLU. The decoder selects the relevant branch operation on the BLU, allowing it to make the required comparison and control the program counter multiplexer accordingly.

## Miscellaneous operations
### AUIPC
### LUI
### SYSTEM
The SYSTEM opcode under the RV32I instruction set includes Control Status Register(CSR) reads and writes. Since in the base implementation, the only available CSRs are read-only counters, we choose to omit the from the model. In the future, we will implement them to extend the RISC-V model with an interrupt controller.
### MISC-MEM
The MISC-MEM opcode under RV32I only includes the FENCE operation. The RISC-V ISA specification states that simpler cores may omit it completely by implementing it as a NOP. Since we are simulating a single-hart system with a very simple memory interface, out model falls under the simple category in this context. Because of this, we choose to implement FENCE as NOP.


In the SyncRim model of the RV32I instruction set, each of the instructions is fed by the instruction fetch stage (which in it's most basic implementation consists of an adder with it's inputs hardwired to the program counter register, and a constant 0x4 signal (incrementing the program counter by 4 bytes with each clock cycle), and it's output sent directly to the instruction memory, which in turn outputs the instruction at the input address to the decoder), through the decoder component which acts as a controller for the rest of the model, forwarding parts of the instructions to relevant components, and controlling the signal flow according to the input opcode using multiplexers. The RV32I only defines eleven unique opcodes, with the signal flow of different instructions using the same opcode being identical. In the following section we describe the signal flow through the SyncRim RISC-V model for each of the opcodes, starting at the aforementioned decoder. 

%this is 100% impossible to follow without some kind of figure, maybe even one for each "extension" proposed here...
\subsection{OP}
    The OP instructions implement simple arithmetic register-register operations. For an OP instruction, the decoder forwards the rs1 rs2 and rd fields of the instruction to the register file component, selects the register signals on the two ALU input muxes, sends the appropriate control signal to the ALU signifying the appropriate arithmetic operation, selects the ALU output signal on the register file write back MUX, and finally sets the write enable signal of the register file to 1, signifying the output signal of the write back MUX should be written to the rd register. 
\subsection{OP-IMM}
    The OP-IMM instructions are variants on the OP instructions using the immediate field instead of a second source register. For an OP-IMM instruction, the signal flow is similar to the OP instruction, the only difference being the immediate field of the instruction being forwarded to a sign/zero-extend component, the appropriate operation being performed based on whether the instruction is signed, the result of the extend being forwarded to one of the ALU input MUXes, and finally the immediate signal being selected on that MUX instead of the register file signal.
\subsection{JALR}
    A JALR instruction is an absolute jump-and-link instruction, adding the contents of the rs1 register to the contents of the (sign-extended) immediate field, and setting the program counter to the resulting value. To that end, we extend the model with a JALR-specific adder. We set the two inputs of the adder to the immediate forward of the decoder, and the rs1 output of the register file. Lastly, to follow the RISC-V ISA, the least significant bit of the resulting address must be set to 0 to ensure alignment. 
    To model the control transfer of the JALR instruction, we extend the simple instruction fetch stage described before\ref{ref:inst_fetch}, with a multiplexer (the PC multiplexer). The inputs of the multiplexer are set to the PC+4 output of the program counter adder, and the resulting address from the JALR adder.
    We also extend the model with a simple branch logic unit(BLU), which controls the PC multiplexer.
    Finally, the decoder forwards the rs1 and rd field of the instruction to the register file, the immediate field to the JALR adder, selects a hardwired zero and the current program counter as the ALU inputs, selects the JALR BLU operation, and sets the write enable signal of the register file to 1, signifying the output signal of the write back MUX should be written to the rd register.
\subsection{BRANCH}
    The BRANCH instructions perform a program counter-relative conditional jump. Since the target address is obtained by adding the immediate field to the program counter, we cannot realistically reuse the JALR adder for the BRANCH instruction. Because of this, we extend the model with another adder. The input signals of the adder are set to the sign/zero extended immediate field forward of the decoder, and the output of the PC+4 adder. The output of the branch adder is connected to the program counter mux\ref{ref:pc_mux}.
    We also connect the rs1/rs2 outputs of the register file to the BLU mentioned earlier, allowing it to perform the necessary branch comparison, and control the program counter mux accordingly.
    The decoder forwards the rs1 and rs2 fields of the instruction to the register file, the immediate to the branch adder sign/zero extend, and selects the appropriate branch comparison operation for the BLU.
\subsection{JAL}
    A JAL instruction is a program counter-relative jump-and-link. It allows for an unconditional jump, writing the jump source address to the register provided in the rd field.
    Since the JAL instruction is program counter-relative, we can reuse the branch adder\ref{ref:branch_adder} adding a multiplexer to its' PC offset input, feeding the multiplexer with the JAL and BRANCH immediates, and letting the decoder control it accordingly.
    The decoder forwards the rd field of the instruction to the register file, selects the hardwired zero and program counter signals on the ALU multiplexers (much like the JALR instruction) and selects the ADD ALU operation to yield the jump source address and sets the write enable signal of the register file to 1 to signify the ALU result should be written back to the register file. It also forwards the decoded value of the immediate field to the branch adder mux and selects the JAL operation on the BLU, allowing it to control the program counter MUX accordingly.
\subsection{STORE}
    The STORE instructions are simple register to memory operations. To implement them, we extend the SyncRim model with a simple memory component. Initially the memory component has four input signals: the target address, the data to be written, a write enable signal, and a width signal for the requested store operation. The memory also has an internal state consisting of a binary tree map containing byte-aligned memory addresses as keys, and their contents as values. This makes the memory fetch on a SyncRim level efficient as can be while keeping the data sorted by memory address, which is useful for the memory view in the GUI.
    Functionally, when the write enable signal is high, the data contained in the data input signal is written to the address at the address input signal, respecting the selected write width.
    The decoder selects the appropriate data width and enables write for the data memory, feeds the rs1 and rs2 fields to the register file, the immediate field to the one of the ALU multiplexers, selects the rs1 field and the immediate as ALU operands and selects the ALU ADD operation. The ALU result is fed to the data memory as the address input signal, and finally, the rs2 output signal of the register file is connected to the memory component, providing it with data to be written.
\subsection{LOAD}
    The LOAD instructions are simple memory to register operations. To implement them, we extend the general model with a write back multiplexer and the memory model\ref{ref:memory} with a data output signal. The two inputs of the write back multiplexer are the ALU results and the data output signal of the memory component. On LOAD, the decoder forwards the rd and rs1 fields of the instruction to the register file, the immediate field to the ALU, selects the immediate and rs1 signals on the ALU muxes, selects the data memory data output signal on the write back mux, and finally sets the write enable signal of the register file to 1.
\subsection{LUI}
\subsection{AUIPC}
\subsection{SYSTEM}
    The SYSTEM opcode under the RV32I instruction set encodes CSR reads/writes. Since the base implementation of the instruction set only includes read-only counter registers, we chose to omit these operations (and the control status registers) from our implementation, by implementing them as NOP. In the future, these will be implemented to extend the SyncRim model with an interrupt controller.
\subsection{MISC-MEM}
    The MISC-MEM opcode under the RV32I instruction set is only used for the FENCE instruction, which, by specification, may be implemented as a NOP in simpler implementations. Since the SyncRim model is intended for simulation of a specific, single-hart implementation this simplification can be made with no drawbacks.
    
