fn main() { println!("test1");dbg!("dbgtest1");}

#[cfg(test)] mod tests {#[test]fn test01() {assert!(true);} #[test]fn test02() {assert_eq!(2,2);}}
