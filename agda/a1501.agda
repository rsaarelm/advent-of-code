{-# OPTIONS --guardedness #-}
module a1501 where

open import IO

-- TODO: Actual solution for 1501 here

main : Main
main = run do
    input <- readFiniteFile "aoc-input.txt"
    putStrLn input
