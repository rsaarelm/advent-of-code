(ns aoc.2020
  (:require [aoc.util :refer [str->grid assert-eq maptor]]
            [aoc.vec :as v]
            [clojure.set :as set]
            [clojure.string :as str]
            [clojure.test :refer [is]]
            [instaparse.core :as insta]))

(defn parse-tile [s]
  (->> (str->grid s)
       (keep (fn [[p c]] (when (= c \#) p)))
       (set)))

(def parser
  (insta/parser
   "root = tile (<'\n\n'> tile)*
    tile = header block
    header = <'Tile '> #'[0-9]+' <':\n'>
    block = #'[#\\.\\n]+[#\\.]'"))

(defn parse [input]
  (->> (parser input)
       (insta/transform
        {:root maptor
         :tile vector
         :header read-string
         :block parse-tile})))

(defn edge [i->pt]
  (fn [tile]
    (reduce
     bit-or
     (for [i (range 10) :when (tile (i->pt i))]
       (bit-shift-left 1 i)))))

; Edge checksums for tile.
; Top-bottom and left-right must be equal for tiles to match.
(def top (edge #(vector % 0)))
(def bottom (edge #(vector % 9)))
(def left (edge #(vector 0 %)))
(def right (edge #(vector 9 %)))

(defn neighbors [[x y]] #{[(dec x) y] [(inc x) y] [x (dec y)] [x (inc y)]})

(defn variants [tile]
  (let
   [dim (apply max (reduce (partial map max) tile))
    mirror (fn [tile] (set (map (fn [[x y]] [(- dim x) y]) tile)))
    rotate (fn [tile] (set (map (fn [[x y]] [(- dim y) x]) tile)))]
    (mapv #((apply comp %) tile)
          [[]
           [rotate]
           [rotate rotate]
           [rotate rotate rotate]
           [mirror]
           [mirror rotate]
           [mirror rotate rotate]
           [mirror rotate rotate rotate]])))

(defn fits? [jigsaw [x y] tile]
  (and
   (if-let [other (jigsaw [(dec x) y])] (= (right other) (left tile)) true)
   (if-let [other (jigsaw [(inc x) y])] (= (left other) (right tile)) true)
   (if-let [other (jigsaw [x (dec y)])] (= (bottom other) (top tile)) true)
   (if-let [other (jigsaw [x (inc y)])] (= (top other) (bottom tile)) true)))

(defn fit
  "Try to orient a tile to fit the jigsaw.

  Return oriented tile if successful, nil otherwise."
  [jigsaw pos tile]
  (first (filter (partial fits? jigsaw pos) (variants tile))))

(defn solve [tiles]
  (loop [jigsaw {[0 0] (first tiles)}
         [pos & posns] (neighbors [0 0])
         tiles (set (rest tiles))]
    (if (not pos) (do
                    ; No more candidate posns, all pieces should be placed
                    (is (= 0 (count tiles)))
                    jigsaw)
        (let
         [[tile tile' :as hit]
          (->> (map #(when-let [t (fit jigsaw pos %)] [% t]) tiles)
               (filter identity)
               (first))
         ; XXX: Why do I have to cast the second arg to set here?
          expansion (set/difference (neighbors pos) (set (keys jigsaw)))]
          (if hit
          ; Move tile from candidates (unrotated) to jigsaw (rotated),
          ; expand open set with non-jigsaw neighboring tiles.
            (recur
             (assoc jigsaw pos tile')
             (set/union posns expansion)
             (set/difference tiles #{tile}))
          ; If no hit, just discard the candidate pos.
            (recur jigsaw posns tiles))))))

; NB: "Pieces" are labeled, [id tile]. "Tiles" are just the raw geometry set.

(defn recognizer
  "Turn a set of indexed pieces into map from any tile variant to index."
  [pieces]
  (->> pieces
       (mapcat (fn [[id tile]] (map #(vector % id) (variants tile))))
       (into {})))

(defn run-1 [input]
  (let [pieces (parse input)
        lookup (recognizer pieces)
        solution (->>
                  (solve (vals pieces))
                  (map (fn [[pos tile]] [pos (lookup tile)]))
                  (into {}))
        [x1 y1] (reduce (partial mapv min) (keys solution))
        [x2 y2] (reduce (partial mapv max) (keys solution))]
    (* (solution [x1 y1])
       (solution [x2 y1])
       (solution [x1 y2])
       (solution [x2 y2]))))

(defn picture [jigsaw]
  (->> jigsaw
       (mapcat
        (fn [[[u v] pts]]
          (->> (filter (fn [[x y]] (and (<= 1 x 8) (<= 1 y 8))) pts)
               (map (fn [[x y]] [(+ (* u 8) (dec x)) (+ (* v 8) (dec y))])))))
       (set)))

(defn run-2 [input]
  (let [pieces (parse input)
        solution (solve (vals pieces))
        picture (picture solution)
        pics (variants picture)
        monster (->> (str->grid "
                  #
#    ##    ##    ###
 #  #  #  #  #  #   ")
                     (keys)
                     (set))
        scan (fn [pic pos]
               (let [slide (map (partial v/+ pos) monster)]
                 (when (= (set/union pic slide) pic)
                   (set slide))))
        dim (apply max (reduce (partial mapv max) picture))
        monsters
        (mapv (fn [pic]
                (reduce
                 set/union
                 (for [x (range (* (- dim) 2) (* dim 2))
                       y (range (* (- dim) 2) (* dim 2))]
                   (scan pic [x y]))))
              pics)]
    (reduce max (map
                 (fn [slide pic]
                   (if (seq slide) (count (set/difference pic slide)) 0))
                 monsters
                 pics))))

(def test-input
  "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...")

(is (= 20899048083289 (run-1 test-input)))
(is (= 273 (run-2 test-input)))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))
    (println (run-2 input))))

