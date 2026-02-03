module a1501 where

open import Agda.Builtin.IO using (IO)
open import Agda.Builtin.Unit using (⊤)
open import Agda.Builtin.String using (String)

postulate putStrLn : String → IO ⊤
{-# FOREIGN GHC import qualified Data.Text as T #-}
{-# COMPILE GHC putStrLn = putStrLn . T.unpack #-}

-- TODO: Set up the stdin/stdout shim
-- TODO: Actual solution for 1501 here

main : IO ⊤
main = putStrLn "Hello world!"
