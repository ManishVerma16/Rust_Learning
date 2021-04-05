// Modules for defining public and private functions

mod module{
    fn noodles(){
        println!("Maggi Noodles");
    }
    pub fn print_msg(){
        noodles();
        println!("Calling public print_msg function defined in module");
    }

    // Nested modules

    pub mod new_module{
        pub fn cola(){
            println!("Sprite is my favourite cola drink");
        }
    }

}

fn main(){
    module::print_msg();
    module::new_module::cola();

}