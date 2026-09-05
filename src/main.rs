fn main() {
 let all: Vec<String> = std::env::args().collect();

 let one: &String  = &all[1];

 match std::fs::read(one) {
  Ok(slice_u8) => {
   match goblin::pe::PE::parse(&slice_u8) {
    Ok(pe) => {
     println!("pe.name: {:?}", pe.name);

     println!("pe.is_64: {:?}", pe.is_64);

     println!("pe.libraries:"); for library in pe.libraries { println!("{:?}", library); }

     println!("pe.exports:"); for export in pe.exports { println!("{:?}", export); }

     unsafe {
      match libloading::Library::new(one) {
       Ok(library) => {
        println!("library: {:?}", library);

        match library.get(b"local_ip_address\0") {
         Ok(local_ip_address) => {
          let f_local_ip_address: libloading::Symbol<extern "C" fn() -> *const std::os::raw::c_char> = local_ip_address;

          let c_local_ip_address: *const std::os::raw::c_char = f_local_ip_address();

          if c_local_ip_address.is_null() {
           println!("local ip address is null"); 

          } else {//if c_local_ip_address.is_null() {
           println!("local ip address: {:?}", std::ffi::CStr::from_ptr(c_local_ip_address).to_string_lossy().into_owned()); 

          }//} else {//if c_local_ip_address.is_null() {
         }//Ok(local_ip_address) {

         Err(error) => {
          println!("error: {:?}", error);

         }//Err(error) => {
        }//match library.get(b"local_ip_address\0") {
       }//Ok(library) => {

       Err(error) => {
        println!("error: {:?}", error);

       }//Err(error) => {
      }//match libloading::Library::new(one) {
     }//unsafe {
    }//Ok(pe) => {

    Err(error) => {
     println!("error: {:?}", error);

    }//Err(error) => {
   }//match goblin::pe::PE::parse(&slice_u8) {
  }//Ok(slice_u8) => {

  Err(error) => {
   println!("error: {:?}", error);

  }//Err(error) => {
 };//match std::fs::read(one) {
}//fn main() {

