(ns aoc.XXXX
  (:require [clojure.string :as str]
            [clojure.test :refer [is]]
            [instaparse.core :as insta]))

(def parser
  (insta/parser
   "root = #'.*'"))

(defn parse [input]
  (->> (parser input)
       (insta/transform
        {:root vector})))

(defn run-1 [input]
  nil)

(defn run-2 [input]
  nil)

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))
    (println (run-2 input))))
