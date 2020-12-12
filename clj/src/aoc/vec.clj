(ns aoc.vec
  (:require [clojure.core :as s])
  (:refer-clojure :exclude [+ - *]))

(defn + [[x1 y1] [x2 y2]] [(s/+ x1 x2) (s/+ y1 y2)])
(defn - [[x1 y1] [x2 y2]] [(s/- x1 x2) (s/- y1 y2)])
(defn * [a [x y]] [(s/* a x) (s/* a y)])
