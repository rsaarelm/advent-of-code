(ns aoc.2022
  (:require [clojure.string :as str]
            [clojure.test :refer [is]]
            [instaparse.core :as insta]))

(def parser
  (insta/parser
   "root = deck <'\n'> deck
    deck = <#'Player .:\n'> (card <'\n' | #'$'>)+
    card = #'[0-9]+'
   "))

(defn parse [input]
  (->> (parser input)
       (insta/transform
        {:root vector
         :deck vector
         :card read-string})))

(defn winner [[deck-1 deck-2]]
  (if (empty? deck-2) 0 1))

(defn score [decks]
  (reduce + (map * (reverse (decks (winner decks))) (drop 1 (range)))))

(defn game-over? [[deck-1 deck-2]] (or (empty? deck-1) (empty? deck-2)))

(defn play [rule seen-decks [deck-1 deck-2 :as decks]]
  (let [[card-1 card-2] (map first decks)
        [deck-1 deck-2 :as decks] (mapv #(into [] (rest %)) decks)
        seen-decks' (conj seen-decks decks)
        winner (if (seen-decks decks)
                 0
                 (rule seen-decks' card-1 deck-1 card-2 deck-2))
        pile (if (= winner 0) [card-1 card-2] [card-2 card-1])
        decks (update decks winner #(into [] (concat % pile)))]
    (if (game-over? decks)
      decks
      (recur rule seen-decks' decks))))

(defn regular [_ card-1 _ card-2 _]
  (if (> card-1 card-2) 0 1))

(defn recursive [seen-decks card-1 deck-1 card-2 deck-2]
  (if (and (<= card-1 (count deck-1)) (<= card-2 (count deck-2)))
    (-> (play
         recursive
         seen-decks
         [(into [] (take card-1 deck-1)) (into [] (take card-2 deck-2))])
        (winner))
    (regular seen-decks card-1 deck-1 card-2 deck-2)))

(defn run-1 [input] (score (play regular #{} (parse input))))

(defn run-2 [input] (score (play recursive #{} (parse input))))

(def test-input "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10")

(is (= 306 (run-1 test-input)))

; Check infinite loop prevention.
(is (= 273 (run-2 "Player 1:
43
19

Player 2:
2
29
14")))

(is (= 291 (run-2 test-input)))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim))]
    (println (run-1 input))
    ; Took 30 minutes to run.
    (println (run-2 input))))

