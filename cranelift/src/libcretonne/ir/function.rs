//! Intermediate representation of a function.
//!
//! The `Function` struct defined in this module owns all of its extended basic blocks and
//! instructions.

use ir::{FunctionName, Signature, StackSlot, StackSlotData, JumpTable, JumpTableData,
         DataFlowGraph, Layout};
use entity_map::{EntityMap, PrimaryEntityData};
use std::fmt::{self, Debug, Formatter};

/// A function.
pub struct Function {
    /// Name of this function. Mostly used by `.cton` files.
    pub name: FunctionName,

    /// Signature of this function.
    signature: Signature,

    /// Stack slots allocated in this function.
    pub stack_slots: EntityMap<StackSlot, StackSlotData>,

    /// Jump tables used in this function.
    pub jump_tables: EntityMap<JumpTable, JumpTableData>,

    /// Data flow graph containing the primary definition of all instructions, EBBs and values.
    pub dfg: DataFlowGraph,

    /// Layout of EBBs and instructions in the function body.
    pub layout: Layout,
}

impl PrimaryEntityData for StackSlotData {}
impl PrimaryEntityData for JumpTableData {}

impl Function {
    /// Create a function with the given name and signature.
    pub fn with_name_signature(name: FunctionName, sig: Signature) -> Function {
        Function {
            name: name,
            signature: sig,
            stack_slots: EntityMap::new(),
            jump_tables: EntityMap::new(),
            dfg: DataFlowGraph::new(),
            layout: Layout::new(),
        }
    }

    /// Create a new empty, anomymous function.
    pub fn new() -> Function {
        Self::with_name_signature(FunctionName::new(), Signature::new())
    }

    /// Get the signature of this function.
    pub fn own_signature(&self) -> &Signature {
        &self.signature
    }
}

impl Debug for Function {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use write::function_to_string;
        fmt.write_str(&function_to_string(self))
    }
}
