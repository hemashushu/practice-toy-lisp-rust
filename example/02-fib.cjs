(do
    (defn fib (a)
        (if
            (lte a 1)
            a
            (add
                (fib (sub a 1))
                (fib (sub a 2))
            )
        )
    )
    (fib 10)
)