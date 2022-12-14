
## Simple case call stack

```
[
    mergesort: [2, 1]
    
    [
        merge:
        
        [
            mergesort: [2]
            [
                [2]
            ]
        ],
        [
            mergesort: [1]
            [
                [1]
            ]
        ]

        [
            : [1, 2]
        ]
    ]
]
```

## References
* https://en.wikipedia.org/wiki/Merge_sort
* https://www.geeksforgeeks.org/merge-sort/
* https://mohitkarekar.com/posts/2020/merge-sort-in-rust/