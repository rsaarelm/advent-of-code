(ns aoc.XXXX
  (:require [aoc.util :refer [re-read]]
            [clojure.string :as str]
            [clojure.test :refer [is]]))

(defn parse [input]
  (->> (str/split-lines input)
       (re-read)))

(defn run-1 [input]
  nil)

(defn run-2 [input]
  nil)

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))
    (println (run-2 input))))
