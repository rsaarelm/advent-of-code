(ns aoc.2023
  (:require [aoc.util :as util]
            [clojure.string :as str]
            [clojure.test :refer [is]]))

(defn parse [input]
  (mapv (comp read-string str) (filter #(Character/isDigit %) input)))

(defn cups->nexts
  "Turn cup list into vec[label] -> next cup's label."
  [cups]
  (->> cups
       (#(map vector % (drop 1 (cycle %))))
       (into (sorted-map))
       (#(assoc % 0 0)) ; Cup labels start from 1, add shim to line up.
       (vals)
       (into [])))

(defn nexts-seq [cup nexts]
  (let [next (nexts cup)]
    (lazy-seq (cons next (nexts-seq next nexts)))))

(defn move [[current nexts]]
  (let [[a b c current'] (take 4 (nexts-seq current nexts))
        wrapping-dec (fn [c] (if (= (dec c) 0) (dec (count nexts)) (dec c)))
        target (->> (iterate wrapping-dec current)
                    (filter (complement #{current a b c}))
                    (first))
        d (nexts target)
        nexts' (assoc nexts
                      current current',
                      target a,
                      c d)]
    [current' nexts']))

(defn play
  "Returns nexts vector."
  [rounds cups]
  (->> [(first cups) (cups->nexts cups)]
       (iterate move)
       (take (inc rounds))
       (last)
       (second)))

(defn run-1 [input]
  (->> (play 100 (parse input))
       (nexts-seq 1)
       (take 8)

       (str/join)
       (read-string)))

(defn run-2 [input]
  (->> (play 10000000 (util/concat-vec (parse input) (range 10 1000001)))
       (nexts-seq 1)
       (take 2)
       (reduce *)))

(is (= 67384529 (run-1 "389125467")))

(comment (is (= 149245887792 (run-2 "389125467"))))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))
    (println (run-2 input))))
