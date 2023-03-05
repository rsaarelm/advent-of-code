let input = ($in | split chars | each {|it| ({'(':1, ')':-1} | get $it)})

$input | reduce {|it, acc| $acc + $it }

$input |
    reduce --fold [0, 0] {|it, acc|
        if $acc.1 == -1 { $acc } else { [($acc.0 + 1), ($acc.1 + $it)] }
    } |
    get 0

