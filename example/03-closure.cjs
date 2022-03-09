(do
    (defn inc_x
        (x)
        (fn
            (i)
            (add x i)
        )
    )
    (let inc_two (inc_x 2))
    (inc_two 10)
)