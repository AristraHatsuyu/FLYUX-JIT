F>main(){
  🚀 := 0
  🔢 := 3

  // Postfix in print
  print("Launch:", 🚀++, "Altitude:", 🚀)

  // Arithmetic combination
  result := 🚀 * 🔢++
  print("Result of 🚀 * 🔢++:", result, "🔢 now:", 🔢)

  // Nested postfix operations
  x := 1
  y := x++ + x++
  print("y = x++ + x++:", y, "final x:", x)

  // Loop with postfix decrement
  counter := 3
  print("Countdown:")
  L>(counter > 0) {
    print(counter--)
  }

  // Emoji in complex condition
  α := 0
  print("Condition (α++ < 2):", α++ < 2, "α now:", α)
}