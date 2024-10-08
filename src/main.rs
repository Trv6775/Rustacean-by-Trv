#[no_mangle]
extern "C" fn call_from_c(){
    println!("Just called a Rust function from C");
}
fn main(){
    call_from_c();
}