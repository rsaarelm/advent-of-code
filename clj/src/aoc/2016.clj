(ns aoc.2016
  (:require [aoc.util :refer [re-read]]
            [clojure.set :refer [difference intersection]]
            [clojure.string :as str]
            [clojure.test :refer [is]]))

(defn parse [input]
  (->> (str/split-lines input)
       (reduce
        (fn [[fields tickets] line]
          (cond
            (.contains line ",")
            [fields (conj tickets (mapv read-string (str/split line #",")))]

            ; XXX: Running the regex twice when match,
            ; don't know how to bind directly in a cond condition...
            (re-find #"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)" line)
            (let
             [[name a1 a2 b1 b2] (re-read
                                  #"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)"
                                  line)]
              [(assoc fields name [[a1 a2] [b1 b2]]) tickets])

            :else [fields tickets]))
        [{} []])
       ((fn [[fields [yours & nearby]]] [fields yours (into [] nearby)]))))

(defn match-field [fields value]
  (loop [acc #{} fields fields]
    (if-let [[[name [[a1 a2] [b1 b2]]] & rest] (seq fields)]
      (if (or (<= a1 value a2) (<= b1 value b2))
        (recur (conj acc name) rest)
        (recur acc rest))
      acc)))

(is (= #{} (match-field {} 1)))
(is (= #{"foo"} (match-field {"foo" [[0 1] [3 4]]} 1)))
(is (= #{"foo"} (match-field {"bar" [[10 20] [22 30]], "foo" [[0 1] [3 4]]} 1)))
(is (= #{} (match-field {"foo" [[0 1] [3 4]]} 9)))

(defn run-1 [input]
  (let
   [[fields _ tickets] (parse input)]
    (reduce + (filter #(empty? (match-field fields %)) (flatten tickets)))))

(is (= 71 (run-1 "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12")))

(defn valid-ticket? [fields ticket]
  (every? #(seq (match-field fields %)) ticket))

(defn run-2 [input]
  (let
   [[fields my-ticket tickets] (parse input)
    tickets (filter (partial valid-ticket? fields) tickets)
    candidates (mapv (fn [_] (into #{} (keys fields))) (keys fields))
    ; Remove impossible values for each field.
    candidates (reduce
                (fn [candidates ticket]
                  (mapv (fn [possible val]
                          (intersection possible (match-field fields val)))
                        candidates
                        ticket))
                candidates tickets)
    fields (loop [field-posns {} candidates candidates]
             (if-let
              [[idx unique] (first (keep-indexed
                                    #(when (= (count %2) 1) [%1 (first %2)])
                                    candidates))]
               (recur
                (assoc field-posns idx unique)
                (mapv #(difference % #{unique}) candidates))
               field-posns))]
    (->> (map-indexed
          (fn [idx n] (if (.contains (fields idx) "departure") n 1))
          my-ticket)
         (reduce *))))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))
    (println (run-2 input))))
