(ns aoc.2021
  (:require [clojure.set :as set]
            [clojure.string :as str]
            [clojure.test :refer [is]]
            [instaparse.core :as insta]))

(def parser
  (insta/parser
   "<root> = food+
    food = ingredients allergens <'\n' | #'$'>
    ingredients = (#'[a-z]+' <' '>)+
    allergens = <'(contains '> #'[a-z]+' (<', '> #'[a-z]+')* <')'>"))

(defn parse [input]
  (->> (parser input)
       (insta/transform
        {:root vector
         :food vector
         :ingredients hash-set
         :allergens hash-set})))

(defn possible-allergens [input]
  (->> (mapcat (fn [[igs ags]] (map #(-> [% igs]) ags)) input)
       (reduce (fn [ags [ag igs]]
                 (update ags ag
                         #(if % (set/intersection % igs) igs)))
               {})))

(defn prune [agmap]
  (loop [agmap agmap, result {}]
    (let [[ag igs] (first (filter (fn [[_ ig]] (= (count ig) 1)) agmap))
          known-ig (first igs)]
      (if (not ag)
        result
        (recur
         (->> (dissoc agmap ag)
              (map (fn [[ag igs]] [ag (disj igs known-ig)]))
              (into {}))
         (assoc result ag known-ig))))))

(defn run-1 [input]
  (let [input (parse input)
        allergens (set (vals (prune (possible-allergens input))))]
    (->> (mapcat first input)
         (remove allergens)
         (count))))

(defn run-2 [input]
  (let [input (parse input)
        allergens (sort (prune (possible-allergens input)))]
    (str/join "," (map second allergens))))

(comment (is (= 5 (run-1 "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"))))

(comment (is (= "mxmxvkd,sqjhc,fvjkl"
                (run-2 "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"))))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))
    (println (run-2 input))))
