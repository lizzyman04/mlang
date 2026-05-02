# Examples

## Fibonacci

```mlang
int fib(int n) {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

main() {
    for i in 0..10 {
        print(fib(i));
    }
}
```

## Factorial

```mlang
int factorial(int n) {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

main() {
    print(factorial(10));   # 3628800
}
```

## Arrays

```mlang
main() {
    array<int> nums = [10, 20, 30, 40, 50];

    print(nums.len());          # 5
    nums.push(60);
    print(nums.len());          # 6

    for n in nums {
        print(n);
    }

    let first_three = nums.slice(0, 3);
    print(first_three);         # [10, 20, 30]
}
```

## Structs

```mlang
struct Point {
    int x
    int y
}

Point midpoint(Point a, Point b) {
    return Point { x = (a.x + b.x) / 2, y = (a.y + b.y) / 2 };
}

main() {
    let p1 = Point { x = 0, y = 0 };
    let p2 = Point { x = 10, y = 6 };
    let mid = midpoint(p1, p2);
    print(mid.x);   # 5
    print(mid.y);   # 3
}
```

## Interactive I/O

```mlang
main() {
    txt name = read("Enter your name: ");
    int age  = int(read("Enter your age: "));

    print("Hello, " + name + "!");
    print("Next year you will be " + (age + 1));
}
```

## FizzBuzz

```mlang
main() {
    for i in 1..101 {
        bool fizz = i % 3 == 0;
        bool buzz = i % 5 == 0;

        if fizz && buzz {
            print("FizzBuzz");
        } else {
            if fizz {
                print("Fizz");
            } else {
                if buzz {
                    print("Buzz");
                } else {
                    print(i);
                }
            }
        }
    }
}
```

> All example files live in the `examples/` directory of the repository.
