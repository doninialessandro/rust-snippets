mod constants;
mod core_data_types;
mod operators;
mod scope;
mod stack_and_heap;

pub fn sub() {
    core_data_types::function();
    operators::function();
    scope::function();
    constants::function();
    stack_and_heap::function()
}
