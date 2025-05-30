F>main(){
  // 1. 先输入数组长度
  len := I>["请输入元素个数：", number]
  
  // 2. 初始化空数组
  arr := []
  
  // 3. 按索引依次填充
  i := 0
  L>(i < len) {
    prompt := "请输入第" + i + "个值："
    val    := I>[prompt, number]
    arr[i] = val
    i++
  }
  
  // 4. 输出最终数组
  print("输入完的数组：", arr)
}