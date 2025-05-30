F>main(){
  // 1. 冒泡排序 Bubble Sort
  arr1 := [5, 3, 8, 1, 4]
  print("原始 arr1:", arr1)
  n1 := arr1.length
  i1 := 0
  L>(i1 < n1 - 1){
    j1 := 0
    L>(j1 < n1 - i1 - 1){
      if (arr1[j1] > arr1[j1+1]) {
        tmp := arr1[j1]
        arr1[j1] = arr1[j1+1]
        arr1[j1+1] = tmp
      }
      j1++
    }
    i1++
  }
  print("冒泡排序后 arr1:", arr1)

  // 2. 选择排序 Selection Sort
  arr2 := [64, 25, 12, 22, 11]
  print("原始 arr2:", arr2)
  n2 := arr2.length
  i2 := 0
  L>(i2 < n2 - 1){
    min_idx := i2
    j2 := i2 + 1
    L>(j2 < n2){
      if (arr2[j2] < arr2[min_idx]) {
        min_idx = j2
      }
      j2++
    }
    // swap
    tmp2 := arr2[i2]
    arr2[i2] = arr2[min_idx]
    arr2[min_idx] = tmp2
    i2++
  }
  print("选择排序后 arr2:", arr2)

  // 3. 插入排序 Insertion Sort（线性查找插入位置）
  arr3 := [4, 2, 8, 6, 1, 3]
  print("原始 arr3:", arr3)
  n3 := arr3.length
  k := 1
  L>(k < n3){
    key := arr3[k]
    // find insertion position via linear search
    pos := 0
    L>(pos < k && arr3[pos] <= key){
      pos++
    }
    // shift elements to make room
    m := k
    L>(m > pos){
      arr3[m] = arr3[m-1]
      m--
    }
    arr3[pos] = key
    k++
  }
  print("插入排序后 arr3:", arr3)

  // 5. 数组修改/读取/添加 操作测试
  test := [1, 2, 3]
  print("原始 test:", test)
  // 读取
  print("test[1] 读取:", test[1])
  // 修改
  test[1] = 22
  print("修改 test[1]:", test[1])
  // 追加
  test[] = 4
  print("追加 test[]:", test)
  // 再次追加
  test[] = 5
  print("再追加 test[]:", test)
  // 长度
  print("test.length:", test.length)
}