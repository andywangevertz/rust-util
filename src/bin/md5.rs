use md5::compute;
pub fn md5digest(str: &str) -> String
{
    let md5str = format!("{:x}", md5::compute(str));
    String::from(md5str)
}
fn main() {
   let text = "hello, world";
   let md5str = format!("{:x}", compute(text));
   println!("md5 digest: {}", md5str); 

   println!("md5digest: {}", md5digest("hello, world"));
}
