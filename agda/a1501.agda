{-# OPTIONS --guardedness #-}
module a1501 where

open import Data.Char using (Char)
open import Data.Nat using (ℕ; suc)
open import Data.Nat.Show
open import Data.Integer hiding (suc)
open import Data.Integer.Show
open import Data.List
open import Data.String.Base using (toList)
open import Function using (_$_)
open import IO

floor : List Char -> ℤ
floor [] = + 0
floor ('(' ∷ cs) = (floor cs) + (+ 1)
floor (')' ∷ cs) = (-[1+ 0 ]) + (floor cs)
floor (_ ∷ cs) = (+ 0) + (floor cs)

stops : List Char -> ℕ -> ℕ -> ℕ
stops [] _ last = last
stops _ 0 last = last
stops ('(' ∷ cs) a last = stops cs (suc a) (suc last)
stops (')' ∷ cs) (suc a) last = stops cs a (suc last)
stops (_ ∷ cs) a last = stops cs a last

main : Main
main = run do
    input <- readFiniteFile "aoc-input.txt"
    putStrLn $ Data.Integer.Show.show $ floor $ toList input
    putStrLn $ Data.Nat.Show.show $ stops (toList input) 1 0
