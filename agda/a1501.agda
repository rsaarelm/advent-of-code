{-# OPTIONS --guardedness #-}
module a1501 where

open import IO

-- TODO: Set up the stdin/stdout shim
-- TODO: Actual solution for 1501 here

main : Main
main = run (putStrLn "Hello, World!")
