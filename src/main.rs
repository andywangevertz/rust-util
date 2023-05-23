use std::process::Command;

fn exec(cmd: &str, args: &[&str]) -> String
{
   let output = Command::new(cmd)
   .args(args)
   .output() 
   .expect("failed to execute cmd");

   String::from_utf8(output.stdout).unwrap()
}

fn userinfo(auth: &str) -> (&str, &str) 
{
  let parts = auth.splitn(2, "@").next().unwrap();
  println!("{:#?}", parts); 
  if parts == auth { return ("", ""); }
  let userpass = parts.rsplitn(2, "//").next().unwrap();
  let mut userpass = userpass.splitn(2, ":");
  (userpass.next().unwrap(), userpass.next().unwrap()) 
}

fn main() {
    println!("Hello, world!");
    let s = "sip://3000:password@127.0.0.1:5060/register";
    //let s = "sip://127.0.0.1:5060/register";
    let (user, pass) = userinfo(&s);
    println!(" get: {} {} ",  user, pass);
    let ret = exec("sh", &["-c", "ip addr |grep global"]);
    //let ret = exec("sh", &["-c", "ls -l |grep target"]);
    println!("exec: <{}>", ret);
    let mut parts = ret.split_whitespace();
    let inet = parts.next().unwrap();
    let ipcidr = parts.next().unwrap();
    println!("{} {}", inet, ipcidr);
    let (ip, cidr) = ipcidr.split_once('/').unwrap(); 
    println!("{} {}", ip,cidr);
    
    // https://internals.rust-lang.org/t/splitn-but-returning-an-array/15712/9
    let mut split = "hey there world".split(' ');
    let words = [(); 3].map(|()| split.next().unwrap());
    println!("{:#?}", words);
}
