(ns aoc.2024
  (:require [aoc.vec :as v]
            [clojure.set :as set]
            [clojure.string :as str]
            [clojure.test :refer [is]]
            [instaparse.core :as insta]))

(def parser
  (insta/parser
   "root = path (<'\n'> path)*
    path = dir+
    dir = 'e' | 'w' | 'se' | 'sw' | 'ne' | 'nw'"))

; Pointy-top hex grid
; x-axis is W-E
; y-axis is NE-SW
; x=y diagonal is NW-SE

(def dirs {"e" [1 0]
           "se" [1 1]
           "sw" [0 1]
           "w" [-1 0]
           "nw" [-1 -1]
           "ne" [0 -1]})

(defn parse [input]
  (->> (parser input)
       (insta/transform
        {:root vector
         :path vector
         :dir dirs})))

(defn build-floor [paths]
  (->> paths
       (map (partial reduce v/+))
       ; Flip when tile is hit odd times.
       (reduce (fn [occurs x] (if (occurs x) (disj occurs x) (conj occurs x)))
               #{})))

(defn neighbors [pt] (->> (vals dirs) (map (partial v/+ pt)) (set)))

(defn black-neighbors [grid pt] (->> (neighbors pt) (filter grid) (count)))

(defn expand [grid] (set/union grid (set (mapcat neighbors grid))))

(defn becomes-black? [grid pt]
  (let [neighbors (black-neighbors grid pt)]
    (if (grid pt)
      (when (#{1 2} neighbors) true)   ; Black tile staying black
      (when (#{2} neighbors) true))))  ; White tile becomes black

(defn update-floor [grid]
  (->> (expand grid)
       (filter (partial becomes-black? grid))
       (set)))

(defn run-1 [input]
  (->> (parse input)
       (build-floor)
       (count)))

(defn run-2 [input]
  (->> (parse input)
       (build-floor)
       (iterate update-floor)
       (take 101)
       (last)
       (count)))

(def test-input
  "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew")

(is (= 10 (run-1 test-input)))
(is (= 2208 (run-2 test-input)))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))
    (println (run-2 input))))
