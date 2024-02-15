// this is a comment shouldn't be seen by the parser
var1 = true
var2: bool

/*
this is also a comment shouldn't
be seen by the parser
*/

if var1 == false {
  var2 = true
} else {
  var2 = false
}

hi() {
  print("Hello, World!")
}

main(param: num, param2: str) {
  var1: num = 1234
  var2 = var1 + 1234
  var3: bool = true

  print("{} & {}", var1, var2)

  hello(): (num, num) {
    var1 = 1234

    return (var1, 1234)
  }

  (var1, var2) = hello()
  print("{} & {}", var1, var2)

  var = () {
    var: str = "Hello, World!"
    print("From hashlang: " + var)
  }
}

hi(): bool {
  var1 = true
  var2: bool

  if var1 == false {
    var2 = true
  } else {
    var2 = false
  }

  print("Hello, World!")

  return var1
}

