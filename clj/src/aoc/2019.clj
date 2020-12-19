(ns aoc.2019
  (:require [aoc.util :refer [maptor]]
            [clojure.string :as str]
            [instaparse.core :as insta]))

(def parser
  (insta/parser
   ; yo dawg
   "root = grammar <'\n\n'> inputs

    grammar = rule+
    rule = <' '>* id <': '> clause (<' | '> clause)* [<'\n'>]
    clause = match (<' '> match)*
    <match> = id | terminal
    id = #'[0-9]+'
    terminal = <'\"'>#'[^\"]+'<'\"'>

    inputs = input+
    <input> = #'[a-z]+' [<'\n'>]"))

(defn parse [input]
  (->> (parser input)
       (insta/transform
        ; i herd you like grammars
        {:root vector
         :grammar maptor
         :rule (fn [head & rest] [head (vec rest)])
         :clause vector
         :id read-string
         :terminal #(str "'" % "'")
         :inputs vector})))

(defn bake [grammar]
  (->> grammar
       (map (fn [[rule clauses]]
              (->> (map (partial str/join " ") clauses)
                   (str/join " | ")
                   (#(str rule ": " %)))))
       (str/join "\n")
       ; ...
       (insta/parser)))

(defn run-1 [input]
  (let [[grammar input] (parse input)
        parser (bake grammar)]
    (->> (map parser input)
         (filter vector?)
         (count))))

(assert (= 2 (run-1 "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb")))

(defn run-2 [input]
  (let [[grammar input] (parse input)
        grammar (assoc grammar 8 [[42] [42 8]], 11 [[42 31] [42 11 31]])
        parser (bake grammar)]
    (->> (map parser input)
         (filter vector?)
         (count))))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))
    (println (run-2 input))))
