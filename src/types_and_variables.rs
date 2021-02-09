mod constants;
mod core_data_types;
mod operators;
mod scope;

pub fn sub() {
    core_data_types::function();
    operators::function();
    scope::function();
    constants::function();
}
