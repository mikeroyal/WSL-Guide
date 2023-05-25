// Setting up WSL System Config
pub mod WSL {
   pub fn Hello(name:String) {
      println!("Hello {}",name);
   }
}

fn main(){
   WSL::Hello("WSL users".to_string());
}
