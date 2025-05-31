F>main(){
  // 比较运算符
  print("1 >  0:",  1 > 0)
  print("1 <  0:",  1 < 0)
  print("1 >= 1:",  1 >= 1)
  print("1 <= 1:",  1 <= 1)
  print("1 =  1:",  1 = 1)   // 单等同“==”
  print("1 == 1:", 1 == 1)

  // 逻辑运算符
  print("true && true:",  1 && 1)   // 非零视为 true
  print("true && false:", 1 && 0)
  print("false && true:", 0 && 1)
  print("false && false:",0 && 0)

  print("true || true:",  1 || 1)
  print("true || false:", 1 || 0)
  print("false || true:", 0 || 1)
  print("false || false:",0 || 0)

  // 复合表达式
  print(" (1<2) && (2<3):", (1<2) && (2<3))
  print(" (1<2) && (2>3):", (1<2) && (2>3))
  print(" (1>2) || (2<3):", (1>2) || (2<3))
  print(" (1>2) || (2>3):", (1>2) || (2>3))
}