use is::compiler;

#[test]
fn test_i32_handling() {
  let src = "let a: i32 = 0;";
  let exp = "int a = 0;";
  let res = compiler(src);
  assert_eq!(res, exp)
}
