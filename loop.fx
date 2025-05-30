F>main(){
  次数 := 3
  print("Times Loop:")
  L>[次数]{
    print("  🔁")
  }

  数组 := [10, 20, 30]
  print("ForEach Loop:")
  L>数组:项{
    print("  →", 项)
  }

  计数 := 2
  print("While Loop:")
  L>(计数 > 0){
    print("  ●", 计数)
    计数--
  }

  索引 := 1
  print("For Loop:")
  L>(索引 := 1; 索引 <= 4; 索引++){
    print("  ◇", 索引)
  }
}