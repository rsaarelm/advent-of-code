(ns aoc.2015
  (:require [clojure.test :refer [is]]))

; XXX: Didn't bother writing a parser
(def input [5 2 8 16 18 0 1])

(defn round [[prev seen] [turn input]]
  ; Output input while it lasts, then start outputting delays
  (let [n (if input input (- turn (get seen prev turn)))]
    [n (assoc seen prev turn)]))

(defn run [n input]
  ; Generate sequence of input values followed by nils.
  (->> (map-indexed (fn [i n] [(dec i) n]) (concat input (repeat nil)))
       ; Play game for n turns.
       (#(reduce round [nil {}] (take n %)))
       ; Get prevously said number from reduce state.
       (first)))

(is (= (run 2020 [0 3 6]) 436))
(is (= (run 2020 [1 3 2]) 1))
(is (= (run 2020 [2 1 3]) 10))
(is (= (run 2020 [1 2 3]) 27))
(is (= (run 2020 [2 3 1]) 78))
(is (= (run 2020 [3 2 1]) 438))
(is (= (run 2020 [3 1 2]) 1836))

(defn -main []
  (println (run 2020 input))
  (println (run 30000000 input)))
