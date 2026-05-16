# Modules

MLang supports splitting code across multiple files using `module` and `import`.

## Declaring a module

Use `module <name>` at the top of a file to declare its namespace:

```mlang
module adder

int add(int a, int b) {
    return a + b;
}

int subtract(int a, int b) {
    return a - b;
}
```

## Importing a module

Use `import "<path>"` at the top of your main file:

```mlang
import "adder.mth"

main() {
    print(adder::add(5, 3))       # 8
    print(adder::subtract(10, 4)) # 6
}
```

Paths are relative to the importing file. Circular imports are detected and reported as errors.

## Qualified call syntax

Imported functions are called with `module::function(args)`:

```mlang
import "math.mth"

main() {
    print(math::double(7))   # 14
    print(math::square(4))   # 16
}
```

## Rules

- `module` declaration is required in every module file
- `import` is only valid at the top level of the main file
- Module files cannot have a `main()` function
- Multiple modules can be imported in one file
