let input = ($in | lines | parse "{x}x{y}x{z}" |
    each { values } | each { into int })

# p1
$input |
    each {|it|
        let a = $it.0 * $it.1
        let b = $it.1 * $it.2
        let c = $it.2 * $it.0
        2 * $a + 2 * $b + 2 * $c + ([$a $b $c] | math min)
    } |
    math sum

# p2
$input |
    each {|it|
        let a = 2 * $it.0 + 2 * $it.1
        let b = 2 * $it.1 + 2 * $it.2
        let c = 2 * $it.2 + 2 * $it.0
        let v = $it.0 * $it.1 * $it.2
        ([$a $b $c] | math min) + $v
    } |
    math sum
