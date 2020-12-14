(ns aoc.2014
  (:require [aoc.util :refer [re-read]]
            [clojure.string :as str]
            [clojure.test :refer [is]]))

(defn clear-mask [bitmask-str]
  (-> bitmask-str
      (str/replace #"1" "0")  ; All digits become 0
      (str/replace #"X" "1")  ; Non-digits become 1
      (#(read-string (str "2r" %)))))

(defn update-mask [bitmask-str]
  (-> bitmask-str
      (str/replace #"X" "0")  ; Non-digits become 0
      (#(read-string (str "2r" %)))))

(defn apply-mask [bitmask-str value]
  (bit-or (update-mask bitmask-str)
          (bit-and value (clear-mask bitmask-str))))

(defn vm-update-1 [[mem mask] [op & args]]
  (case op
    "mem" [(assoc mem (first args) (apply-mask mask (second args)))
           mask]
    "mask" [mem (first args)]))

(defn run-1 [input]
  (let [[mem mask]
        (reduce vm-update-1
                [{} ""]
                input)]
    (reduce + (vals mem))))

; Clear X bits, leave digits
(defn decode-mask [bitmask-str]
  (-> bitmask-str
      (str/replace #"0" "1")  ; All digits become 1
      (str/replace #"X" "0")  ; Non-digits become 0
      (#(read-string (str "2r" %)))))

(defn nth-mask [bitmask-str n]
  (let
   [x-idxs (keep-indexed #(when (= %2 \X) %1) (reverse bitmask-str))
    bits (loop [acc [] n n]
           (if (= n 0) acc (recur (conj acc (mod n 2)) (quot n 2))))]
    (->> (map #(bit-shift-left %1 %2) bits x-idxs)
         (reduce +))))

(is (= (nth-mask "X00X1" 0) 0))
(is (= (nth-mask "X00X1" 1) 2r10))
(is (= (nth-mask "X00X1" 2) 2r10000))
(is (= (nth-mask "X00X1" 3) 2r10010))

(defn float-seq [bitmask-str]
  (let [n (count (filter #{\X} bitmask-str))]
    (map (partial nth-mask bitmask-str) (range (bit-shift-left 1 n)))))

(defn decode-addresses [bitmask-str value]
  (let [value (bit-or value (update-mask bitmask-str))   ; Mask 1s overwrite
        value (bit-and (decode-mask bitmask-str) value)] ; Clear at mask Xs
    (mapv (partial bit-or value) (float-seq bitmask-str))))

(defn vm-update-2 [[mem mask] [op & args]]
  (case op
    "mem"
    (let [value (second args)
          mem (->> (decode-addresses mask (first args))
                   (reduce #(assoc %1 %2 value) mem))]
      [mem mask])
    "mask" [mem (first args)]))

(defn run-2 [input]
  (let [[mem mask]
        (reduce vm-update-2
                [{} ""]
                input)]
    (reduce + (vals mem))))

(defn parse [input]
  (->> (str/split-lines input)
       (mapv (partial re-read
                      [#"(mask) = (.+)"
                       #"(mem)\[(\d+)\] = (\d+)"]))))

(defn -main []
  (let [input (->> (slurp *in*) (str/trim) (parse))]
    (println (run-1 input))
    (println (run-2 input))))
