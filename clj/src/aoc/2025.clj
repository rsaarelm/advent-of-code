(ns aoc.2025
  (:require [aoc.util :refer [re-read]]
            [clojure.string :as str]
            [clojure.test :refer [is]]))

(defn parse [input]
  (->> (str/split-lines input)
       (mapv re-read)))

(defn transform-loop [subject] (iterate #(rem (* % subject) 20201227) 1))

(defn break [key]
  (->>
   (map-indexed vector (transform-loop 7))
   (filter (fn [[_ k]] (= k key)))
   (first)
   (first)))

(defn run-1 [input]
  (let [[card door] (parse input)
        card-loop (break card)]
    (->> (transform-loop door)
         (drop card-loop)
         (first))))

(comment (is (= 14897079 (run-1 "5764801
17807724"))))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))))

