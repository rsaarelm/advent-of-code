module Main

toFloor : Char -> Integer
toFloor '(' = 1
toFloor ')' = -1
toFloor _ = 0

basement : Integer -> Integer -> (List Integer) -> Integer
basement i acc (d::ds) =
    if acc + d < 0 then i else basement (i + 1) (acc + d) ds
basement i acc [] = i

main : IO ()
main = do
    input <- getLine
    printLn $ sum $ map toFloor $ unpack input

    printLn $ basement 1 0 $ map toFloor (unpack input)
