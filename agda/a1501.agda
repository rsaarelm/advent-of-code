{-# OPTIONS --guardedness #-}
module a1501 where

open import Data.Char using (Char)
open import Data.Integer using (ℤ; _+_; +_; -[1+_])
open import Data.Integer.Show
open import Data.List
open import Data.String.Base using (toList)
open import IO

floor : List Char -> ℤ
floor [] = + 0
floor ('(' ∷ cs) = (floor cs) + (+ 1)
floor (')' ∷ cs) = (-[1+ 0 ]) + (floor cs)
floor (_ ∷ cs) = (+ 0) + (floor cs)

main : Main
main = run do
    input <- readFiniteFile "aoc-input.txt"
    putStrLn (Data.Integer.Show.show (floor (toList input)))
