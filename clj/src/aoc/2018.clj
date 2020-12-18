(ns aoc.2018
  (:require [aoc.util :refer [re-read]]
            [clojure.core.match :refer [match]]
            [clojure.string :as str]
            [clojure.test :refer [is]]
            [instaparse.core :as insta]))

(def parse-expr
  (insta/parser
   "EXPR = (EXPR | OPERAND) OP OPERAND
    <OPERAND> = NUM | <'('> EXPR <')'>
    <NUM> = #'[0-9]+'
    <OP> = PLUS | TIMES
    PLUS = <' + '>
    TIMES = <' * '>"))

(defn eval-parsed [expr]
  (match [expr]
    [[:EXPR a [:PLUS] b]] (+ (eval-parsed a) (eval-parsed b))
    [[:EXPR a [:TIMES] b]] (* (eval-parsed a) (eval-parsed b))
    :else (read-string expr)))

(defn eval-expr [expr] (->> expr parse-expr eval-parsed))

(is (= 26 (eval-expr "2 * 3 + (4 * 5)")))
(is (= 13632 (eval-expr "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")))

(defn run-1 [input]
  (->> (str/split-lines input)
       (map eval-expr)
       (reduce +)))

; Part 2: Sums take precedence over products
(def parse-expr-2
  (insta/parser
   "prod = sum | sum (<times> sum)+
    sum = operand | operand (<plus> operand)+
    <operand> = num | <'('> prod <')'>
    num = #'[0-9]+'
    <op> = plus | times
    plus = <' + '>
    times = <' * '>"))

; Super-improved transform evaluator!
(defn eval-expr-2 [expr]
  (->> (parse-expr-2 expr)
       (insta/transform
        {:prod *,
         :sum +,
         :num read-string})))

(is (= (eval-expr-2 "1 + (2 * 3) + (4 * (5 + 6))") 51))
(is (= (eval-expr-2 "2 * 3 + (4 * 5)") 46))
(is (= (eval-expr-2 "5 + (8 * 3 + 9 + 3 * 4 * 3)") 1445))
(is (= (eval-expr-2 "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))") 669060))
(is (= (eval-expr-2 "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2") 23340))

(defn run-2 [input]
  (->> (str/split-lines input)
       (map eval-expr-2)
       (reduce +)))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))
    (println (run-2 input))))
