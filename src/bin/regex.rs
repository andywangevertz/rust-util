fn extract_string(input: &str, reg: &str) -> (String, String) {
   let re = regex::Regex::new(reg).unwrap();
   let caps = re.captures(input).unwrap();
   let p1 = caps.get(1).map_or("", |m| m.as_str());
   let p2 = caps.get(2).map_or("", |m| m.as_str()); 
   (String::from(p1), String::from(p2))
}
fn main() {
  let (part1, part2) = extract_string("nonce=\"2dkjdfj3df\"", "\"(.*?)\""); 
  println!("{} {}", part1, part2);
  // with nat=force_rport
  let (part1, part2) = extract_string("172.16.246.101;branch=z9hG4k123456;received=172.16.246.101;rport=52072", "received=(.*?);rport=(.*?)$");
  println!("{} {}", part1, part2);
  // normal case
  let (part1, part2) = extract_string("172.16.246.101;branch=z9hG4k123456", "(.*?);branch=(.*?)$");
  println!("{} {}", part1, part2);
}
