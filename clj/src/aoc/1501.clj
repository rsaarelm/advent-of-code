(ns aoc.1501)

(defn- part-1 [input] (reduce (partial +) 0 input))

(defn- part-2 [input]
  (let [running-sum (fn [acc elt]
                      (conj acc (+ (peek acc) elt)))]
    (->> input
         (reduce running-sum [0])
         (take-while #(>= % 0))
         (count))))

(defn- parse [input]
  (->> input
       (map {\( 1 \) -1})))

(defn -main [& args]
  (let [input (parse (slurp *in*))]
    (println (part-1 input))
    (println (part-2 input))))
