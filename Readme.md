# Guessing Game

- ```make rec``` 
  
    Using Recursion within Result.map:

# How to do Error-Handling within a Loop in Rust

```
// Cannot use Closures for recursion.
// Use inline fn instead:

fn recursion(args) {
    do_sthg.map(|result| {
        ...
        recursion(args);
    }).map_err(|error| {
        ...
        recursion(args);
    });
}
```


- ```make loop``` 
  
    Using Loop and pattern-matching the Result.map for an Error:
  
```
    let mut proceed = true;
    while proceed {
        let result = do_sthg().map(|result| { ... });
        
        // Use Pattern Matching here for Error Handling:
        let Err(error) = result {
            ...
        }
    }
```