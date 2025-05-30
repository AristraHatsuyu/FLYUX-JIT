F>main(){
  // 定义一个简单对象并测试属性修改
  obj := {a:1, b:2}
  print("原始 obj.a:", obj.a)
  obj.a = 10
  print("修改后 obj.a:", obj.a)

  // 嵌套对象键值修改
  nested := {x:{y:20}}
  print("原始 nested.x.y:", nested.x.y)
  nested.x.y = 30
  print("修改后 nested.x.y:", nested.x.y)

  // 对象中数组元素修改
  complex := {arr:[5,6,7]}
  print("原始 complex.arr[1]:", complex.arr[1])
  complex.arr[1] = 66
  print("修改后 complex.arr[1]:", complex.arr[1])
}