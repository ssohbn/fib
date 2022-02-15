fn main() {
  for i in 1..20 {
    println!("{}: {}", i, fib(i));
  }
}

fn fib(n: i32) -> i32 {
  assert!(n>0);
  // 0, 1, 1, 2, 3
  let mut j:i32 = 0;
  let mut k:i32 = 1;
  let mut temp:i32;
  for _ in 0..n {
    temp = j;
    j+=k;
    k=temp;
  }
  k
}