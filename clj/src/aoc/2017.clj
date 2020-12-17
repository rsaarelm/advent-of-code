(ns aoc.2017
  (:require [aoc.util :refer [str->grid]]
            [clojure.set :as set]
            [clojure.string :as str]
            [clojure.test :refer [is]]))

(defn parse [input]
  (->> (str->grid input)
       (keep (fn [[[x y] c]] (when (#{\#} c) [x y])))
       (into #{})))

(defn threedify [[x y z]] [(or x 0) (or y 0) (or z 0)])
(defn fourdify [[x y z w]] [(or x 0) (or y 0) (or z 0) (or w 0)])

(defn expansion-3
  "Expand single cell to a set with all its neighbors."
  [[x y z]]
  (->> (for [x' (range -1 2)
             y' (range -1 2)
             z' (range -1 2)]
         (mapv + [x y z] [x' y' z']))
       (into #{})))

(defn expansion-4
  "Expand single cell to a set with all its neighbors."
  [[x y z w]]
  (->> (for [x' (range -1 2)
             y' (range -1 2)
             z' (range -1 2)
             w' (range -1 2)]
         (mapv + [x y z w] [x' y' z' w']))
       (into #{})))

(defn expand
  "Expand a group of cells to contain neighbors of all cells."
  [ex grid]
  (->> (map ex grid)
       (reduce set/union)))

(defn neighbors
  "List active neighbor cells of a cell."
  [ex grid cell]
  (count (set/intersection grid (set/difference (ex cell) #{cell}))))

(defn new-state
  "Update single cell's state according to game of life rules.

  Live cells stay alive at 2 or 3 neighbors.
  Dead cells come to life at 3 neigbhors.
  All other cells die."
  [ex grid cell]
  (let [n (neighbors ex grid cell)]
    (cond
      (and (grid cell) (<= 2 n 3)) #{cell}
      (and (not (grid cell)) (= n 3)) #{cell}
      :else nil)))

(defn update
  "Construct next state of a grid of cells."
  [ex grid]
  (->> (expand ex grid)
       (map (partial new-state ex grid))
       (reduce set/union)))

(defn run-1 [input]
  (let [grid (into #{} (map threedify (parse input)))]
    (count (last (take 7 (iterate (partial update expansion-3) grid))))))

(is (= 112 (run-1 "
                  .#.
                  ..#
                  ###")))

(defn run-2 [input]
  (let [grid (into #{} (map fourdify (parse input)))]
    (count (last (take 7 (iterate (partial update expansion-4) grid))))))

(is (= 848 (run-2 "
                  .#.
                  ..#
                  ###")))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))
    (println (run-2 input))))
