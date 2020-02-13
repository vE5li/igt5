mod parameter;

use super::instruction;
use self::parameter::MethodParameter;
use internal::*;

pub fn method(method: &Data, parameters: Vector<Data>, current_pass: &Option<AsciiString>, root: &Data, build: &Data, context: &Data) -> Status<Option<Data>> {
    let method_body = unpack_list!(method);
    let mut method_stack = DataStack::new(&method_body);
    let mut scope = map!();
    let mut last = None;

    let mut expected_parameters = Vec::new();
    while let Some(next) = method_stack.peek(0) {
        if next.is_list() {
            method_stack.advance(1);
            expected_parameters.push(confirm!(MethodParameter::new(&next)));
        } else {
            break;
        }
    }

    confirm!(MethodParameter::validate(&mut scope, &parameters, &expected_parameters));
    while let Some(instruction_name) = method_stack.pop() {
        let internal_method = unpack_keyword!(&instruction_name);
        if confirm!(instruction(&internal_method, None, &mut method_stack, &mut last, current_pass, root, &scope, build, context), Tag, instruction_name.clone()) {
            return success!(last);
        }
    }

    ensure!(method_stack.closed(), UnclosedScope);
    return success!(None);
}
