(ns aoc.2017
  (:require [aoc.util :refer [str->grid]]
            [clojure.set :as set]
            [clojure.math.combinatorics :refer [cartesian-product]]
            [clojure.string :as str]
            [clojure.test :refer [is]]))

(defn parse [dim input]
  (->> (str->grid input)
       (keep (fn [[[x y] c]]
               (when (#{\#} c)
                 (vec (take dim (concat [x y] (repeat 0)))))))
       (into #{})))

(defn expansion [dim p]
  (->> (apply cartesian-product (take dim (repeat (range -1 2))))
       (map (partial mapv + p))
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

(defn update-grid
  "Construct next state of a grid of cells."
  [ex grid]
  (->> (expand ex grid)
       (map (partial new-state ex grid))
       (reduce set/union)))

(def d3 (partial expansion 3))

(comment (neighbors d3 (parse "
                           .#.
                           ..#
                           ###") [1 0]))

(defn run-1 [input]
  (let [grid (parse 3 input)]
    (count (last (take 7 (iterate (partial update-grid d3) grid))))))

(is (= 112 (run-1 "
                  .#.
                  ..#
                  ###")))

(def d4 (partial expansion 4))

(defn run-2 [input]
  (let [grid (parse 4 input)]
    (count (last (take 7 (iterate (partial update-grid d4) grid))))))

(is (= 848 (run-2 "
                  .#.
                  ..#
                  ###")))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))
    (println (run-2 input))))
