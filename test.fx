// 定义方法：自增、自减、加法、乘法
F>increment(a){
    R>a + 1
}
F>decrement(a){
    R>a - 1
}
F>add(a, b, c){
    // 若未传 b、c，可在此设默认
    if (b) { bval := b } else { bval := 0 }
    if (c) { cval := c } else { cval := 0 }
    R>a + b + c
}
F>mul(a, b){
    R>a * b
}

// 定义一个返回数组的函数
F>makeArr(a, b, c){
    R>[a, b, c]
}

F>main(){
    // 1. 直接赋值给变量
    x := 10
    y := x.>increment        // y = 11
    z := x.>add(2)           // z = 10 + 2 + 0 = 12
    w := x.>add(2,3)         // w = 10 + 2 + 3 = 15
    print("y, z, w:", y, z, w)

    // 2. 嵌套链式赋值
    a := 5
    b := a.>increment.>mul(4)  // (5+1)*4 = 24
    print("a chain:", b)

    // 3. 用在条件判断中
    if (x.>decrement > 5) {
        cond :(bool)= true
    } else {
        cond :(bool)= false
    }
    print("cond (9>5):", cond) // true

    // 4. 在循环中使用方法
    cnt := 0
    sum := 0
    L>(cnt < 5){
        // 每次累加 cnt.increment()
        sum = sum + cnt.>increment
        cnt++
    }
    print("sum of 1..5:", sum) // 1+2+3+4+5 = 15

    // 5. 将方法返回值再做变量解包
    arr := makeArr(1,2,3)
    lenArr := arr.>length      // 3
    first := arr[0]
    last  := arr[arr.>length - 1]
    print("arr, len, first, last:", arr, lenArr, first, last)

    // 6. 在函数返回值后续调用
    val := x.>add(1,2).>mul(3)  // (10+1+2)*3 = 39
    print("chained call:", val)

    // 7. 复杂链式混合
    complex := 2.>add(3).>mul(4).>increment  // ((2+3)*4)+1 = 21
    print("complex:", complex)
}