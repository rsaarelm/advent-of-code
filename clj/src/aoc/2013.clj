(ns aoc.2013
  (:require [aoc.util :refer [re-read]]
            [clojure.string :as str]
            [clojure.test :refer [is]]))

(defn next-bus [time bus]
  (->> (range) (map (partial * bus)) (filter (partial < time)) (first)))

(defn run-1 [[start-time buses]]
  (let [[time bus]
        (->> (filter number? buses)
             (map #(vector (next-bus start-time %) %))
             (sort)
             (first))]
    (* (- time start-time) bus)))

(is (= (run-1 [939 [7 13 59 31 19]]) 295))

(defn mod-inv
  "Multiplicative inverse for a in modulo n,
   solve linear congruence ax = 1 mod n for x."
  [a n]
  (first (filter #(= (mod (* a %) n) 1) (range))))

(defn run-2 [[_ buses]]
  (let [buses (keep-indexed #(when (number? %2) [%2 %1]) buses)
        mods (mapv first buses)
        offsets (mapv #(mod (- (second %)) (first %)) buses)
        ; Solve with Chinese remainder theorem
        n (reduce * mods)
        ys (mapv (partial / n) mods)
        zs (mapv mod-inv ys mods)]
    (->> (map * offsets ys zs)
         (reduce +)
         (#(mod % n)))))

(is (= (run-2 [939 [17 "x" 13 19]])          3417))
(is (= (run-2 [939 [67 7 59 61]])          754018))
(is (= (run-2 [939 [67 "x" 7 59 61]])      779210))
(is (= (run-2 [939 [67 7 "x" 59 61]])     1261476))
(is (= (run-2 [939 [1789 37 47 1889]]) 1202161486))

(defn parse [input]
  (let [[timestamp schedule] (str/split-lines input)]
    [(re-read timestamp) (map re-read (str/split schedule #","))]))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim) (parse))]
    (println (run-1 input))
    (println (run-2 input))))
