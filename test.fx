F>main(){
  // 链式比较自动转换为 &&
  c1:(bool)= 1<2<3
  print(c1)
  c2:(bool)= 5>4>2
  print(c2)
  c3:(bool)= 10>5>2<4  // 10>5 && 5>2 && 2<4
  print(c3)

  // 循环中链式比较
  sum:[int]= 0
  print("loop start")
  L>(sum<3<5){
    sum++
    print(sum)
  }
  print("loop end sum=", sum)
}