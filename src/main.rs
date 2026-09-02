fn main() {
 let all: Vec<String> = std::env::args().collect();

 let dll: &String  = &all[1];

 unsafe {
  match libloading::Library::new(dll) {
   Ok(library) => {
    println!("library: {:?}", library);

   }//Ok(library) => {

   Err(error) => {
    println!("error: {:?}", error);

   }//Err(error) => {
  }//match libloading::Library::new("") {
 }//unsafe {

 match std::fs::read(dll) {
  Ok(slice_u8) => {
   match goblin::pe::PE::parse(&slice_u8) {
    Ok(pe) => {
     println!("pe.name: {:?}", pe.name);

     println!("pe.is_64: {:?}", pe.is_64);

     println!("pe.libraries:"); for library in pe.libraries { println!("{:?}", library); }

     println!("pe.exports:"); for export in pe.exports { println!("{:?}", export); }
    }//Ok(pe) => {

    Err(error) => {
     println!("error: {:?}", error);

    }//Err(error) => {
   }//match goblin::pe::PE::parse(&slice_u8) {
  }//Ok(slice_u8) => {

  Err(error) => {
   println!("error: {:?}", error);

  }//Err(error) => {
 };//match std::fs::read(dll) {
}//fn main() {
