mod stack;
mod parameter;
mod instruction;
mod method;

pub use self::stack::DataStack;
pub use self::instruction::{ instruction, initialize_time };
pub use self::method::method;

use self::instruction::{ INSTRUCTIONS, InstructionParameter };
use self::parameter::ParameterType;
