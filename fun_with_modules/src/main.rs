mod a_module;
mod folder_module;

fn main() {
    use a_module::print_something;
    // use folder_module::print_something;
    // use folder_module::internal_folder_module::print_something;
    // use folder_module::internal_folder_module::first_internal_internal_folder_module::print_something;
    // use folder_module::internal_folder_module::second_internal_internal_folder_module::print_something;

    print_something();
}
