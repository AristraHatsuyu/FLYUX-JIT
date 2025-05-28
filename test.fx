F>main(){
  L>[2]{
    print("reoutput")
  }

  🔢:[obj] = [1, 2, 3, 4, 5]

  print("🌀 for-each 遍历数组：")
  L>🔢:数值{
    print(数值)
  }

  计数器 := 0
  print("🔁 while 循环直到 < 3：")
  L>(计数器 < 3){
    print(计数器)
    计数器 = 计数器 + 1
  }

}