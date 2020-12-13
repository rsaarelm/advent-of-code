(ns aoc.2012
  (:require [aoc.util :refer [re-read sl]]
            [aoc.vec :as v]
            [clojure.string :as str]
            [clojure.test :refer [is]]))

(def dirs
  {"E" [1  0],
   "S" [0  1],
   "W" [-1 0],
   "N" [0 -1]})

(def compass (into [] (vals dirs)))

; Clockwise dirs, right turn increses, left turn decreases
(defn dir->vec [dir]
  (compass (mod (/ dir 90) 4)))

(defn sail-1 [{dir :dir, pos :pos, :as state} [op n]]
  (cond
    (dirs op)  (update state :pos v/+ (v/* n (dirs op)))
    (= op "F") (update state :pos v/+ (v/* n (dir->vec dir)))
    (= op "R") (update state :dir + n)
    (= op "L") (update state :dir - n)))

(defn- run-1 [input]
  (->> input
       (reduce sail-1 {:dir 0, :pos [0 0]})
       (:pos)
       (map #(Math/abs %))
       (reduce +)))

(defn- rotated [v dir]
  (v/+ (v/* (v 0) (dir->vec dir)) (v/* (v 1) (dir->vec (+ dir 90)))))

(defn sail-2 [{wp :wp, pos :pos, :as state} [op n]]
  (cond
    (dirs op)  (update state :wp v/+ (v/* n (dirs op)))
    (= op "F") (update state :pos v/+ (v/* n wp))
    (= op "R") (update state :wp rotated n)
    (= op "L") (update state :wp rotated (- n))))

(defn- run-2 [input]
  (->> input
       (reduce sail-2 {:wp [10 -1] :dir 0, :pos [0 0]})
       (:pos)
       (map #(Math/abs %))
       (reduce +)))

(defn parse [input]
  (->> input
       (str/split-lines)
       (mapv (partial re-read #"(.)(\d+)"))))

(defn- r [input] (run-1 (parse (sl input))))
(run!
 (fn [[a b]] (is (= a b)))
 (partition
  2
  [(r "F10
       N3
       F7
       R90
       F11")
   25]))

(defn- r [input] (run-2 (parse (sl input))))
(run!
 (fn [[a b]] (is (= a b)))
 (partition
  2
  [(r "F10
       N3
       F7
       R90
       F11")
   286]))

(defn -main [& args]
  (let [input (->> (slurp *in*) (str/trim) (parse))]
    (println (run-1 input))
    (println (run-2 input))))
