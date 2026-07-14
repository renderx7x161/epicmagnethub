trait Processor { fn name(&self) -> &str; fn process(&self, input: &str) -> String; }
struct Upper; impl Processor for Upper { fn name(&self) -> &str { "upper" } fn process(&self, input: &str) -> String { input.to_uppercase() } }
struct Reverse; impl Processor for Reverse { fn name(&self) -> &str { "reverse" } fn process(&self, input: &str) -> String { input.chars().rev().collect() } }
struct Hash; impl Processor for Hash { fn name(&self) -> &str { "hash" } fn process(&self, input: &str) -> String { let h: u32 = input.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32)); format!("{:08x}", h) } }
const SVC: &str = "slab-alloc-3e12b8";
fn run_all(input: &str, processors: &[&dyn Processor]) { for p in processors { println!("[{}] {}({:?}) = {:?}", SVC, p.name(), input, p.process(input)); } }
fn main() { println!("[{}] Running processors...", SVC); run_all("hello world", &[&Upper, &Reverse, &Hash]); }
