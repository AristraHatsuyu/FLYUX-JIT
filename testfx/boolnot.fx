F>main(){
  // 布尔文字
  print("!true =", !true)
  print("!false =", !false)

  // 数值：0 视为假，非 0 视为真
  print("!0 =", !0)
  print("!1 =", !1)
  print("!-5 =", !-5)

  // 字符串："" 或 "false" 视为假，其它非空视为真
  print("!\"\" =", !"")
  print("!\"false\" =", !"false")
  print("!\"hello\" =", !"hello")

  // 变量混合测试
  x := 0
  y := "foo"
  z := false
  print("!x =", !x)
  print("!y =", !y)
  print("!z =", !z)

  // 复合表达式
  print("!(1<2) =", !(1<2))
  print("!(0||true) =", !(0||true))
}